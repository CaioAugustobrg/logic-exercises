use std::io;

//62. Faça um programa que leia um vetor de números inteiros e exiba o maior elemento presente no vetor.

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
    
    let mut largest_number = numbers[0];
    
    for num in numbers {
        if num > largest_number {
            largest_number = num;
        }
    }
    
    println!("The largest number is: {}", largest_number);
}
