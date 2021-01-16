mod helpers;
// use helpers::create::create_cash;

fn main() {
    let cash = helpers::create::create_cash("usd", 15000);
    println!("{} {}", cash.amount, cash.currency)
}