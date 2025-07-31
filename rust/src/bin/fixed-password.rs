use std::io;

fn main() {
    let correct_password: &str = "2002";

    loop {
        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to write on variable input");

        if input.trim() == correct_password {
            println!("Acesso Permitido");
            break;
        }
        println!("Senha Invalida");
    }
}
