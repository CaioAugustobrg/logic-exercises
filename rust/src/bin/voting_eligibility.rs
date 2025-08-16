use std::io;

// Faça um programa que leia o ano de nascimento de uma pessoa e informe se ela está apta a votar (idade maior ou igual a 16 anos).

fn main() {
    let birth_year: i32 = read_input("Type your birth year");
    let currently_year: i32 = 2025;

    let age: i32 = currently_year - birth_year;

    if age >= 16 {
        println!("You can vote");
    } else {
        println!("You can not vote");
    }
}

fn read_input(message: &str) -> i32 {
    loop {
        println!("{}", message);
        let mut input: String = String::new();

        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(num) = input.trim().parse::<i32>() {
                return num;
            }
        }
        println!("Invalid age. Try again");
    }
}
