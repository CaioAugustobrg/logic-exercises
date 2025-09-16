//50. Escreva um programa que leia números do usuário até que seja
// digitado um número negativo, e exiba a soma dos números positivos.
use std::io;

fn main() {
    let mut sum: i32 = 0;
    loop {
       let number: i32 = get_input("Please, type a number");
       println!("{}", number);
       if number < 0 {
           break
       } else {
           sum = sum + number;
       }
    }
    println!("{}", sum);
}

fn get_input(message: &str) -> i32 {
    loop {
        println!("{}", message);
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