fn main() {
    println!("Enter the center point");

    let mut center_point = String::new();
    std::io::stdin()
        .read_line(&mut center_point)
        .expect("Failed to read line");

    let center_point = {
        let center_point = center_point
            .trim()
            .split_whitespace()
            .collect::<String>()
            .split(",")
            .map(|x| x.replace(&['(', ')'][..], "").parse::<f64>().unwrap())
            .collect::<Vec<f64>>();
        (center_point[0], center_point[1])
    };

    println!("Enter either the radius or the diameter (enter `r=' or `d=')");

    let mut radius = String::new();
    std::io::stdin()
        .read_line(&mut radius)
        .expect("Failed to read line");

    let radius = {
        let radius = radius
            .trim()
            .split_whitespace()
            .collect::<String>()
            .to_lowercase();
        match &radius[..2] {
            "r=" => radius[2..].parse::<f64>().unwrap(),
            "d=" => radius[2..].parse::<f64>().unwrap()/2.,
            _ => unreachable!()
        }
    };

    println!("{}^2 = (y - {})^2 + (x - {})^2", radius, center_point.1, center_point.0);
}
