#![cfg(test)]

use soroban_sdk::testutils::Ledger;
pub(crate) use super::*;
use token::Client as TokenClient;
extern crate std;
use crate::{token};
use token::StellarAssetClient  as StellarAssetClient;
use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, Env, IntoVal, Symbol, vec
};


use crate::{RecoveryWalletContract, RecoveryWalletContractClient };

struct RecoveryWalletTest<'a> {
    e: Env,
    owner_address: Address,
    recovery_addresses: Vec<Address>,
    recovery_threshold: u32,
    recovery_time_seconds: u64,
    new_owner: Address,
    token: TokenClient<'a>,
    contract: RecoveryWalletContractClient<'a>,
}

impl<'a> RecoveryWalletTest<'a> {
    fn setup(init: bool) -> Self {
        let e = Env::default();
        e.mock_all_auths_allowing_non_root_auth();

        e.ledger().with_mut(|li| {
            li.timestamp = 12345;
        });

        let owner_address: Address = Address::random(&e);
        
        // let token_admin = Address::random(&e);
        let new_owner: Address = Address::random(&e);
    
        let recovery_threshold: u32 = 2;
        let recovery_time_seconds: u64 = 86400;

        let token_admin = Address::random(&e);

        let (token, token_admin_client) = create_token_contract(&e, &token_admin);
        token_admin_client.mint(&owner_address, &1000);
        token_admin_client.mint(&new_owner, &1000);

        let recovery_addresses = vec![ &e, Address::random(&e), Address::random(&e), Address::random(&e)];

        let contract = create_recovery_wallet_contract(&e);

        if init {
            contract.init(  
                &owner_address,
                &recovery_addresses,
                &recovery_threshold,
                &recovery_time_seconds
            );
        }

        RecoveryWalletTest { 
            e,
            owner_address, 
            recovery_addresses,
            recovery_time_seconds,
            recovery_threshold,
            new_owner,
            token,
            contract,
        }
    }
}

fn create_token_contract<'a>(e: &Env, admin: &Address) -> (TokenClient<'a>, StellarAssetClient<'a>) {
    let contract_address = e.register_stellar_asset_contract(admin.clone());
    (
        TokenClient::new(e, &contract_address),
        StellarAssetClient::new(e, &contract_address),
    )
}

fn create_recovery_wallet_contract<'a>(e: &Env) -> RecoveryWalletContractClient<'a> {
    RecoveryWalletContractClient::new(e, &e.register_contract(None, RecoveryWalletContract {}))
}

#[test]
fn test_successsful_recovery() {
    let test: RecoveryWalletTest<'_> = RecoveryWalletTest::setup(true);

    test.contract.deposit(
        &test.owner_address,
        &test.token.address,
        &200,
    );

    // If the new owner is a) the existing owner or b) one of the recovery addresses then it should throw 
    assert_eq!(test.contract.try_recover(&test.owner_address), Err(Ok(Error::InvalidNewOwnerAddress)));
    assert_eq!(test.contract.try_recover(&test.recovery_addresses.get(0).unwrap()), Err(Ok(Error::InvalidNewOwnerAddress)));

    assert_eq!(test.contract.try_recover(&test.new_owner), Ok(Ok(())));
}

#[test]
fn test_not_initialised() {
    let test: RecoveryWalletTest<'_> = RecoveryWalletTest::setup(false);

    assert_eq!(test.contract.try_recover(&test.new_owner), Err(Ok(Error::NotInitalised)));
    assert_eq!(test.contract.try_recover(&test.new_owner), Err(Ok(Error::NotInitalised)));
}

#[test]
fn test_recovery_in_progress() {
    let test: RecoveryWalletTest<'_> = RecoveryWalletTest::setup(true);
    
    // Call recover once
    assert_ne!(test.contract.try_recover(&test.new_owner), Err(Ok(Error::InvalidNewOwnerAddress)));

    let new_new_owner: Address = Address::random(&test.e);

    // Call recover again, which should throw since a recovery is already in progress
    assert_eq!(test.contract.try_recover(&new_new_owner), Err(Ok(Error::RecoveryInProgress)));
    std::println!("\nLedger Time: {}", test.contract.get_ledger_time());

    let recovery = test.contract.get_recovery();

    std::println!("\nRecovery Time: {}", recovery.recovery_end_time);
}