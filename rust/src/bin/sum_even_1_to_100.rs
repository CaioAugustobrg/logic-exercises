// 43. Faça um programa que calcule e exiba a soma dos números pares de 1 a 100

fn main() {
    let sum: i32 = (2..=100).step_by(2).sum();
    println!("{}", sum);
}
