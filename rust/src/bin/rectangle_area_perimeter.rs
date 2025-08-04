use std::io;

fn main() {

    let width: f64 = read_input("Type the width of the rectangle");
    let length: f64 = read_input("Type the length of the rectangle");

    let perimeter: f64 = 2.0 * (width + length);
    let area: f64 = length * width;

    println!("The perimeter: {} and the area: {}", perimeter, area)

}

fn read_input(message: &str) -> f64 {

    loop {
        println!("{}",message);
        let mut input: String = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("INVALID INPUT.");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("INVALID NUMBER. TRY AGAIN")
        }
    }

}