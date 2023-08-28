#![cfg(test)]

use soroban_sdk::{Env, testutils::Address, vec, Vec, arbitrary::std};
use crate::{Contract, ContractClient};

#[test]
fn test() {
    let env: Env = Env::default();
    let contract_id = env.register_contract(None, Contract);

    let client: ContractClient<> = ContractClient::new(&env, &contract_id);
    let owner: soroban_sdk::Address = soroban_sdk::Address::random(&env);
    let owner2: soroban_sdk::Address = soroban_sdk::Address::random(&env);
    let recovery_threshold: u32 = 2;

    let recovery_addresses: Vec<soroban_sdk::Address> = vec![&env, soroban_sdk::Address::random(&env), soroban_sdk::Address::random(&env)]; // Example with 2 addresses

    client.init(&owner, &recovery_addresses, &recovery_threshold);

    let contract_owner = client.get_owner();
    std::println!("\nOwner: {:?}. Contract Owner: {:?}", &owner, &contract_owner);

    assert_eq!(owner, contract_owner);

    client.change_owner(&owner2);
    let contract_owner2 = client.get_owner();
    std::println!("\nOwner2: {:?}. Contract Owner: {:?}", &owner2, &contract_owner2);

    assert_eq!(owner2, contract_owner2);

    let expected_recovery_address_count = recovery_addresses.len();
    let contract_recovery_address_count = client.get_recovery_address_cnt();
    assert_eq!(expected_recovery_address_count, contract_recovery_address_count);
    std::println!("\nexpected_recovery_address_count: {:?}. contract_recovery_address_count Owner: {:?}", &expected_recovery_address_count, &contract_recovery_address_count);

}