mod helpers;
use helpers::create::*;
use helpers::calculate::*;

fn main() {
   

    let mata = Cash::create_cash("usd", 12000).add(500).subtract(2200).multiply(3).subtract(900).divide(2);
    println!("you have {} {}", mata.get_amount(), mata.get_currency());
}