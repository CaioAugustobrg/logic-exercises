use std::io;

fn main() {
    println!("Type first number");
    let first_value: f64 = read_input();

    println!("Type second number");
    let second_value: f64 = read_input();
    
    println!("Type third number");
    let third_value: f64 = read_input();


    let geometric_mean = (first_value * second_value * third_value).powf(1.0 / 3.0);

    println!("{}", geometric_mean);
    
}

fn read_input() -> f64 {
 loop {
    let mut input: String = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("FAILED TO WRITE ON VARIABLE INPUT");

    match input.trim().parse() {
        Ok(value) => break value,
        Err(_) => println!("INVALID NUMBER")
    }

 }
}