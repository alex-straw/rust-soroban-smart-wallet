Stellar/Soroban Recovery Wallet
A Rust smart contract recovery wallet built using Soroban (for smart contracts on Stellar). Built for the Sorobanathon: Road to Mainnet hackathon (ending 18/09/2023).

Key Components:

RecoveryWalletContract: The primary contract.
init: Initialises the wallet with the initial owner, recovery addresses, and other parameters.
recover: Initiates a recovery process to set a new owner.
sign: Used by recovery addresses to sign off on a recovery process.
deposit and withdraw: Transaction functions to deposit and withdraw funds.
Important Notes:

The contract relies on the soroban_sdk for many of its functionalities.
Recovery processes are completed after a certain threshold of recovery addresses sign off on them.
The contract ensures safety by throwing errors when invalid operations are attempted, such as reusing an address for recovery, signing a recovery process twice, or withdrawing more funds than available.
Events are published for significant operations like initialisation, recovery initiation, and signing.
