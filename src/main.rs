mod helpers;
use helpers::create::*;

fn main() {
    let cash = Cash::create_cash("usd", 17500).add(700).add(1000).subtract(29200);
    println!("{} {}", cash.amount, cash.currency);
}