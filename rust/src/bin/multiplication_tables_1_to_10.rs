// 41. Escreva um programa que imprima na tela a tabuada de todos os números de 1 a 10

fn main() {
    for number in 1..=10 {
        println!("Multiplication table for {}:", number);
        for i in 1..=10 {
            println!("{} x {} = {}", number, i, number * i);
        }
        println!();
    }
}
