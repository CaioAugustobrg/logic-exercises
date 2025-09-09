//44. Escreva um programa que calcule e exiba o valor da potência de um número 
//informado pelo usuário elevado a um expoente também informado pelo usuário, utilizando laços de repetição.

use std::io;

fn main() {
    let base: i32 = read_input("Please, type the base number");
    let exponent: i32 = read_input("Please, type the exponent number");
    
    let mut result: i32 = 1;
    
    for _ in 1..=exponent {
        result = result * base;
    }
    
    println!("The base is {}, the exponent is {}, and the result is {}", base, exponent, result);
}


fn read_input(message: &str) -> i32 {
    println!("{}", message);
    loop {
        let mut input: String = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
        
        match input.trim().parse::<i32>() {
            Ok(value) => break value,
            Err(_) => println!("Invalid number. Try again!")
        }
    }
}