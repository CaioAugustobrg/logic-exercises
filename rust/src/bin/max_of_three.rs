use std::io;

fn main() {
    let first_number: i32 = read_input("Type first number");
    let second_number: i32 = read_input("Type second number");
    let third_number: i32 = read_input("Type third number");

    if first_number > second_number && first_number > third_number {
        println!("First number is the greater {}", first_number);
    } else if second_number > first_number && second_number > third_number {
        println!("Second number is the greater {}", second_number);
    } else if third_number > first_number && third_number > second_number {
        println!("Third number is the greater {}", third_number);
    } else {
        println!("Numbers are equal");
    }
}

fn read_input(message: &str) -> i32 {
    loop {
        println!("{}", message);
        let mut input: String = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(num) = input.trim().parse::<i32>() {
                return num;
            }
        }
        println!("Invalid number. Try again");
    }
}
