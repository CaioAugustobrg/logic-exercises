// 29. Faça um programa que leia a idade de uma pessoa e informe se ela não 
//está apta a votar (idade inferior a 16 anos), se está apta a votar, porém
// não é obrigada (16, 17 anos, ou idade igual ou superior a 70 anos), ou se é obrigada (18 a 69 anos).


use std::io;

fn main() {
    let age: i32 = read_input("Type your age");

    if age < 16  {
        println!("You can not vote");
    } else if age >= 16 && age <= 17 || age >= 70{
        println!("You can vote");
    } else {
        println!("You vote is mandatory");
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
