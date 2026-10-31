// Exercise 5
// Small application that randomly generates numbers
// and tries to guess a target between 1 and 10
use rand::Rng;
use thiserror::Error;

#[derive(Error, Debug)]
enum GuessError {
    #[error("The target is higher!")]
    Higher,

    #[error("The target is lower!")]
    Lower,
}

fn print_error(error: GuessError) {
    println!("Error: {error}")
}

// Generate a number between 1 and 10
fn gen_num() -> u8 {
    let mut rng = rand::rng();
    rng.random_range(1..=10) as u8
}

// Generate a result depending on the guess
fn gen_result(target: u8, guess: u8) -> Result<u8, GuessError> {
    if guess < target {
        return Err(GuessError::Higher);
    } else if guess > target {
        return Err(GuessError::Lower);
    }

    Ok(target)
}

// Play the guessing game (MAX: 5 tries)
pub fn play() -> Option<String> {
    let target = gen_num();
    let max_tries = 5;

    let mut guess = 11;
    let mut tries = 0;

    while guess != target && tries < max_tries {
        guess = gen_num();
        tries += 1;

        print!("guess: {guess}: ");
        match gen_result(target, guess) {
            Err(error) => print_error(error),
            Ok(_) => println!("Target found!"),
        }
    }

    if guess == target {
        return Some(String::from("Number has been guessed!"));
    }
    None
}
