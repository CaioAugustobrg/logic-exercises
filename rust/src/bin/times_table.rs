use std::io;

fn main() {
    // Escreva a sua solução aqui
    // Code your solution here
    // Escriba su solución aquí

        
    let mut input: String = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Faield to write on variable input");

    let input: i32 = input.trim().parse().expect("Failed to aprse variable input");

    let mut count: i32 = 1;
    loop {
        if count > 10 {
            break;
        }
        let result = count * input;
        println!("{} x {} = {}", input, count, result );
        count += 1;
    }
    
}