use std::io;
use rand::Rng;

use crate::garden::vegetables::Asparagus;
pub mod garden;
fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    println!("please input your guess");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("you guessed: {guess}");
}
