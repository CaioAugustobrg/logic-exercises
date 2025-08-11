use std::io;

//Escreva um programa que leia a posição x e y de dois pontos
// no plano cartesiano, e calcule a distância entre ambos.

fn main () {

    let input_x1: f64 = read_input("Type x1 value");
    let input_y1: f64 = read_input("Type y1 value");
    let input_x2: f64 = read_input("Type x2 value");
    let input_y2: f64 = read_input("Type y2 value");


    let distance = ((input_x2 - input_x1).powi(2) + (input_y2 - input_y1).powi(2)).sqrt();

    println!("The distance betwetn two points is: {:.2}", distance);


}


fn read_input(message: &str) -> f64 {

    loop {
        println!("{}", message);
        let mut input: String = String::new();

        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(num) = input.trim().parse::<f64>() {
                return num;
            }
        }
        println!("Invalid number. Try again.");
    }

}