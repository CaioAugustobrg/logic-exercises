use std::io;
//Faça um programa que leia a idade de três pessoas e quantas delas é maior de idade (idade maior ou igual a 18).

fn main() {
    let side_a = read_input("Insert the first side:");
    let side_b = read_input("Insert the second side:");
    let side_c = read_input("Insert the third side:");

    if side_a + side_b > side_c &&
       side_a + side_c > side_b &&
       side_b + side_c > side_a {
        println!("These sides form a valid triangle.");
    } else {
        println!("These sides do not form a valid triangle.");
    }
}

fn read_input(message: &str) -> i32 {
    loop {
        println!("{}", message);
        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(num) = input.trim().parse::<i32>() {
                return num;
            }
        }
        println!("Invalid number. Try again.");
    }
}
