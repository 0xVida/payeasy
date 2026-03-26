#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

/// Storage key definitions for persistent contract state.
///
/// Each variant maps to a unique slot in the Soroban persistent storage trie.
/// Using a `#[contracttype]` enum guarantees type-safe, collision-free keys.
///
/// - `DataKey::Landlord` - stores the landlord's `Address`
/// - `DataKey::Amount`   - stores the escrowed amount as `i128`
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Landlord,
    Amount,
}

#[contract]
pub struct RentEscrowContract;

#[contractimpl]
impl RentEscrowContract {
    /// Placeholder - storage write logic added in the next commit.
    pub fn initialize(_env: Env, _landlord: Address, _amount: i128) {}

    /// Placeholder - retrieval logic added in the next commit.
    pub fn get_landlord(_env: Env) -> Address {
        panic!("not implemented")
    }

    /// Placeholder - retrieval logic added in the next commit.
    pub fn get_amount(_env: Env) -> i128 {
        0
    }
}
