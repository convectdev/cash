use super::create::Cash;

pub trait Operations {
    fn add(&self, amount: i64) -> Self;
    fn subtract(&self, amount: i64) -> Self;
    fn multiply(&self, amount: i64) -> Self;
    fn divide(&self, amount: i64) -> Self;
}

impl Operations for Cash {
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

    fn multiply(&self, amount: i64) -> Self {
        let new_amount: i64 = &self.amount * amount;
        Self {
            currency: String::from(&self.currency),
            amount: new_amount,
        }
    }

    fn divide(&self, amount: i64) -> Self {
        let new_amount: i64 = &self.amount / amount;
        Self {
            currency: String::from(&self.currency),
            amount: if new_amount < 0 { 0 } else { new_amount },
        }
    }
}