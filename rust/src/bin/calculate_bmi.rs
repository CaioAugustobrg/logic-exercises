//Escreva um programa que calcule o IMC de um indivíduo, utilizando a fórmula IMC = peso / altura²

use std::io;

fn main() {
     

    println!("Enter your height in meters");
    let height: f64 = read_input();


    println!("Enter your weight in kilograms");
    let weight: f64 = read_input();

    let bmi: f64 = weight / (height * height);

    println!("{:.1}", bmi);
}

fn read_input() -> f64 {

    loop {
        let mut input: String = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("FAILED TO READ LINE");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("INVALID NUMBER, MUST BE FLOATING POINT")
        }

    }        

}