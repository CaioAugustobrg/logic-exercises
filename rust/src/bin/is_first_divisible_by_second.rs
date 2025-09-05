//Faça um programa que solicite dois números e exiba se o primeiro é divisível pelo segundo.
use std::io;

fn main() {
    let first_number: i32 = read_input("Please type the first number");
    let second_number: i32 = read_input("Please type the second number");
    verify_divisibility(first_number, second_number);
}

fn read_input(message: &str) -> i32 {
    println!("{}", message);
    loop {
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<i32>() {
            Ok(value) => {
                println!("You typed: {}", value);
                return value;
            }
            Err(_) => println!("Invalid number. Try again"),
        }
    }
}

fn verify_divisibility(first_number: i32, second_number: i32) -> () {
    if first_number % second_number == 0 {
        println!("{} is divisible by {}", first_number, second_number)
    } else {
        println!("{} is NOT divisible by {}", first_number, second_number)
    }
}
