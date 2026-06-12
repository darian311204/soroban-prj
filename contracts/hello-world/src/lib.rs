#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct Poap;

#[contractimpl]
impl Poap {
    /// Initialize the contract with the organizer address
    pub fn init(env: Env, organizer: Address) {
        env.storage().instance().set(&"organizer", &organizer);
    }

    /// Mint attendance proof to a recipient (organizer only)
    pub fn mint(env: Env, recipient: Address, event_id: u64) {
        let organizer: Address = env.storage().instance().get(&"organizer").unwrap();
        let invoker: Address = env.source_account().unwrap();
        if invoker != organizer {
            panic!("not authorized");
        }

        let key = (event_id, recipient.clone());
        if env.storage().instance().get::<_, bool>(&key).unwrap_or(false) {
            panic!("already claimed");
        }

        env.storage().instance().set(&key, &true);
        env.events().publish(("poap", "minted"), (event_id, recipient));
    }

    /// Check if an attendee has claimed their proof for an event
    pub fn is_claimed(env: Env, event_id: u64, attendee: Address) -> bool {
        let key = (event_id, attendee);
        env.storage().instance().get::<_, bool>(&key).unwrap_or(false)
    }

    /// Get the organizer address
    pub fn organizer(env: Env) -> Address {
        env.storage().instance().get(&"organizer").unwrap()
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::Env;
    use crate::{Poap, PoapClient};

    fn create_test_env() -> (Env, PoapClient<'static>) {
        let env = Env::default();
        let client = PoapClient::new(&env, &env.register_contract(None, crate::Poap));
        (env, client)
    }

    #[test]
    fn test_init_and_organizer() {
        let (env, client) = create_test_env();
        let organizer = Address::from_string(&"GDATOHIE23D2T5F6BK7LKCWBQM2DLDMXLCEKJHPWLOWSM6KGMZ3W3XXX".into_val(&env));

        client.init(&organizer);
        assert_eq!(client.organizer(), organizer);
    }

    #[test]
    fn test_mint_and_claim() {
        let (env, client) = create_test_env();
        let organizer = Address::from_string(&"GDATOHIE23D2T5F6BK7LKCWBQM2DLDMXLCEKJHPWLOWSM6KGMZ3W3XXX".into_val(&env));
        let recipient = Address::from_string(&"GAFOHIE23D2T5F6BK7LKCWBQM2DLDMXLCEKJHPWLOWSM6KGMZ3W3YYY".into_val(&env));

        client.init(&organizer);

        assert!(!client.is_claimed(&1, &recipient));

        client.mint(&recipient, &1);

        assert!(client.is_claimed(&1, &recipient));
        assert!(!client.is_claimed(&2, &recipient));
    }

    #[test]
    fn test_cannot_claim_twice() {
        let (env, client) = create_test_env();
        let organizer = Address::from_string(&"GDATOHIE23D2T5F6BK7LKCWBQM2DLDMXLCEKJHPWLOWSM6KGMZ3W3XXX".into_val(&env));
        let recipient = Address::from_string(&"GAFOHIE23D2T5F6BK7LKCWBQM2DLDMXLCEKJHPWLOWSM6KGMZ3W3YYY".into_val(&env));

        client.init(&organizer);
        client.mint(&recipient, &1);

        let result = std::panic::catch_unwind(|| client.mint(&recipient, &1));
        assert!(result.is_err());
    }
}