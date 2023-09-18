mod Rusty;
use std::io;
use Rusty::*;
fn main() {
    let mut choice = String::new();
    println!("Enter Choice O for check odd-even or T for Loop");
    io::stdin()
    .read_line(&mut choice)
    .expect("Faild to read Data");

    match  choice.trim() {
        "O" => Rusty::is_odd_even(),
        "T" => Rusty::infinite(),
        _ => println!("Invalid Input"),
    };
}
