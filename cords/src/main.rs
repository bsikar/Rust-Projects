fn main() {
    println!("Enter the center point");

    let mut center_point = String::new();
    std::io::stdin()
        .read_line(&mut center_point)
        .expect("Couldn't read line");

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

    println!("{:?}", center_point);
}
