#![cfg(test)]

use soroban_sdk::testutils::Ledger;
pub(crate) use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{token, vec, Address, Env };
use token::AdminClient as TokenAdminClient;
use token::Client as TokenClient;
extern crate std;

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

    test.contract.deposit(
        &test.owner_address,
        &test.token.address,
        &200,
    );

    assert_eq!(test.contract.try_recover(&test.owner_address), Err(Ok(Error::InvalidNewOwnerAddress)));
    assert_ne!(test.contract.try_recover(&test.new_owner), Err(Ok(Error::InvalidNewOwnerAddress)));
}