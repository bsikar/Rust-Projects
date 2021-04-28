use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("test.csv").unwrap();
    let reader = BufReader::new(file);
    let mut map = BTreeMap::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap().parse::<u32>().unwrap();
        map.entry(line).or_insert((0, vec![]));
        map.get_mut(&line).unwrap().1.push(i + 1);
        map.insert(line, (map[&line].0 + 1, map[&line].1.clone()));
    }
    println!("{:#?}", map);
}
