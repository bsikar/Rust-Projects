use std::env;
use std::fs::OpenOptions;
use std::io::{Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().skip(1);
    let empty = args.len() == 0;
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("foo.txt")?;

    for arg in args {
        writeln!(file, "{}", arg)?;
    }

    if empty {
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        print!("{}", contents);
        std::io::stdout().flush().unwrap();
    }
    file.flush()?;
    Ok(())
}
