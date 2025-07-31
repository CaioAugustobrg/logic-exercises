//Escreva um programa que calcule a média aritmética de dois números.

use std::io;

fn main() {

    println!("TYPE FIRST NUMBER");
    let mut x: String = String::new();
    io::stdin().read_line(&mut x).expect("FAILED TO WIRTE ON VARIABLE X");
    let x: f64 = x.trim().parse().expect("INVALID FIRST NUMBER");
    
    println!("TYPE SECOND NUMBER");
    let mut y: String = String::new();
    io::stdin().read_line(&mut y).expect("FAILED TO WIRTE ON VARIABLE Y");
    let y: f64 = y.trim().parse().expect("INVALID SECOND NUMBER");

    let avarage: f64 = (x + y) / 2.0;

    println!("AVARAGE: {}", avarage);

}