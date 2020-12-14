use rand::Rng;
use std::io::prelude::*;

fn main() {
    println!("Please think of a number between 1 and 100 (inclusive)\n\
              .... Now, please answer lower, higher, or yes, based on my questions.");
    binary_search(1, 100);
}

fn binary_search(lower_bound: u16, upper_bound: u16) {
    if lower_bound > upper_bound {
        println!("You must have cheated!");
        return
    }
    let mut guess: u16 = lower_bound;
    if lower_bound < upper_bound {
        guess = rand::thread_rng().gen_range(lower_bound, upper_bound);
    }
    print!("Is your number {}? ", guess);
    std::io::stdout().flush().ok().expect("Could not flush stdout");
    let mut response = String::new();
    std::io::stdin().read_line(&mut response).expect("Failed to read line");
    match response.as_str() {
        "higher\n" => binary_search(guess+1, upper_bound),
        "lower\n" => binary_search(lower_bound, guess-1),
        _ => {
            println!("I win!");
            return
        }
    }
}
