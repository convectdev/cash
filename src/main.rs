#[allow(dead_code)]

struct Cash {
    currency: String,
    amount: i64,
}

impl Cash {
    fn get_amount(&self) -> String {
        format!("{}", self.amount)
    }
}


fn main() {
    let my_new_obj = Cash {
        currency: String::from("dolari"),
        amount: 570,
    };
    println!("main output: {}", my_new_obj.get_amount());
}