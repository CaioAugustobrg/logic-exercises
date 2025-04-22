use std::io;

fn main() {
    let mut a: String = String::new();
    
    io::stdin().
    read_line(&mut a)
    .expect("FAILED TO WRITE ON THAT VARIABLE A");
    
    let a: i32 = a.trim().parse().expect("FAILED TO PARSE A");

    let mut b: String = String::new();
    
    io::stdin()
    .read_line(&mut b)
    .expect("FAILED TO WRITE ON THAT VARIABLE B");

    let b: i32 = b.trim().parse().expect("FAILED TO PARSE B");

    println!("PROD = {}", a * b);
}