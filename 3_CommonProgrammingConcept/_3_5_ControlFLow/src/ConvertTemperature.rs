use std::io;

fn main() {
    println!("Please input Fahrenheit");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let fahrenheit: f32 = input
        .trim()
        .parse()
        .expect("Please type a number");

    let celsius = convert_to_celsius(fahrenheit);
    println!("convertToCelsius is {celsius}");
}

fn convert_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) / 1.8
}
