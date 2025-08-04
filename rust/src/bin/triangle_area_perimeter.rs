//10. Escreva um programa que calcule o perímetro e a área de um triângulo,
//utilizando as fórmulas P = a + b + c e A = (b * h) / 2, onde a, b e c são os lados do triângulo e h é a altura relativa ao lado b.
use std::io;
fn main() {


    let a: f64 = read_input("TYPE A NUMBER TO BE SIDE A");
    let b: f64 = read_input("TYPE A NUMBER TO BE SIDE B");
    let c: f64 = read_input("TYPE A NUMBER TO BE SIDE C");
    let height: f64 = read_input("TYPE A NUMBER TO BE THE HEIGHT");

    let perimeter: f64 = a + b + c;
    let area: f64 = (b * height) / 2.0;

    println!("The perimeter of triangle is {} and the area is {}", perimeter, area)


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