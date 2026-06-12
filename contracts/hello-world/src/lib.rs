#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String};

#[contract]
pub struct Token;

#[contractimpl]
impl Token {
    pub fn init(env: Env, admin: String) {
        env.storage().instance().set(&"admin", &admin);
    }

    pub fn mint(env: Env, to: String, amount: u64) {
        let bal: u64 = env.storage().instance().get::<_, u64>(&to).unwrap_or(0);
        env.storage().instance().set(&to, &(bal + amount));
    }

    pub fn transfer(env: Env, from: String, to: String, amount: u64) {
        let from_bal: u64 = env.storage().instance().get::<_, u64>(&from).unwrap_or(0);
        if from_bal < amount {
            panic!("insufficient balance");
        }
        let to_bal: u64 = env.storage().instance().get::<_, u64>(&to).unwrap_or(0);
        env.storage().instance().set(&from, &(from_bal - amount));
        env.storage().instance().set(&to, &(to_bal + amount));
    }

    pub fn balance_of(env: Env, account: String) -> u64 {
        env.storage().instance().get::<_, u64>(&account).unwrap_or(0)
    }

    pub fn faucet(env: Env, to: String, amount: u64) {
        let bal: u64 = env.storage().instance().get::<_, u64>(&to).unwrap_or(0);
        env.storage().instance().set(&to, &(bal + amount));
    }
}