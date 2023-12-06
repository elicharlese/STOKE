use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Erc20 {
    balances: UnorderedMap<AccountId, Balance>,
    total_supply: Balance,
}

impl Default for Erc20 {
    fn default() -> Self {
        panic!("Contract should be initialized before usage")
    }
}

#[near_bindgen]
impl Erc20 {
    #[init]
    pub fn new(owner_id: AccountId, total_supply: Balance) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let mut balances = UnorderedMap::new(b"b".to_vec());
        balances.insert(&owner_id, &total_supply);
        Self {
            balances,
            total_supply,
        }
    }

    pub fn transfer(&mut self, receiver_id: AccountId, amount: Balance) {
        let sender_id = env::predecessor_account_id();
        let sender_balance = self.balance_of(&sender_id);
        assert!(sender_balance >= amount, "Not enough balance");

        self.balances.insert(&sender_id, &(sender_balance - amount));
        self.balances
            .insert(&receiver_id, &(self.balance_of(&receiver_id) + amount));
    }

    pub fn balance_of(&self, account_id: &AccountId) -> Balance {
        self.balances.get(account_id).unwrap_or(0)
    }
}
