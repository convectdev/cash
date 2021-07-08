mod helpers;
use helpers::create::*;
use helpers::calculate::*;

fn main() {
    let _start_here = Cash {
        currency: String::from("eur"),
        amount: 3400,
    };

    let another = Cash::create_cash(String::from("ursu"), 20000000).subtract(1).get_currency();
    println!("you have {}", another);
}




