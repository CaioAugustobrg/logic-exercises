use std::io;
//27. Faça um programa que leia três números e informe se eles podem ser os lados de um triângulo
// (a soma de dois lados deve ser sempre maior que o terceiro lado).

fn main() {
    let first_number: i32 = read_input("Insert the first number");
    let second_number: i32 = read_input("Insert the second number");
    let third_number: i32 = read_input("Insert the third number");

    let condition1 = first_number + second_number > third_number;
    let condition2 = first_number + third_number > second_number;
    let condition3 = second_number + third_number > first_number;

    if condition1 && condition2 && condition3 {
        println!("The numbers can be a triangle");
    } else {
        println!("The numbers can not be a triangle");
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