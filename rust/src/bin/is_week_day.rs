//31. Faça um programa que solicite o nome de um dia da semana e exiba se é um dia útil (segunda a sexta-feira)
//  ou um dia de fim de semana (sábado e domingo).
use std::io;

fn main() {
    let day = read_input("Enter the day of the week:");
    is_week_day(&day);
}

fn is_week_day(day: &str) {
    let day = day.to_lowercase();

    if day == "saturday" || day == "sunday" {
        println!("It's the weekend");
    } else if day == "monday"
           || day == "tuesday"
           || day == "wednesday"
           || day == "thursday"
           || day == "friday" {
        println!("It's a weekday");
    } else {
        println!("Invalid input: {}", day);
    }
}

fn read_input(message: &str) -> String {
    println!("{}", message);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}
