mod triangle;
use triangle::*;

fn main() {
    let _triangle = Triangle::new()
        .add_sides(vec![('a', Some(12.)), ('b', Some(15.)), ('c', Some(12.))])
        .build();
}

#[cfg(test)]
mod test;
