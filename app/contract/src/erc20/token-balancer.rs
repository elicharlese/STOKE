use std::collections::HashMap;

struct TokenBalancer {
    usdc_balance: u64,
    token_supply: u64,
    token_balances: HashMap<String, u64>,
}

impl TokenBalancer {
    fn new() -> Self {
        TokenBalancer {
            usdc_balance: 0,
            token_supply: 0,
            token_balances: HashMap::new(),
        }
    }

    fn deposit_usdc(&mut self, amount: u64) {
        self.usdc_balance += amount;
    }

    fn withdraw_usdc(&mut self, amount: u64) {
        if amount <= self.usdc_balance {
            self.usdc_balance -= amount;
        } else {
            panic!("Insufficient USDC balance");
        }
    }

    fn mint_tokens(&mut self, recipient: String, amount: u64) {
        self.token_supply += amount;
        let balance = self.token_balances.entry(recipient).or_insert(0);
        *balance += amount;
    }

    fn burn_tokens(&mut self, recipient: String, amount: u64) {
        if let Some(balance) = self.token_balances.get_mut(&recipient) {
            if amount <= *balance {
                self.token_supply -= amount;
                *balance -= amount;
            } else {
                panic!("Insufficient token balance");
            }
        } else {
            panic!("Recipient does not have any tokens");
        }
    }
}
