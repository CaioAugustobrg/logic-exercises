//17. Faça um programa que leia dois números e informe qual é o maior.

use std::io;

fn main() {
    let first_number: i32 = read_input("Type first number");
    let second_number: i32 = read_input("Type second number");

    find_max_number(first_number, second_number);
}


fn read_input(message: &str) -> i32 {
    println!("{}", message);
    loop {
        let mut input: String = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(num) = input.trim().parse::<i32>() {
                return num;
            }
        }
        println!("Invalid number. Try again");
    }
}

fn find_max_number(first_number: i32, second_number: i32) {
    if first_number > second_number {
        println!("The first number {} is greater.", first_number);
    } else if second_number > first_number {
        println!("The second number {} is greater.", second_number);
    } else {
        println!("Both numbers are equal.");
    }
}