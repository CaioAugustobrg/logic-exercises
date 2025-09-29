use std::io;

// 63. Escreva um programa que leia um vetor de números inteiros e exiba a média dos elementos.

fn main() {
    println!("Enter the numbers separated by spaces:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|num_str| num_str.parse().ok())
        .collect();

    if numbers.is_empty() {
        println!("No valid numbers were entered.");
        return;
    }

    let sum: i32 = numbers.iter().sum();
    let average: f64 = sum as f64 / numbers.len() as f64;

    println!("Average: {:.2}", average);
}
