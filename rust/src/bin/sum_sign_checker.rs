// 24. Crie um programa que leia três números e verifique se a soma deles é positiva, negativa ou igual a zero
use std::io;


fn main() {
    
    let first_number = read_input();
    let second_number = read_input();
    let third_number = read_input();

    let sum: i32 = first_number + second_number + third_number;

    if sum > 0 {
        println!("The result of the sum is a positive number {}", sum);
    } else if sum < 0 {
        println!("The result of the sum is a negative number {}", sum);
    } else {
        println!("The result is zero");
    }
}

fn read_input() -> i32 {
    println!("Type a number");
    loop {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Invalid input");

        match input.trim().parse::<i32>() {
            Ok(num) => break num,
            Err(_) => println!("Invalid number. Try again")
        }
    }
}
