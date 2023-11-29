use std::{io, u64};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
fn main() {
    println!("--- Fibonacci Sequency N ---");
    println!("Select a number: ");

    let mut n = String::new();

    io::stdin().read_line(&mut n).unwrap();

    let n: u64 = n.trim().parse().unwrap();

    for x in 1..=n {
        println!("{} - {}", x, fibonacci(x));
    }
}
