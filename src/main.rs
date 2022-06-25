fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    println!("Fibonacci generator");
    println!("{}", fibonacci(0));
    println!("{}", fibonacci(1));
    println!("{}", fibonacci(2));
    println!("{}", fibonacci(3));
    println!("{}", fibonacci(4));
    println!("{}", fibonacci(5));
    println!("{}", fibonacci(6));
    println!("{}", fibonacci(7));
    println!("{}", fibonacci(8));
    println!("{}", fibonacci(9));
    println!("{}", fibonacci(10));
    println!("{}", fibonacci(11));
    println!("{}", fibonacci(12));
    println!("{}", fibonacci(13));
    println!("{}", fibonacci(14));
    println!("{}", fibonacci(15));
    println!("{}", fibonacci(16));
    println!("{}", fibonacci(17));
    println!("{}", fibonacci(18));
    println!("{}", fibonacci(19));
    println!("{}", fibonacci(20));
}