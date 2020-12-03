use std::io::prelude::*;

fn main() {
    let mut angle = take_input("Angle");

    if angle < 0 {
        let lower_bound = take_input("Lower Bound");
        while angle < lower_bound {
            angle += 360;
        }
    }
    else {
        let upper_bound = take_input("Upper Bound");
        while angle > upper_bound {
            angle -= 360;
        }
    }

    println!("Coterminal Angle: {}", angle);
}

fn take_input(var: &str) -> i32 {
    print!("{}: ",var);
    std::io::stdout().flush().ok().expect("Could not flush stdout");
    let mut input  = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("What happened?")
}
