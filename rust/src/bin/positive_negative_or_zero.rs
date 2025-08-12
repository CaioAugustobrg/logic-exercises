//Faça um programa que leia um número e informe se ele é positivo, negativo ou zero.
use std::io;

fn main() {
   let number: i32 = read_input("Type a number");

   if number > 0 {
        println!("The number {} is positive", number);

   } else if number < 0 {
    println!("The number {} is negative", number)
   } else {
    println!("The number {} is zero", number)
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
    println!("Invalid number. Try again");
}
}