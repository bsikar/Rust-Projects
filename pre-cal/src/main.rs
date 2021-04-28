use std::io::prelude::*;

fn main() {
    let mut angle = take_input("Angle");

    if angle < 0.0 {
        let lower_bound = take_input("Lower Bound");
        while angle < lower_bound {
            angle += 360.0;
        }
    } else {
        let upper_bound = take_input("Upper Bound");
        while angle >= upper_bound {
            angle -= 360.0;
        }
    }
    if angle >= 0.0 {
        println!(
            "\nCoterminal Angle: {}\nQuadrant: {}\nReference Angle: {}",
            angle,
            find_quadrant(angle),
            find_reference_angle(angle)
        );
    } else {
        println!("Coterminal Angle: {}", angle);
    }
}

fn take_input(var: &str) -> f64 {
    print!("{}: ", var);
    std::io::stdout()
        .flush()
        .ok()
        .expect("Could not flush stdout");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("\nInvalid Input\n-------------\nOnly enter whole or decimal numbers; you can enter negative numbers.\n\nExamples\n--------\n{x}: 32\n{x}: -43\n{x}: 54.343\n{x}: -543.10\n", x=var);
            match var {
                "Angle" => println!("You were supposed to enter your angle.\n"),
                "Lower Bound" => println!("You were supposed to enter your lower bound, this is what the angle has to be greater than\n\nExample\n-------\nFind an angle \'x\' coterminal to 1021 degrees, where 0 degrees <= x < 360 degrees\n** Your lower bound would be 0 here **\n"),
                "Upper Bound" => println!("You were supposed to enter your upper bound, this is what the angle has to be less than\n\nExample\n-------\nFind an angle \'x\' coterminal to 1021 degrees, where 0 degrees <= x < 360 degrees\n** Your upper bound would be 360 here **\n"),
                _ => (),
            }
            panic!();
        }
    }
}

fn find_reference_angle(var: f64) -> u16 {
    if var <= 90.0 {
        var as u16
    } else if var <= 180.0 {
        (180.0 - var) as u16
    } else if var <= 270.0 {
        (var - 180.0) as u16
    } else {
        (360.0 - var) as u16
    }
}

fn find_quadrant(var: f64) -> u8 {
    if var <= 90.0 {
        1
    } else if var <= 180.0 {
        2
    } else if var <= 270.0 {
        3
    } else {
        4
    }
}
