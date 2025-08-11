//15. Crie um programa que solicite ao usuário o valor do raio de uma esfera e calcule e exiba o seu volume
use std::{f64::consts::PI, io};

fn main() {
    let radius: f64 = read_input("Type the radius value");

    let volume: f64 = (4.0 / 3.0) * PI * radius.powi(3);

    println!("The sphere volume is: {:.2}", volume);
}



fn read_input(message: &str) -> f64 {
    loop {
        println!("{}", message);
        let mut input: String = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(num) = input.trim().parse::<f64>() {
                return num;
            }
        }
        println!("Invalid number. Try again");
    }
}