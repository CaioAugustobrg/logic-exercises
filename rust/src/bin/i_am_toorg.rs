use std::io;

fn main() {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("FAILED TO WRITE ON VARIABLE INPUT");

    let input: i32 = input
        .trim()
        .parse()
        .expect("FAILED TO PARSE VARIABLE INPUT");

    let mut count: i32 = 0;
    while count < input {
        let mut question: String = String::new();
        io::stdin()
            .read_line(&mut question)
            .expect("FAILED TO WRITE ON VARIABLE INPUT");
        println!("I am Toorg!");
        count = count + 1;
    }
}
