#![allow(non_snake_case)]
use std::io::prelude::*;

fn main() {
    loop {
        let mut input = String::new();
        println!("\nEnter type of problem ('SSS, SAS, SSA, ASA, AAS, AAA')");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.pop();
        input = input.to_uppercase();
        if input == "SSS" {
            sss();
        } else if input == "SAS" {
            sas();
        } else if input == "SSA" {
            ssa();
        } else if input == "ASA" {
            asa();
        } else if input == "AAS" {
            aas();
        } else if input == "AAA" {
            aaa();
        } else if input == "QUIT" || input == "EXIT" {
            break;
        }
    }
}

fn sss() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    print!("Side a: ");
    std::io::stdout()
        .flush()
        .ok()
        .expect("Could not flush stdout");
    std::io::stdin()
        .read_line(&mut a)
        .expect("Failed to red line");
    a.pop();
    let a = a.parse::<f64>().unwrap();

    print!("Side b: ");
    std::io::stdout()
        .flush()
        .ok()
        .expect("Could not flush stdout");
    std::io::stdin()
        .read_line(&mut b)
        .expect("Failed to red line");
    b.pop();
    let b = b.parse::<f64>().unwrap();

    print!("Side c: ");
    std::io::stdout()
        .flush()
        .ok()
        .expect("Could not flush stdout");
    std::io::stdin()
        .read_line(&mut c)
        .expect("Failed to red line");
    c.pop();
    let c = c.parse::<f64>().unwrap();

    if (a + b) <= c {
        println!(
            "This is not a valid triangle, since a({}) + b({}) <= c({})",
            a, b, c
        );
        return;
    } else if (b + c) <= a {
        println!(
            "This is not a valid triangle, since b({}) + c({}) <= a({})",
            b, c, a
        );
        return;
    } else if (c + a) <= b {
        println!(
            "This is not a valid triangle, since c({}) + a({}) <= b({})",
            c, a, b
        );
        return;
    }

    let A = ((b.powf(2.) + c.powf(2.) - a.powf(2.)) / (2. * b * c))
        .acos()
        .to_degrees();
    let B = ((c.powf(2.) + a.powf(2.) - b.powf(2.)) / (2. * c * a))
        .acos()
        .to_degrees();
    let C = 180. - A - B;
    let area = (4. * a.powf(2.) * b.powf(2.) - (a.powf(2.) + b.powf(2.) - c.powf(2.)).powf(2.))
        .sqrt()
        / 4.;
    println!(
        "\nA = cos^-1(b^2 + c^2 - a^2) / 2bc\nA = cos^-1({}^2 + {}^2 - {}^2) / 2*{}*{}\nA = {}",
        b, c, a, b, c, A
    );
    println!(
        "\nB = cos^-1(c^2 + a^2 - b^2) / 2ca\nB = cos^-1({}^2 + {}^2 - {}^2) / 2*{}*{}\nB = {}",
        c, a, b, c, a, B
    );
    println!("\nC = 180 - A - B\nC = 180 - {} - {}\nC = {}", A, B, C);
    println!("\nArea = sqrt(4 * a^2 * b^2 - (a^2 + b^2 - c^2)^2)\nArea = sqrt(4 * {}^2 * {}^2 - ({}^2 + {}^2 - {}^2)^2)\nArea = {}", a, b, a, b, c, area);
}

fn sas() {
    let mut a = String::new();
    let mut B = String::new();
    let mut c = String::new();

    print!("Side a: ");
    std::io::stdout()
        .flush()
        .ok()
        .expect("Could not flush stdout");
    std::io::stdin()
        .read_line(&mut a)
        .expect("Failed to red line");
    a.pop();
    let a = a.parse::<f64>().unwrap();

    print!("Angle B: ");
    std::io::stdout()
        .flush()
        .ok()
        .expect("Could not flush stdout");
    std::io::stdin()
        .read_line(&mut B)
        .expect("Failed to red line");
    B.pop();
    let B = B.parse::<f64>().unwrap();

    print!("Side c: ");
    std::io::stdout()
        .flush()
        .ok()
        .expect("Could not flush stdout");
    std::io::stdin()
        .read_line(&mut c)
        .expect("Failed to red line");
    c.pop();
    let c = c.parse::<f64>().unwrap();

    if B >= 180. {
        println!("This is not a valid triangle, since B >= 180, (B = {})", B);
        return;
    }

    let b = (a.powf(2.) + c.powf(2.) - 2. * a * c * B.to_radians().cos()).sqrt();
    let A = ((b.powf(2.) + c.powf(2.) - a.powf(2.)) / (2. * b * c))
        .acos()
        .to_degrees();
    let C = 180. - A - B;
    let area = (4. * a.powf(2.) * b.powf(2.) - (a.powf(2.) + b.powf(2.) - c.powf(2.)).powf(2.))
        .sqrt()
        / 4.;
    println!(
        "\nb = sqrt(a^2 + c^2 - 2ac * cos(B))\nb = sqrt({}^2 + {}^2 - 2*{}*{} * cos({}))\nb = {}",
        a, c, a, c, B, b
    );
    println!(
        "\nA = cos^-1(b^2 + c^2 - a^2) / 2bc\nA = cos^-1({}^2 + {}^2 - {}^2) / 2*{}*{}\nA = {}",
        b, c, a, b, c, A
    );
    println!("\nC = 180 - A - B\nC = 180 - {} - {}\nC = {}", A, B, C);
    println!("\nArea = sqrt(4 * a^2 * b^2 - (a^2 + b^2 - c^2)^2)\nArea = sqrt(4 * {}^2 * {}^2 - ({}^2 + {}^2 - {}^2)^2)\nArea = {}", a, b, a, b, c, area);
}

fn ssa() {
    let mut a = String::new();
    let mut b = String::new();
    let mut A = String::new();

    print!("Side a: ");
    std::io::stdout()
        .flush()
        .ok()
        .expect("Could not flush stdout");
    std::io::stdin()
        .read_line(&mut a)
        .expect("Failed to red line");
    a.pop();
    let a = a.parse::<f64>().unwrap();

    print!("Side b: ");
    std::io::stdout()
        .flush()
        .ok()
        .expect("Could not flush stdout");
    std::io::stdin()
        .read_line(&mut b)
        .expect("Failed to red line");
    b.pop();
    let b = b.parse::<f64>().unwrap();

    print!("Angle A: ");
    std::io::stdout()
        .flush()
        .ok()
        .expect("Could not flush stdout");
    std::io::stdin()
        .read_line(&mut A)
        .expect("Failed to red line");
    A.pop();
    let A = A.parse::<f64>().unwrap();

    if A >= 180. {
        println!("This is not a valid triangle, since A >= 180, (A = {})", A);
        return;
    }

    let mut B = ((b * A.to_radians().sin()) / a).asin().to_degrees();
    let mut C = 180. - A - B;
    let mut c = (a * C.to_radians().sin()) / A.to_radians().sin();
    let mut area = (4. * a.powf(2.) * b.powf(2.) - (a.powf(2.) + b.powf(2.) - c.powf(2.)).powf(2.))
        .sqrt()
        / 4.;
    let more_than_one_solution = ((180. - B) + A) < 180.;
    if more_than_one_solution {
        println!("\nSolution one:\n==================");
        println!(
            "c = (a * sin(C)) / sin(A))\nc = ({} * sin({})) / sin({}))\nc = {}",
            a, C, A, c
        );
        println!(
            "\nB = sin^-1((b * sin(A)) / a)\nB = sin^-1(({} * sin({})) / {})\nB = {}",
            b, A, a, B
        );
        println!("\nC = 180 - A - B\nC = 180 - {} - {}\nC = {}", A, B, C);
        println!("\nArea = sqrt(4 * a^2 * b^2 - (a^2 + b^2 - c^2)^2)\nArea = sqrt(4 * {}^2 * {}^2 - ({}^2 + {}^2 - {}^2)^2)\nArea = {}", a, b, a, b, c, area);

        println!("\nSolution two:\n==================");
        B = 180. - B;
        C = 180. - A - B;
        c = (a * C.to_radians().sin()) / A.to_radians().sin();
        area = (4. * a.powf(2.) * b.powf(2.) - (a.powf(2.) + b.powf(2.) - c.powf(2.)).powf(2.))
            .sqrt()
            / 4.;
        println!(
            "c = (a * sin(C)) / sin(A))\nc = ({} * sin({})) / sin({}))\nc = {}",
            a, C, A, c
        );
        println!(
            "\nB = sin^-1((b * sin(A)) / a)\nB = sin^-1(({} * sin({})) / {})\nB = {}",
            b, A, a, B
        );
        println!("\nC = 180 - A - B\nC = 180 - {} - {}\nC = {}", A, B, C);
        println!("\nArea = sqrt(4 * a^2 * b^2 - (a^2 + b^2 - c^2)^2)\nArea = sqrt(4 * {}^2 * {}^2 - ({}^2 + {}^2 - {}^2)^2)\nArea = {}", a, b, a, b, c, area);
    } else {
        println!(
            "\nc = (a * sin(C)) / sin(A))\nc = ({} * sin({})) / sin({}))\nc = {}",
            a, C, A, c
        );
        println!(
            "\nB = sin^-1((b * sin(A)) / a)\nB = sin^-1(({} * sin({})) / {})\nB = {}",
            b, A, a, B
        );
        println!("\nC = 180 - A - B\nC = 180 - {} - {}\nC = {}", A, B, C);
        println!("\nArea = sqrt(4 * a^2 * b^2 - (a^2 + b^2 - c^2)^2)\nArea = sqrt(4 * {}^2 * {}^2 - ({}^2 + {}^2 - {}^2)^2)\nArea = {}", a, b, a, b, c, area);
    }
}

fn asa() {
    let mut A = String::new();
    let mut c = String::new();
    let mut B = String::new();

    print!("Angle A: ");
    std::io::stdout()
        .flush()
        .ok()
        .expect("Could not flush stdout");
    std::io::stdin()
        .read_line(&mut A)
        .expect("Failed to red line");
    A.pop();
    let A = A.parse::<f64>().unwrap();

    print!("Side c: ");
    std::io::stdout()
        .flush()
        .ok()
        .expect("Could not flush stdout");
    std::io::stdin()
        .read_line(&mut c)
        .expect("Failed to red line");
    c.pop();
    let c = c.parse::<f64>().unwrap();

    print!("Angle B: ");
    std::io::stdout()
        .flush()
        .ok()
        .expect("Could not flush stdout");
    std::io::stdin()
        .read_line(&mut B)
        .expect("Failed to red line");
    B.pop();
    let B = B.parse::<f64>().unwrap();

    let C = 180. - A - B;
    let a = (c * A.to_radians().sin()) / C.to_radians().sin();
    let b = (c * B.to_radians().sin()) / C.to_radians().sin();
    let area = (4. * a.powf(2.) * b.powf(2.) - (a.powf(2.) + b.powf(2.) - c.powf(2.)).powf(2.))
        .sqrt()
        / 4.;
    println!("\nC = 180 - A - B\nC = 180 - {} - {}\nC = {}", A, B, C);
    println!(
        "\na = (c * sin(A)) / sin(C)\na = ({} * sin({})) / sin({})\na = {}",
        c, A, C, a
    );
    println!(
        "\nb = (c * sin(B)) / sin(C)\na = ({} * sin({})) / sin({})\na = {}",
        b, B, C, b
    );
    println!("\nArea = sqrt(4 * a^2 * b^2 - (a^2 + b^2 - c^2)^2)\nArea = sqrt(4 * {}^2 * {}^2 - ({}^2 + {}^2 - {}^2)^2)\nArea = {}", a, b, a, b, c, area);
}

fn aas() {
    let mut A = String::new();
    let mut B = String::new();
    let mut a = String::new();

    print!("Angle A: ");
    std::io::stdout()
        .flush()
        .ok()
        .expect("Could not flush stdout");
    std::io::stdin()
        .read_line(&mut A)
        .expect("Failed to red line");
    A.pop();
    let A = A.parse::<f64>().unwrap();

    print!("Angle B: ");
    std::io::stdout()
        .flush()
        .ok()
        .expect("Could not flush stdout");
    std::io::stdin()
        .read_line(&mut B)
        .expect("Failed to red line");
    B.pop();
    let B = B.parse::<f64>().unwrap();

    print!("Side a: ");
    std::io::stdout()
        .flush()
        .ok()
        .expect("Could not flush stdout");
    std::io::stdin()
        .read_line(&mut a)
        .expect("Failed to red line");
    a.pop();
    let a = a.parse::<f64>().unwrap();

    let C = 180. - A - B;
    let b = (a * B.to_radians().sin()) / A.to_radians().sin();
    let c = (a * C.to_radians().sin()) / A.to_radians().sin();
    let area = (4. * a.powf(2.) * b.powf(2.) - (a.powf(2.) + b.powf(2.) - c.powf(2.)).powf(2.))
        .sqrt()
        / 4.;
    println!("\nC = 180 - A - B\nC = 180 - {} - {}\nC = {}", A, B, C);
    println!(
        "\nb = (a * sin(B)) / sin(A)\nb = ({} * sin({})) / sin({})\nb = {}",
        a, B, A, b
    );
    println!(
        "\nc = (a * sin(C)) / sin(A)\nc = ({} * sin({})) / sin({})\nc = {}",
        a, C, A, c
    );
    println!("\nArea = sqrt(4 * a^2 * b^2 - (a^2 + b^2 - c^2)^2)\nArea = sqrt(4 * {}^2 * {}^2 - ({}^2 + {}^2 - {}^2)^2)\nArea = {}", a, b, a, b, c, area);
}

fn aaa() {
    println!("One side is needed.");
}
