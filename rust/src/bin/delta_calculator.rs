use std::io;
// 8. Escreva um programa que calcule o delta de uma equação de segundo grau (Δ = b² - 4ac).
fn main() {

    let a: f64 = read_input();
    let b: f64 = read_input();
    let c: f64 = read_input();

    let delta : f64 = b * b - 4.0 * a * c;

    println!("The delta is: {}", delta)


}


fn read_input() -> f64 {
    loop {
        let mut input: String = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("INVALID NUMBER");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("INVALID NUMBER. TRY AGAIN")
        }
    }
}