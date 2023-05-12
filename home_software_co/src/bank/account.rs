pub struct Account {
    balance: f64,
    int_rate: f64,
    interest: f64,
    transactions: i32,
}

impl Account {
    pub fn new() -> Account {
        Account {
            balance: 0.0,
            int_rate: 0.045,
            interest: 0.0,
            transactions: 0,
        }
    }

    pub fn make_deposit(&mut self, balance: f64) {
        self.balance = balance;
        self.transactions += 1;
    }

    pub fn withdraw(&mut self, amount: f64) -> Option<f64> {
        if amount <= self.balance {
            self.transactions += 1;
            self.balance -= amount;
            Some(self.balance)
        } else {
            None
        }
    }

    pub fn calc_interest(&mut self) {
        self.interest = self.balance * self.int_rate;
        self.balance += self.interest;
    }

    pub fn get_balance(&self) -> f64 {
        self.balance
    }

    pub fn get_transactions(&self) -> i32 {
        self.transactions
    }

    pub fn get_interest(&self) -> f64 {
        self.interest
    }
}
