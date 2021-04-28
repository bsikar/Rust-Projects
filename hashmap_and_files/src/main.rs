use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input_file").unwrap();
    let reader = BufReader::new(file);
    let mut map = HashMap::new();

    for line in reader.lines() {
        *map.entry(line.unwrap()).or_insert(0) += 1;
    }
    println!("{:#?}", map);
}
