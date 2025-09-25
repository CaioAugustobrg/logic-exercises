// 61. Crie um programa que leia um vetor de números inteiros e exiba a soma de todos os elementos.
use std::io;

fn main() {
    let total_numbers: i32 = get_input("Enter the amount of numbers you want to input:");
    println!("You will enter {total_numbers} numbers.");
    
    let mut numbers: Vec<i32> = Vec::new();
    
    for _ in 1..=total_numbers {
        let value = get_input("Enter a number:");
        println!("You entered: {value}");
        numbers.push(value);
    }
    
    let mut sum: i32 = 0;
    for num in numbers {
        sum += num;
    }
    
    println!("The sum of all numbers is: {sum}");
}

fn get_input(message: &str) -> i32 {
    println!("{message}");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        
        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid number. Try again."),
        }
    }
}
