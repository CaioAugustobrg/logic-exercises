// 42. Escreva um programa que solicite ao usuário um número N e exiba a soma de todos os números de 1 a N.

use std::io;

fn main() {
    println!("Please, type a number");
    let mut number: String = String::new();
    io::stdin()
    .read_line(&mut number)
    .expect("Failed to read input");
    
    let number: i32 = number.trim().parse().expect("Invalid number. Try again");
    
    let sum: i32 = (1..=number).sum();

    println!("{}", sum);
    
}