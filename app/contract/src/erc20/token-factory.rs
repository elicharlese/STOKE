use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::env;
use near_sdk::ext_contract;
use near_sdk::json_types::U128;
use near_sdk::near_bindgen;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;
use near_sdk::Balance;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct TokenFactory {
    tokens: UnorderedMap<AccountId, Token>,
}

#[near_bindgen]
impl TokenFactory {
    pub fn new() -> Self {
        Self {
            tokens: UnorderedMap::new(b"t".to_vec()),
        }
    }

    pub fn create_token(&mut self, name: String, symbol: String, total_supply: U128) -> Token {
        let owner = env::predecessor_account_id();
        let token = Token::new(name, symbol, total_supply, owner.clone());
        self.tokens.insert(&owner, &token);
        token
    }

    pub fn get_token(&self, owner: AccountId) -> Option<Token> {
        self.tokens.get(&owner)
    }
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
pub struct Token {
    name: String,
    symbol: String,
    total_supply: U128,
    owner: AccountId,
}

impl Token {
    pub fn new(name: String, symbol: String, total_supply: U128, owner: AccountId) -> Self {
        Self {
            name,
            symbol,
            total_supply,
            owner,
        }
    }
}
