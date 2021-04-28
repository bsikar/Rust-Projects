use num_bigint::BigUint;

fn main() {
    println!("{} {}", 99, fib(100000));

    println!("Done");
}

fn fib(n: u32) -> BigUint {
    if n < 2 {
        return BigUint::from(1u32);
    }

    let mut sum = BigUint::from(0u32);
    let mut last = BigUint::from(0u32);
    let mut curr = BigUint::from(1u32);

    for _ in 1..n {
        sum = &last + &curr;
        last = curr;
        curr = sum.to_owned();
    }

    sum
}
