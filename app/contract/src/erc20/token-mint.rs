use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct TokenMint {
    owner_id: AccountId,
    total_supply: Balance,
}

#[near_bindgen]
impl TokenMint {
    #[init]
    pub fn new(owner_id: AccountId, total_supply: Balance) -> Self {
        Self {
            owner_id,
            total_supply,
        }
    }

    pub fn mint(&mut self, amount: Balance) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only the owner can mint tokens"
        );

        self.total_supply += amount;
    }

    pub fn get_total_supply(&self) -> Balance {
        self.total_supply
    }
}
