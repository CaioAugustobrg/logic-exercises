use std::io;

fn main() {
    // Escreva a sua solução aqui
    // Code your solution here
    // Escriba su solución aquí

    let mut seller_name: String = String::new();
    io::stdin()
    .read_line(&mut seller_name)
    .expect("FAILED TO WRITE ON VARIABLE SELLERNAME");
    
    let mut a: String = String::new();
    io::stdin()
    .read_line(&mut a)
    .expect("FAILED TO WRITE ON VARIABLE A");
    let a: f64 = a.trim().parse().expect("FAILED TO PARSE A");
    
    let mut b: String = String::new();
    io::stdin()
    .read_line(&mut b)
    .expect("FAILED TO WRITE ON VARIABLE B");
    let b: f64 = b.trim().parse().expect("FAILED TO PARSE B");
    
    let final_payment: f64 = a + (b * 0.15);
    println!("TOTAL = R$ {:.2}", final_payment);
}