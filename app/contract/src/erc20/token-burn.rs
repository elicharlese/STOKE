use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise, PromiseOrValue};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct TokenBurn {
    owner_id: AccountId,
    total_supply: Balance,
}

#[near_bindgen]
impl TokenBurn {
    #[init]
    pub fn new(owner_id: AccountId, total_supply: Balance) -> Self {
        Self {
            owner_id,
            total_supply,
        }
    }

    pub fn burn(&mut self, amount: Balance) -> PromiseOrValue<()> {
        assert!(amount > 0, "Amount must be greater than zero");
        assert!(self.total_supply >= amount, "Not enough tokens to burn");

        self.total_supply -= amount;

        Promise::new(env::predecessor_account_id()).transfer(amount)
    }
}
