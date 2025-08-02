//3. Crie um programa que calcule e exiba a média aritmética de três notas informadas pelo usuário.

use std::io;

fn main() {
    
    println!("TYPE FIRST NUMBER");
    let grade1: f64 = read_grade();

    println!("TYPE SECOND NUMBER");
    let grade2: f64 = read_grade();


    println!("TYPE THIRD NUMBER");
    let grade3: f64 = read_grade();
    
    println!("{}", (grade1 + grade2 + grade3) / 3.00);

}


fn read_grade() -> f64 {
    loop {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Invalid number");
        match input.trim().parse() {
            Ok(grade) => break grade,
            Err(_) => println!("Invalid input. Try again"),
        }
    }
}