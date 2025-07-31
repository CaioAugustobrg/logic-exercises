//Escreva um programa que solicite ao usuário dois números e exiba a soma, subtração, multiplicação e divisão entre eles.

use std::io;

fn main() {

    println!("Type first number");
    let mut x: String = String::new();

    io::stdin()
    .read_line(&mut x)
    .expect("FAILED TO WRITE ON VARIABLE X");

    let x: f64 = x.trim().parse().expect("FIRST NUMBER INVALID");

    println!("Type second number");
    let mut y: String = String::new();

    io::stdin()
    .read_line(&mut y)
    .expect("FAILED TO WRITE ON VARIABLE Y");

    let y: f64 = y.trim().parse().expect("SECOND NUMBER INVALID");


    println!("Sum: {}", x + y);
    println!("Subtraction: {}", x - y);
    println!("Multiplication: {}", x * y);
    println!("Division: {}", x / y);


}