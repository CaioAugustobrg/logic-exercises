use std::io;

fn main() {
    
    let mut a: String = String::new();
    io::stdin()
    .read_line(&mut a)
    .expect("FAILED TO WRITE ON VARIABLE A");
    let a: f64 = a.trim().parse().expect("FAILED TO PARSE VARIABLE A");
    
    
    let mut b: String = String::new();
    io::stdin()
    .read_line(&mut b)
    .expect("FAILED TO WRITE ON VARIABLE B");
    let b: f64 = b.trim().parse().expect("FAILED TO PARSE VARIABLE B");
    
    
    let mut c: String = String::new();
    io::stdin()
    .read_line(&mut c)
    .expect("FAILED TO WRITE ON VARIABLE C");
    let c: f64 = c.trim().parse().expect("FAILED TO PARSE VARIABLE C");
    
    println!("MEDIA = {:.1}", ((a * 2.0)  + (b * 3.0) + (c * 5.0)) / 10.0);
}