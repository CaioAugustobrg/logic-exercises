//54. Escreva um programa que solicite ao usuário uma frase e exiba a quantidade de vogais na frase.
use std::io;

fn main() {
    println!("Please, write a phrase:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get input.");

    let vowels = "aeiouAEIOU";
    let mut count_vowels = 0;

    for c in input.chars() {
        if vowels.contains(c) {
            count_vowels += 1;
        }
    }

    println!("There are {} vowels in: {}", count_vowels, input.trim());
}
