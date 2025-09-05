// 34. Crie um programa que solicite a idade de uma pessoa e exiba se ela é criança (0-12 anos),
//adolescente (13-17 anos), adulto (18-59 anos) ou idoso (60 anos ou mais).
use std::io;

fn main() {
    let age: i32 = read_input("Please type your age");
    verify_age(age);
}

fn read_input(message: &str) -> i32 {
    println!("{}", message);
    loop {
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid number. Try again"),
        }
    }
}

fn verify_age(age: i32) -> () {
    if age >= 0 && age < 12 {
        println!("You typed: {}. You are a child", age)
    } else if age >= 13 && age <= 17 {
        println!("You typed: {}. You are a teenager", age)
    } else if age >= 18 && age <= 59 {
        println!("You typed: {}. You are an adult", age)
    } else {
        println!("You typed: {}. You are an eldery", age)
    }
}
