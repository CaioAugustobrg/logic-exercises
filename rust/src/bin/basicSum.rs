use std::io;

fn main() {
   let mut a: String = String::new();

    io::stdin()
        .read_line(&mut a)
        .expect("FAILED");
        
    let mut b: String = String::new();
    
    io::stdin()
        .read_line(&mut b)
        .expect("FAILED");

   
   let a: i32 = a.trim().parse().expect("FAILED");
   let b: i32 = b.trim().parse().expect("FAILED"); 
   
   let x: i32 = a + b;
   
   println!("X = {}", x)
}