//3. Crie um programa que calcule e exiba a média aritmética de três notas informadas pelo usuário.

use std::io;

fn main() {
    let grade1: f64 = loop {
        println!("TYPE FIRST NUMBER");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Invalid number");

        match input.trim().parse() {
            Ok(grade) => break grade,
            Err(_) => println!("Invalid input. Try again"),
        }
    };

    let grade2: f64 = loop {
        println!("TYPE SECOND NUMBER");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("INVALID NUMBER");

        match input.trim().parse() {
            Ok(grade) => break grade,
            Err(_) => println!("Invalid input. Try agaun"),
        }
    };

    let grade3: f64 = loop {
        println!("TYPE THIRD NUMBER");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("INVALID NUMBER");

        match input.trim().parse() {
            Ok(grade) => break grade,
            Err(_) => println!("Invalid input. Try agaun"),
        }
    };


    println!("{}", (grade1 + grade2 + grade3) / 3.00);

}
