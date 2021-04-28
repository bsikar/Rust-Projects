use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let s: String = std::iter::from_fn(|| {
        if rng.gen::<bool>() {
            Some(rng.gen_range('A'..='Z'))
        } else {
            Some(rng.gen_range('a'..='z'))
        }
    })
    .take(5)
    .collect();

    println!("{}", s);
}
