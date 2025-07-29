use std::io;

fn main() {
    let price_of_tables = [(1, 4.00), (2, 4.50), (3, 5.00), (4, 2.00), (5, 1.50)];

    let mut input: String = String::new();

    io::stdin()
    .read_line(&mut input)
    .unwrap();

    let values: Vec<i32> = 
    input
    .split_whitespace()
    .map(|x| x.parse().unwrap())
    .collect();

    let number = values[0];
    let item = values[1];

    let price = price_of_tables
    .iter()
    .find(|(c, _)| *c == number)
    .unwrap().1;


    println!("{}" ,number);
    println!("{}" ,item);
    println!("Total: R$ {:.2}" , price * item as f64 );
    

}
