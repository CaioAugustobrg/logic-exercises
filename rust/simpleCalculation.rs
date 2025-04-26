use std::io;

fn main() {
    let mut first_line = String::new();
    let mut second_line = String::new();
    
    io::stdin()
        .read_line(&mut first_line)
        .expect("Failed to read first line");

    io::stdin()
        .read_line(&mut second_line)
        .expect("Failed to read second line");

    let first_parts: Vec<&str> = first_line.trim().split_whitespace().collect();
    let _first_item_code: i32 = first_parts[0].parse().expect("Failed to parse first item code");
    let first_quantity: i32 = first_parts[1].parse().expect("Failed to parse first quantity");
    let first_price: f64 = first_parts[2].parse().expect("Failed to parse first price");

    let second_parts: Vec<&str> = second_line.trim().split_whitespace().collect();
    let _second_item_code: i32 = second_parts[0].parse().expect("Failed to parse second item code");
    let second_quantity: i32 = second_parts[1].parse().expect("Failed to parse second quantity");
    let second_price: f64 = second_parts[2].parse().expect("Failed to parse second price");

    let total_first = (first_quantity as f64) * first_price;
    let total_second = (second_quantity as f64) * second_price;

    let total_amount = total_first + total_second;

    println!("VALOR A PAGAR: R$ {:.2}", total_amount);
}
