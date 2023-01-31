use std::io;
use rand::Rng;

fn main() {
    println!("Please input a range of numbers to guess between.");

    println!("Low number will be 1. What do you want a high number to be?");

    let mut high = String::new();

    //Get input for number range
    io::stdin()
        .read_line(&mut high)
        .expect("Failed to read line");
    
    println!("You input: {high}");

    let high_num : i32 = high.trim().parse().unwrap();

    let mut secret_number : i32 = rand::thread_rng().gen_range(1..=high_num);

    println!("The secret number is : {secret_number}.");

    println!("Please enter a guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}.");

    let mut guess_num: i32 = guess.trim().parse().unwrap();

    compare_guess_and_secret(&mut guess_num, &mut secret_number);

    fn compare_guess_and_secret(g:&mut i32, s:&mut i32) {
        if g < s {
            println!("Higher");
        } else if g > s {
            println!("Lower");
        } else {
            println!("You guessed the secret number {s}. You win!");
        }
    }



}