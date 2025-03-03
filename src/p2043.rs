pub fn run() {
    
}

struct Bank {
    pub balances: Vec<i64>,
    pub length: i32
}

impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        Bank {
            length: balance.len() as i32,
            balances: balance
        }
    }
    
    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        let account1 = account1 - 1;
        let account2 = account2 - 1;

        if !(self.validate_account(account1) && self.validate_account(account2)) {
            return false;
        }

        if money < 0 {
            return false;
        }

        if money > self.balances[account1 as usize] {
            return false;
        }

        self.balances[account1 as usize] -= money;
        self.balances[account2 as usize] += money;

        true
    }
    
    fn deposit(&mut self, account: i32, money: i64) -> bool {
        let account = account - 1;

        if !self.validate_account(account) {
            return false;
        }

        if money < 0 {
            return false;
        }

        self.balances[account as usize] += money;

        true
    }
    
    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        let account = account - 1;

        if !self.validate_account(account) {
            return false;
        }

        if money < 0 {
            return false;
        }

        if money > self.balances[account as usize] {
            return false;
        }

        self.balances[account as usize] -= money;

        true
    }

    fn validate_account(&self, account: i32) -> bool {
        (account > 0) && (account < self.length)
    }
}