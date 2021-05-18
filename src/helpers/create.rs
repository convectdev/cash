pub struct Cash {
    pub currency: String,
    pub amount: i64,
}

pub trait CashOps {
    fn create_cash(currency: &str, amount: i64) -> Self;
    fn get_currency(&self) -> String;
    fn get_amount(&self) -> i64;
}

impl CashOps for Cash {
    fn create_cash(currency: &str, amount: i64) -> Self {
        Self {
            currency: String::from(currency),
            amount: amount,
        }
    }

    fn get_currency(&self) -> String {
        String::from(&self.currency)
    }

    fn get_amount(&self) -> i64 {
        self.amount
    }
}