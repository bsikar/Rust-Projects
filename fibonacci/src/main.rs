fn main() {
    for i in 0..=10 {
        println!("fib({}): {}", i, fib(i));
    }
}

fn fib(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
