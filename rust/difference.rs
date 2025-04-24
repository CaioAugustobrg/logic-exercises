use std::io;

fn main() {
    // Escreva a sua solução aqui
    // Code your solution here
    // Escriba su solución aquí

    let mut a: String = String::new();
    
    io::stdin()
    .read_line(&mut a)
    .expect("FAILED TO WRITE ON VARIABLE A");
    
    let a: i32 = a.trim().parse().expect("FAILED TO PARSE A");
        
    let mut b: String = String::new();
    
    io::stdin()
    .read_line(&mut b)
    .expect("FAILED TO WRITE ON VARIABLE B");
    
    let b: i32 = b.trim().parse().expect("FAILED TO PARSE B");
    
    let mut c: String = String::new();
    
    io::stdin()
    .read_line(&mut c)
    .expect("FAILED TO WRITE ON VARIABLE C");
    
    let c: i32 = c.trim().parse().expect("FAILED TO PARSE C");
    
    let mut d: String = String::new();
    
    io::stdin()
    .read_line(&mut d)
    .expect("FAILED TO WRITE ON VARIABLE D");
    
    let d: i32 = d.trim().parse().expect("FAILED TO PARSE D");
    
    let difference: i32 = (a * b) - (c * d);
    
    println!("DIFERENCA = {}", difference);
    
    
    
    
    
}