pub struct Cash {
    pub currency: String,
    pub amount: i64,
}

pub fn create_cash(currency: &str, amount: i64) -> Cash {
    let cash: Cash = Cash {
        currency: String::from(currency),
        amount: amount,
    };
    cash
}