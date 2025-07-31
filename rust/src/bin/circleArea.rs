use std::io;

fn main() {
    let mut raio: String = String::new();
    io::stdin()
        .read_line(&mut raio)
        .expect("FAILED TO READ INPUT");

    let raio: f32 = raio.trim().parse().expect("FAILED TO PARSE FLOAT");

    let n: f32 = 3.14159;

    let area: f32 = n * (raio * raio);

    println!("A={}", area);
}
