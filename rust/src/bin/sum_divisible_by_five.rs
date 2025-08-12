//Faça um programa que leia três números, e informe se a soma deles é divisível por 5 ou não.
use std::io;

fn main() {
    let first_number: i32 = read_input("Type the first number");
    let second_number: i32 = read_input("Type the second number");
    let third_number: i32 = read_input("Type the third number");

    let sum: i32 = first_number + second_number + third_number;

    if sum % 5 == 0 {
        println!("The sum of the numbers is divisible by 5")
    } else {
        println!("The sum of the numbers is NOT divisible by 5")
    }
}

fn read_input(message: &str) -> i32 {
    println!("{}", message);
    loop {
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<i32>() {
            Ok(num) => break num,
            Err(_) => println!("Invalid input. Try again"),
        }
    }
}
