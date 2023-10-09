#![no_std]
mod test;

use soroban_sdk::{contract, contractimpl, contracttype, contracterror, Env, Address, Vec, BytesN, token };

// -------------- VARIABLES -------------- // 

#[contracttype]
#[derive(Clone)]
enum DataKey {
    ContractInit,
    Balance,
    OwnerAddress,
    RecoveryAddress(Address),
    RecoveryAddressCnt,
    RecoveryThreshold,
    RecoveryTime,
    Recovery,
}

#[derive(Clone)]
#[contracttype]
pub struct Recovery {
    pub new_owner_address: Address,
    pub signature_count: u32,
    pub signatures_list: Vec<Address>,
    pub recovery_end_time: u64,
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum State {
    NotInProgress,
    InProgress,
    CompletedAndReset,
}

// -------------- ERRORS -------------- // 

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotInitalised = 1,
    InvalidRecoveryAddress = 2,
    InvalidRecoveryThreshold = 3,
    RecoveryNotInProgress = 4,
    InvalidNewOwnerAddress = 5,
    AlreadySigned = 6,
    RecoveryInProgress = 7,
    InsufficientFunds = 8,
    SignatureThresholdAlreadyReached = 9,
}

// -------------- Contract -------------- // 

#[contract]
pub struct RecoveryWalletContract;

#[contractimpl]
impl RecoveryWalletContract {

    // -------------- INITIALISATION -------------- // 

    // Sets up the initial state of the recovery wallet

    pub fn init(
        e: Env, 
        owner: Address, 
        recovery_addresses: Vec<Address>,
        recovery_threshold: u32,
        recovery_time_seconds: u64,
    ) -> Result<(), Error> {
        
        e.storage().instance().set(&DataKey::OwnerAddress, &owner);
        
        let mut recovery_address_cnt: u32 = 0;

        for recovery_address in recovery_addresses.iter() {
            if recovery_address == owner || e.storage().instance().has(&DataKey::RecoveryAddress(recovery_address.clone())) {
                return Err(Error::InvalidRecoveryAddress)
            } 
            else {
                e.storage().instance().set(&DataKey::RecoveryAddress(recovery_address), &());
                recovery_address_cnt +=1;
            }
        }

        e.storage().instance().set(&DataKey::RecoveryAddressCnt, &recovery_address_cnt);
        
        if recovery_threshold > recovery_address_cnt || recovery_threshold == 0 {
            return Err(Error::InvalidRecoveryThreshold)
        }

        e.storage().instance().set(&DataKey::RecoveryThreshold, &recovery_threshold);

        e.storage().instance().set(&DataKey::RecoveryTime, &recovery_time_seconds);

        e.storage().instance().set(&DataKey::ContractInit, &true);

        e.storage().instance().set(&DataKey::Recovery, &Recovery {
            new_owner_address: owner,
            signature_count: 0,
            signatures_list: Vec::from_array(&e, []),
            recovery_end_time: 0,
        });

        Ok(())
    }


    // -------------- RECOVERY PROCESS -------------- // 

    pub fn recover(
        e: Env, 
        new_owner: Address
    ) -> Result<(), Error> {

        if !Self::initialised(&e) {
            return Err(Error::NotInitalised);
        }

        if e.storage().instance().get::<DataKey, Address>(&DataKey::OwnerAddress).unwrap() == new_owner {
            return Err(Error::InvalidNewOwnerAddress);
        }

        if e.storage().instance().has(&DataKey::RecoveryAddress(new_owner.clone())) {
            return Err(Error::InvalidNewOwnerAddress);
        }

        match Self::recovery_state(e.clone()) {
            State::NotInProgress => {},
            State::InProgress => { return Err(Error::RecoveryInProgress); },
            State::CompletedAndReset => {}
        }

        let recovery_end_time = e.storage().instance().get::<DataKey, u64>(&DataKey::RecoveryTime).unwrap() + e.ledger().timestamp();

        e.storage().instance().set(&DataKey::Recovery, &Recovery {
            new_owner_address : new_owner,
            signature_count: 0,
            signatures_list: Vec::from_array(&e, []),
            recovery_end_time: recovery_end_time,
        });

        Ok(())
    }

    // -------------- SIGN -------------- // 

    pub fn sign(
        e: Env, 
        signer: Address
    ) -> Result<(), Error> {
        
        if !Self::initialised(&e) {
            return Err(Error::NotInitalised)
        }

        signer.require_auth();

        if !e.storage().instance().has(&DataKey::RecoveryAddress(signer.clone())) {
            return Err(Error::InvalidRecoveryAddress)
        }

        match Self::recovery_state(e.clone()) {
            State::NotInProgress => { return Err(Error::RecoveryNotInProgress) },
            State::InProgress => {},
            State::CompletedAndReset => { return Err(Error::SignatureThresholdAlreadyReached) },
        }

        let mut recovery: Recovery = e.storage().instance().get(&DataKey::Recovery).unwrap();

        if recovery.signatures_list.contains(signer.clone()) {
            return Err(Error::AlreadySigned)
        }

        recovery.signatures_list.push_back(signer);
        recovery.signature_count += 1;

        e.storage().instance().set(&DataKey::Recovery, &recovery);
    
        Ok(())
    }

    // -------------- TRANSACTIONS -------------- // 
    
    pub fn deposit(
        e: Env,
        from: Address,
        token: Address,
        amount: i128
    ) -> Result<(), Error> {

        if !Self::initialised(&e) {
            return Err(Error::NotInitalised)
        }

        token::Client::new(&e, &token).transfer(&from, &e.current_contract_address(), &amount);

        let balance = e.storage().instance().get(&DataKey::Balance).unwrap_or(0);

        e.storage().instance().set(&DataKey::Balance, &(balance + amount));
        
        Ok(())
    }

    pub fn withdraw(
        e: Env,
        token: Address,
        amount: i128
    ) -> Result<(), Error>  {

        if !Self::initialised(&e) {
            return Err(Error::NotInitalised)
        }

        let owner: Address = e.storage().instance().get(&DataKey::OwnerAddress).unwrap();

            owner.require_auth();

        let balance = e.storage().instance().get(&DataKey::Balance).unwrap_or(0);

        if balance < amount {
            return Err(Error::InsufficientFunds);
        }

        token::Client::new(&e, &token).transfer(
            &e.current_contract_address(),
            &owner,
            &&amount,
        );

        e.storage().instance().set(&DataKey::Balance,  &(balance - amount)); 

        Ok(())
    }

    pub fn recovery_state(e: Env) -> State {

        if !Self::initialised(&e) {
            panic!("Not initialised");
        }
    
        let cur_time = e.ledger().timestamp();

        let recovery: Recovery = e.storage().instance().get(&DataKey::Recovery).unwrap();
    
        let sig_count = recovery.signature_count;
        let recovery_threshold : u32 = e.storage().instance().get(&DataKey::RecoveryThreshold).unwrap();

        let recovery_threshold_met = sig_count >= recovery_threshold;        
        // let recovery_threshold_met = recovery.signature_count >= e.storage().instance().get(&DataKey::RecoveryThreshold).unwrap();
        
        if recovery.recovery_end_time == 0 {
            State::NotInProgress
        } else if !recovery_threshold_met && cur_time < recovery.recovery_end_time {
            State::InProgress
        } else {
            if recovery_threshold_met {
                e.storage().instance().set(&DataKey::OwnerAddress, &recovery.new_owner_address)
            }
            e.storage().instance().set(&DataKey::Recovery, &Recovery {
                new_owner_address: Address::from_contract_id(&BytesN::from_array(&e, &[1u8; 32])),
                signature_count: 0,
                signatures_list: Vec::from_array(&e, []),
                recovery_end_time: 0,
            }); 
            State::CompletedAndReset
        }
    }
    
    fn initialised(e: &Env) -> bool {
        return e.storage().instance().get::<DataKey, bool>(&DataKey::ContractInit).unwrap_or(false);
    }
    
    // -------------- UTILITY FUNCTIONS --------------

    pub fn get_owner(e: Env) -> Address{
        e.storage().instance().get(&DataKey::OwnerAddress).unwrap()
    }
    
    pub fn get_balance(e: Env) -> i128 {
        e.storage().instance().get(&DataKey::Balance).unwrap_or(0)
    }

    pub fn get_ledger_time(e: Env) -> u64 {
        e.ledger().timestamp()
    }

    pub fn get_recovery(e: Env) -> Recovery {
        if !Self::initialised(&e) {
            panic!("Contract has not been initalised");
        }

        e.storage().instance().get(&DataKey::Recovery).unwrap()
    }
}