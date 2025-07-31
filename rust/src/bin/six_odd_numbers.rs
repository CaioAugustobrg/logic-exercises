use std::io;

fn main() {
    let mut input: String = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to write on that variable");

    let input: i32 = input.trim().parse().expect("FAILED TO PARSE INPUT");
    let mut count: i32 = 1;
    

    if input % 2 == 0 {   
        while count < 12 {
            println!("{}", input + count);
            count = count + 2;
        }
    } else {
        while count < 13 {
            println!("{}", (input + count) - 1);
            count = count + 2;
        }
    }
}