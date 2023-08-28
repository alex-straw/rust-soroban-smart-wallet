#![no_std]
mod test;

use soroban_sdk::{contract, contractimpl, contracttype, contracterror, Env, Address, Vec };

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Owner,
    RecoveryAddress(Address),
    RecoveryAddressCnt,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum WalletError {
    InvalidRecoveryAddress = 1,
    InvalidRecoveryThreshold = 2,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {

    pub fn init(
        e: Env,
        owner: Address,
        recovery_addresses: Vec<Address>,
        recovery_threshold: u32,
    ) -> Result<(), WalletError> {
        
        e.storage().instance().set(&DataKey::Owner, &owner);
        
        let mut recovery_address_cnt: u32 = 0;

        for recovery_address in recovery_addresses.iter() {
            if recovery_address == owner || e.storage().instance().has(&DataKey::RecoveryAddress(recovery_address.clone())) {
                return Err(WalletError::InvalidRecoveryAddress)
            } 
            else {
                e.storage().instance().set(&DataKey::RecoveryAddress(recovery_address), &());
                recovery_address_cnt +=1;
            }
        }

        e.storage().instance().set(&DataKey::RecoveryAddressCnt, &recovery_address_cnt);
        
        if recovery_threshold > recovery_address_cnt {
            return Err(WalletError::InvalidRecoveryThreshold)
        }

        Ok(())
    }

    pub fn get_owner(e: Env) -> Address
    {
        return e.storage().instance().get(&DataKey::Owner).unwrap();
    }

    pub fn get_recovery_address_cnt(e: Env) -> u32
    {
        return e.storage().instance().get(&DataKey::RecoveryAddressCnt).unwrap_or(0);
    }

    pub fn change_owner(e: Env, _new_owner: Address)
    {
        e.storage().instance().set(&DataKey::Owner, &_new_owner);
    }
}