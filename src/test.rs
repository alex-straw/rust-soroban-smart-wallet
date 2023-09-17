#![cfg(test)]

use super::*;
use soroban_sdk::testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, Ledger};
use soroban_sdk::{symbol_short, token, vec, Address, Env, IntoVal};
use token::AdminClient as TokenAdminClient;
use token::Client as TokenClient;

use crate::{RecoveryWalletContract, RecoveryWalletContractClient };

struct RecoveryWalletTest<'a> {
    e: Env,
    owner_address: Address,
    recovery_addresses: Vec<Address>,
    recovery_threshold: u32,
    recovery_time_seconds: u32,
    new_owner: Address,
    token: TokenClient<'a>,
    contract: RecoveryWalletContractClient<'a>,
}

impl<'a> RecoveryWalletTest<'a> {
    fn setup() -> Self {
        let e = Env::default();
        e.mock_all_auths();

        e.ledger().with_mut(|li| {
            li.timestamp = 12345;
        });

        let owner_address: Address = Address::random(&e);
        
        // let token_admin = Address::random(&e);
        let new_owner: Address = Address::random(&e);
    
        let recovery_threshold: u32 = 2;
        let recovery_time_seconds: u32 = 86400;

        let token_admin = Address::random(&e);

        let (token, token_admin_client) = create_token_contract(&e, &token_admin);
        token_admin_client.mint(&owner_address, &1000);
        token_admin_client.mint(&new_owner, &1000);

        let recovery_addresses = vec![ &e, Address::random(&e), Address::random(&e), Address::random(&e)];

        let contract = create_recovery_wallet_contract(&e);

        contract.init(
            &owner_address,
            &recovery_addresses,
            &recovery_threshold,
            &recovery_time_seconds
        );

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

fn create_token_contract<'a>(e: &Env, admin: &Address) -> (TokenClient<'a>, TokenAdminClient<'a>) {
    let contract_address = e.register_stellar_asset_contract(admin.clone());
    (
        TokenClient::new(e, &contract_address),
        TokenAdminClient::new(e, &contract_address),
    )
}

fn create_recovery_wallet_contract<'a>(e: &Env) -> RecoveryWalletContractClient<'a> {
    RecoveryWalletContractClient::new(e, &e.register_contract(None, RecoveryWalletContract {}))
}

#[test]
fn test() {
    let test: RecoveryWalletTest<'_> = RecoveryWalletTest::setup();

    // token_admin_client.mint(&owner, &1000);
    // token_admin_client.mint(&random_unpermissioned_address, &1000);

    // client.deposit(&owner, &token.address, &100);
    // client.deposit(&random_unpermissioned_address, &token.address, &100);

    // std::println!("\nOwner: {:?}. Contract Owner: {:?}", &owner, &contract_owner);

    // assert_eq!(owner, contract_owner);

    // client.change_owner(&owner2);
    // let contract_owner2 = client.get_owner();
    // std::println!("\nOwner2: {:?}. Contract Owner: {:?}", &owner2, &contract_owner2);

    // assert_eq!(owner2, contract_owner2);

    // let expected_recovery_address_count = recovery_addresses.len();
    // let contract_recovery_address_count = client.get_recovery_address_cnt();
    // assert_eq!(expected_recovery_address_count, contract_recovery_address_count);
    // std::println!("\nexpected_recovery_address_count: {:?}. contract_recovery_address_count Owner: {:?}", &expected_recovery_address_count, &contract_recovery_address_count);

    // let ledger_timestamp = client.get_ledger_timestamp();
    // std::println!("\ncurrent ledger timestamp: {:?}.", &ledger_timestamp);
    
}