use rand::seq::IteratorRandom;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    let file = File::open("foo.txt")?;
    let reader = BufReader::new(file);
    let mut rng = rand::thread_rng();

    println!("{}", reader.lines().choose(&mut rng).unwrap()?);

    Ok(())
}
