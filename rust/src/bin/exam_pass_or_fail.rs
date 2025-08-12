//Faça um programa que leia as notas de duas provas e informe se o aluno
//foi aprovado (nota maior ou igual a 6) ou reprovado (nota menor que 6) em cada uma das provas
use std::io;

fn main() {
    let first_grade: f64 = read_input("Type the first grade");
    let second_grade: f64 = read_input("Type the second grade");
    is_approved("First grade", first_grade);
    is_approved("Second grade", second_grade);
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
        println!("Invalid input. try again");
    }
}

fn is_approved(label: &str,grade: f64) -> () {
    if grade >= 6.0 {
        println!("{}: approved", label);
    } else {
        println!("{}: reproved", label);
    }
}