use std::io;

fn main() {
    println!("Please input a number");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("You should input a number");
    let n: u32 = input.trim().parse().expect("Please type a number");
    for i in 0..n{
        println!("Fibonacci{} is {}",i,fibonacci(i));
    }
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n-2)
    }
}
