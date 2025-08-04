//Crie um programa que calcule e exiba o perímetro de um círculo, solicitando o raio ao usuário.
use std::{f64::consts::PI, io};
fn main() {

    let radius: f64 = read_input();
    let perimeter: f64 = 2.00 * (PI * radius);
    println!("{}", perimeter)
}

fn read_input() -> f64 {
    loop {
        let mut input: String = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("INVALID NUMBER.");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Try again!")
        }


    }
}
