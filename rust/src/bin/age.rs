//16. Faça um programa que solicite a idade de uma pessoa e exiba se ela é maior de idade ou não.
use std::io;

fn main() {
    let mut age: String = String::new();
    io::stdin()
    .read_line(&mut age)
    .expect("Invalid input");

    let age: i32 = age.trim().parse().expect("Invalid number. try again");

    if age >= 18 {
        println!("You are an adult");
    } else {
        println!("You are not an adult");
    }
}