
// 12. Escreva um programa que calcule a energia cinética de um objeto em movimento,
// utilizando a fórmula E = (mv²) / 2, onde E é a energia cinética, m é a massa do objeto e v é a velocidade.

use std::io;

fn main() {
 
    let mass: f64 = read_input("TYPE MASS VALUE");
    let velocity: f64 = read_input("TYPE VELOCITY VALUE");
    let kinetic_energy: f64 = mass * velocity.powi(2) / 2.0;

    println!("Kinetic energy is: {}", kinetic_energy)
}

fn read_input(message: &str) -> f64 {
    loop {
        println!("{}", message);
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("INVALID NUMBER");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("INVALID NUMBER TRY AGAIN"),
        }
    }
}
