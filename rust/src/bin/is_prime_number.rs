// //45. Escreva um programa que solicite ao usuário um número N e diga se o mesmo é primo ou não.

use std::io;

fn main() {
    println!("Please, type a number:");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read input");

    let number: i32 = number.trim().parse().expect("Invalid number. Try again");

    if is_prime(number) {
        println!("Number {} is a prime number", number);
    } else {
        println!("Number {} is NOT a prime number", number);
    }
}

fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    let limit = (n as f64).sqrt() as i32;
    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}
