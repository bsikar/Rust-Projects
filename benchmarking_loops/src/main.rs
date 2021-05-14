use std::time::Instant;

fn main() {
    print!("while_loop1  ");
    benchmark1(while_loop1);
    print!("for_loop1  ");
    benchmark1(for_loop1);
    print!("iter1  ");
    benchmark1(iter1);

    print!("while_loop2  ");
    benchmark2(100_000_000, while_loop2);
    print!("for_loop2  ");
    benchmark2(100_000_000, for_loop2);
    print!("iter2  ");
    benchmark2(100_000_000, iter2);

    print!("while_loop3  ");
    benchmark3(100_000_000, while_loop3);
    print!("for_loop3  ");
    benchmark3(100_000_000, for_loop3);
    print!("iter3  ");
    benchmark3(100_000_000, iter3);
}

fn benchmark1(func: impl Fn()) {
    let now = Instant::now();

    func();

    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed);
}

fn benchmark2(input: usize, func: impl Fn(usize)) {
    let now = Instant::now();

    func(input);

    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed);
}

fn benchmark3(input: usize, func: impl Fn(usize) -> usize) {
    let now = Instant::now();

    func(input);

    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed);
}

fn while_loop1() {
    let mut i = 0usize;
    let mut _s = 0usize;

    while i < 100_000_000 {
        _s += i;
        i += 1;
    }
}

fn while_loop2(n: usize) {
    let mut i = 0usize;
    let mut _s = 0usize;

    while i < n {
        _s += i;
        i += 1;
    }
}

fn while_loop3(n: usize) -> usize {
    let mut i = 0usize;
    let mut s = 0usize;

    while i < n {
        s += i;
        i += 1;
    }

    s
}

fn for_loop1() {
    let mut _s = 0usize;

    for i in 0..100_000_000 {
        _s += i;
    }
}

fn for_loop2(n: usize) {
    let mut _s = 0usize;

    for i in 0..n {
        _s += i;
    }
}

fn for_loop3(n: usize) -> usize {
    let mut s = 0usize;

    for i in 0..n {
        s += i;
    }

    s
}

fn iter1() {
    let mut s = 0usize;

    (0..100_000_000).for_each(|i| s += i);
}

fn iter2(n: usize) {
    let mut s = 0usize;

    (0..n).for_each(|i| s += i);
}

fn iter3(n: usize) -> usize {
    let mut s = 0usize;

    (0..n).for_each(|i| s += i);

    s
}
