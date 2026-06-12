#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    token::Client as TokenClient,
    Address, Env,
};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Buyer,
    Seller,
    Token,
    Amount,
    SellerConfirmed,
    BuyerConfirmed,
    Completed,
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    pub fn initialize(
        env: Env,
        buyer: Address,
        seller: Address,
        token: Address,
        amount: i128,
    ) {
        if env.storage().instance().has(&DataKey::Buyer) {
            panic!("already initialized");
        }

        buyer.require_auth();

        env.storage().instance().set(&DataKey::Buyer, &buyer);
        env.storage().instance().set(&DataKey::Seller, &seller);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::Amount, &amount);
        env.storage().instance().set(&DataKey::SellerConfirmed, &false);
        env.storage().instance().set(&DataKey::BuyerConfirmed, &false);
        env.storage().instance().set(&DataKey::Completed, &false);

        let token_client = TokenClient::new(&env, &token);

        token_client.transfer(
            &buyer,
            &env.current_contract_address(),
            &amount,
        );
    }

    pub fn seller_confirm(env: Env) {
        let seller: Address = env
            .storage()
            .instance()
            .get(&DataKey::Seller)
            .unwrap();

        seller.require_auth();

        env.storage()
            .instance()
            .set(&DataKey::SellerConfirmed, &true);
    }

    pub fn buyer_confirm(env: Env) {
        let buyer: Address = env
            .storage()
            .instance()
            .get(&DataKey::Buyer)
            .unwrap();

        buyer.require_auth();

        env.storage()
            .instance()
            .set(&DataKey::BuyerConfirmed, &true);
    }

    pub fn release(env: Env) {
        let seller_confirmed: bool = env
            .storage()
            .instance()
            .get(&DataKey::SellerConfirmed)
            .unwrap();

        let buyer_confirmed: bool = env
            .storage()
            .instance()
            .get(&DataKey::BuyerConfirmed)
            .unwrap();

        let completed: bool = env
            .storage()
            .instance()
            .get(&DataKey::Completed)
            .unwrap();

        if completed {
            panic!("trade completed");
        }

        if !(seller_confirmed && buyer_confirmed) {
            panic!("waiting confirmations");
        }

        let seller: Address = env
            .storage()
            .instance()
            .get(&DataKey::Seller)
            .unwrap();

        let token: Address = env
            .storage()
            .instance()
            .get(&DataKey::Token)
            .unwrap();

        let amount: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Amount)
            .unwrap();

        let token_client = TokenClient::new(&env, &token);

        token_client.transfer(
            &env.current_contract_address(),
            &seller,
            &amount,
        );

        env.storage()
            .instance()
            .set(&DataKey::Completed, &true);
    }

    pub fn get_status(env: Env) -> (bool, bool, bool) {
        let seller_confirmed: bool = env
            .storage()
            .instance()
            .get(&DataKey::SellerConfirmed)
            .unwrap();

        let buyer_confirmed: bool = env
            .storage()
            .instance()
            .get(&DataKey::BuyerConfirmed)
            .unwrap();

        let completed: bool = env
            .storage()
            .instance()
            .get(&DataKey::Completed)
            .unwrap();

        (
            seller_confirmed,
            buyer_confirmed,
            completed,
        )
    }
}