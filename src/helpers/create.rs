pub struct Cash {
    pub currency: String,
    pub amount: i64,
}

pub trait CashOps {
    fn create_cash(currency: &str, amount: i64) -> Self;
    fn add(&self, amount: i64) -> Self;
    fn subtract(&self, amount: i64) -> Self;
}

impl CashOps for Cash {
    fn create_cash(currency: &str, amount: i64) -> Self {
        Self {
            currency: String::from(currency),
            amount: amount,
        }
    }
    fn add(&self, amount: i64) -> Self {
        let new_amount: i64 = &self.amount + amount;
        Self {
            currency: String::from(&self.currency),
            amount: new_amount,
        }
    }
    fn subtract(&self, amount: i64) -> Self {
        let new_amount: i64 = &self.amount - amount;
        Self {
            currency: String::from(&self.currency),
            amount: if new_amount < 0 { 0 } else { new_amount },
        }
    }
}