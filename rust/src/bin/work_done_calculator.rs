//13. Escreva um programa que calcule o trabalho realizado por uma força que atua sobre um objeto,
// utilizando a fórmula T = F * d, onde T é o trabalho, F é a força aplicada e d é a distância
// percorrida pelo objeto.

use std::io;

fn main() {
    let force: f64 = read_input("TYPE APPLIED FORCE");
    let distance = read_input("TYPE TRAVELLED DISTANCE");

    let work_done: f64 = force * distance;

    println!("WORK DONE IS:  {:.2}", work_done);


}


fn read_input(message: &str) -> f64 {
    loop {

        println!("{}" , message);
        let mut input: String = String::new();
        
        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(num) = input.trim().parse::<f64>() {
                dbg!(num);  
                return num;
            }
        }
        println!("Invalid number. Try again.");
    }
}