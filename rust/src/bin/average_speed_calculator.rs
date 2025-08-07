//11. Escreva um programa que calcule a velocidade média de um objeto, utilizando a fórmula v = Δs/Δt, 
// onde v é a velocidade média, Δs é a variação de espaço e Δt é a variação de tempo

use std::io;

fn main() {
    let displacement: f64 = read_input("TYPE DISPLACEMENT NUMBER");
    let time_interval: f64 = read_input("TYPE TIME INTERVAL NUMBER");

    let avarage_speed: f64 = displacement / time_interval;

    println!("The average speed is: {}", avarage_speed);

}


fn read_input(message: &str) -> f64 {
    loop {
        println!("{}", message);
        let mut input: String = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("INVALID NUMBER");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("INVALID NUMBER TRY AGAIN")
        }
    }
}