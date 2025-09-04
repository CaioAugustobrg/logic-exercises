// 33. Escreva um programa que solicite um número inteiro e verifique se é divisível por 3 e por 5 ao mesmo tempo.
use std::io;

fn main() {
    let number: i32 = read_input("Type a number");
    if number % 3 == 0 && number % 5 == 0 {
        println!("{} is divisible by 5 and 3", number)
    } else {
        println!("{} is NOT divisible by 5 and 3", number)
    }
}

fn read_input(message: &str) -> i32 {
    println!("{}", message);
    
    loop {
        let mut input: String = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
        
        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid number. Try again")
        }
    }
}