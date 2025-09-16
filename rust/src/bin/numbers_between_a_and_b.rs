//49. Escreva um programa que solicite ao usuário dois números A e B e exiba todos os números entre A e B.
use std::io;


fn main() {
    let first_number: i32 = get_input("Please, type the first number");
    let second_number: i32 = get_input("Please, type the second number");
    
    if first_number <= second_number {
        for num in first_number..=second_number {
            println!("{}", num);
        }
    } else {
         for num in second_number..=first_number {
            println!("{}", num);
        }
    }
}

fn get_input(message: &str) -> i32 {
    println!("{}", message);
    loop {
        let mut input: String = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to get input");
        
        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Try again!")
        }
    }
}