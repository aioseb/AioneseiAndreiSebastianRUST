mod app;
mod char_fcts;
mod math;
mod primes;

use primes::next_prime;

use app::play;

use math::panic_add;
use math::panic_mul;
use math::result_add;
use math::result_mul;

use char_fcts::char_to_number;
use char_fcts::char_to_number_hex;
use char_fcts::print_char;
use char_fcts::print_error;
use char_fcts::to_lowercase;
use char_fcts::to_uppercase;

// Test functions
fn test_exercise_1() {
    let mut temp: u16 = 0;
    while let Some(value) = next_prime(temp) {
        print!("{value} ");
        temp = value;
    }
    println!();
}

fn test_exercise_2() {
    println!("Non-panic add: {}", panic_add(53, 11));
    println!("Non-panic mul: {}", panic_mul(53, 11));

    // println!("Panic add: {}", panic_add(2147483648, 2147483648));
    // println!("Panic mul: {}", panic_mul(2147483648, 2147483648));
}

fn test_exercise_3() {
    match result_add(53, 11) {
        Ok(value) => println!("Ok add: {value}"),
        Err(error) => println!("Error: {error}"),
    }

    match result_mul(53, 11) {
        Ok(value) => println!("Ok mul: {value}"),
        Err(error) => println!("Error: {error}"),
    }

    match result_add(2147483648, 2147483648) {
        Ok(value) => println!("Ok add: {value}"),
        Err(error) => println!("Error: {error}"),
    }

    match result_mul(2147483648, 2147483648) {
        Ok(value) => println!("Ok mul: {value}"),
        Err(error) => println!("Error: {error}"),
    }
}

fn text_exercise_4() {
    match to_lowercase('B') {
        Ok(res) => println!("to_lower: {res}"),
        Err(error) => print_error(error),
    }
    match to_lowercase('4') {
        Ok(res) => println!("to_lower: {res}"),
        Err(error) => print_error(error),
    }

    match to_uppercase('g') {
        Ok(res) => println!("to_upper: {res}"),
        Err(error) => print_error(error),
    }
    match to_uppercase('4') {
        Ok(res) => println!("to_upper: {res}"),
        Err(error) => print_error(error),
    }

    match char_to_number('9') {
        Ok(res) => println!("char_to_number: {res}"),
        Err(error) => print_error(error),
    }
    match char_to_number('f') {
        Ok(res) => println!("char_to_number: {res}"),
        Err(error) => print_error(error),
    }

    match char_to_number_hex('C') {
        Ok(res) => println!("char_to_number_hex: {res}"),
        Err(error) => print_error(error),
    }
    match char_to_number_hex('Z') {
        Ok(res) => println!("char_to_number_hex: {res}"),
        Err(error) => print_error(error),
    }

    match print_char('P') {
        Ok(_) => println!(" <- print_char"),
        Err(error) => print_error(error),
    }
    match print_char('\0') {
        Ok(_) => println!(" <- print_char"),
        Err(error) => print_error(error),
    }
}

fn test_exercise_5() {
    play();
}

fn main() {
    test_exercise_1();
    test_exercise_2();
    test_exercise_3();
    text_exercise_4();
    test_exercise_5();
}
