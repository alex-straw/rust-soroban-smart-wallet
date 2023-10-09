# Stellar/Soroban Recovery Wallet

A Rust-based smart contract recovery wallet designed for the Stellar network, utilising the Soroban sdk. Developed for the Sorobanathon: Road to Mainnet hackathon (ending on 11/10/2023).

## RecoveryWalletContract

The Stellar/Soroban Recovery Wallet is a smart contract wallet, designed to keep assets secure while providing a safe and systematic recovery process to prevent loss of access. 

### `init`

- **Purpose:** Initialise the wallet.
- **Parameters:**
  - `owner`: Initial owner address.
  - `recovery_addresses`: A list of addresses which can recover the wallet.
  - `recovery_threshold`: Minimum number of signatures needed for recovery.
  - `recovery_time_seconds`: Recovery process time duration.
- **Behavior:** Sets the wallet owner and configures recovery Parameters. Each recovery address must be unique and not equal to the owner address.

### `recover`

- **Purpose:** Initiate the recovery process.
- **Parameters:**
  - `new_owner`: Proposed new owner address.
- **Behavior:** Sets up the recovery process if valid conditions are met, and establishes a recovery end time.

### `sign`

- **Purpose:** Sign a recovery process.
- **Parameters:**
  - `signer`: Address of the signer.
- **Behavior:** Recovery addresses use this function to sign and approve a recovery process.

### Transaction Functionalities:

#### `deposit`

- **Purpose:** Deposit funds into the wallet.
- **Parameters:**
  - `e`: Environment.
  - `from`: Sender address.
  - `token`: Token/Asset address.
  - `amount`: Deposit amount.
- **Behavior:** Transfers tokens to the contract address and updates the balance.

#### `withdraw`

- **Purpose:** Withdraw funds from the wallet.
- **Parameters:**
  - `token`: Token/Asset address.
  - `amount`: Withdrawal amount.
- **Behavior:** Transfers tokens from the contract to the owner and updates the balance.

## Utility Functions

- `get_owner`: Retrieve the current owner’s address.
- `get_balance`: Fetch the current balance.
- `get_ledger_time`: Obtain the current ledger timestamp.
- `get_recovery`: Access the current recovery state.

## Error Handling

The contract utilises a series of bespoke error responses (like `Error::NotInitalised`, `Error::InvalidRecoveryAddress`, etc.) to ensure that users and developers receive clear feedback on any issues or missteps in usage or during the interaction with the contract.

## State Management

The `recovery_state` function assesses the current state of the recovery process. It makes decisions based on the recovery signatures, end time, and other factors, such as determining if the recovery threshold has been met and if the recovery process should proceed or reset.


## Use Cases

- **Recovery Process:** Enables a recovery mechanism for the owner to regain access to the assets in the case of private key loss or other issues.
- **Asset Transactions:** Allow the secure and straightforward deposit and withdrawal of assets.

## Deploy to Futurenet

```soroban contract build```

Follow the steps here `https://soroban.stellar.org/docs/getting-started/deploy-to-testnet`

```
 soroban contract deploy   --wasm target/wasm32-unknown-unknown/release/recovery_wallet.wasm   --source <configured-identity-alias>   --network testnet
```