//40. Crie um programa que solicite ao usuário um número e exiba
//a tabuada desse número utilizando um laço de repetição.
use std::io;

fn main() {
    let number: i32 = read_input("Please, type a number");
    for i in 1..=10 {
        println!("{} times {} is {}", number, i, number * i);
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
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Try again"),
        }
    }
}
