use std::io;

fn main() {
    println!("Type a number");
    let mut input: String = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    let input: i32 = input.trim().parse().expect("Invalid input. try again");

    if input % 2 == 0 {
        println!("The number {} is even", input);
    } else {
        println!("The number {} is odd", input);
    }
}