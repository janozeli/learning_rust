use std::{io, u64};

fn factorial(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => factorial(n - 1) * n,
    }
}

fn main() {
    println!("--- Factorial of N ---");
    println!("Select a number: ");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let input: u64 = input.trim().parse().unwrap();

    println!("{}! = {}", input, factorial(input));
}
