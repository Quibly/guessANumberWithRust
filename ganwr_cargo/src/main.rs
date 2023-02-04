use rand::Rng;
use std::io;
struct GameObject {
    guess: i32,
    guesses: Vec<i32>,
    secret_number: i32
}

impl GameObject{
    //Method to create GameObject
    fn create(x: i32) -> GameObject {
        let y: i32 = rand::thread_rng().gen_range(1..=x);

        GameObject { guess: 0, guesses: Vec::new(), secret_number: y }
    }
}

impl GameObject {
    //Method to get the high number of the range to guess from
    fn get_high_number() -> i32 {
        println!();
        println!("Please input a range of numbers to guess between.");

        println!("Low number will be 1. What do you want a high number to be?");

        let mut high = String::new();

        // Get input for number range for init.
        io::stdin()
            .read_line(&mut high)
            .expect("Failed to read line");
        
        println!("You input: {}", &mut high);

        // Parse string to int for generating secret_number range.
        let high_num : i32 = high.trim().parse().unwrap();
        
        return high_num;
    }
}

impl GameObject {
    //Function to setup new object
    fn setup() -> GameObject {
        let high_number : i32 = GameObject::get_high_number();
        GameObject::create(high_number)
    }
}

impl GameObject {
    //Module to loop guesses for gameplay
    fn loop_game_guesses(game:&mut GameObject) -> &mut GameObject {
        // If the user guesses correctly then the program will end.
        loop {

            game.guess = GameObject::get_guess();
            game.guesses.push(game.guess);

            if game.guess == game.secret_number {
            
                println!("You guessed the secret number {}. You win!", game.secret_number);
            
                break;
            }

            GameObject::compare_guess_and_secret(&game.guess, &game.secret_number);
        }
        return game;
    }

    // Function to compare the guess and the secret number.
    // Gives a hint to the user of "Higher" or "Lower" depending on guess.
    fn compare_guess_and_secret(g:&i32, s:&i32) {
        if g < s {
            println!("Higher");
        } else if g > s {
            println!("Lower");
        }
    }

    // Method to get the next guess from the user. Returns guess.
    fn get_guess() -> i32 {
        
        println!("Please enter a guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", &mut guess);

        let guess_num: i32 = guess.trim().parse().unwrap();

        return guess_num;
    }
}

impl GameObject {
    fn show_game_stats(game:&mut GameObject) {
        println!("You found the secret number in {} guesses.", game.guesses.len());
        println!("Guesses: {:?}", game.guesses);
        println!();
    }
}

fn main() {
    // Initialize the program by getting the range the user wants to guess in.
    let mut game_build = GameObject::setup();
    // Start main loop for guessing and check for correct guess.
    let mut game_build = GameObject::loop_game_guesses(&mut game_build);
    // Print results of game
    GameObject::show_game_stats(&mut game_build);

}