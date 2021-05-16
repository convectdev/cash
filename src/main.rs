mod helpers;
use helpers::create::*;

fn main() {
    let cash = Cash::create_cash("usd", 17500).add(700).add(1000);
    let new_cash = cash.add(2000).add(89000).subtract(87000).subtract(4001).multiply(2).divide(4);
    println!("{} {}", cash.amount, new_cash.amount);
}