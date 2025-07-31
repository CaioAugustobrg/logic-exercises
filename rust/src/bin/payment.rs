use std::io;

fn main() {
    // Escreva a sua solução aqui
    // Code your solution here
    // Escriba su solución aquí
    let mut employee_number: String = String::new();
    
    io::stdin()
    .read_line(&mut employee_number)
    .expect("FAILED TO WRITE ON VARIABLE EMPLOYEENUMBER");
    
    let employee_number: i32 = employee_number.trim().parse().expect("FAILED TO PARSE employeeNumber");
    
    let mut labour_hours: String = String::new();
    
    io::stdin()
    .read_line(&mut labour_hours)
    .expect("FAILED TO WRITE ON VARIABLE EMPLOYEENUMBER");
    
    let labour_hours: f64 = labour_hours.trim().parse().expect("FAILED TO PARSE labourHours");
    
    let mut hourly_wage: String = String::new();
    
    io::stdin()
    .read_line(&mut hourly_wage)
    .expect("FAILED TO WRITE ON VARIABLE EMPLOYEENUMBER");
    
    let hourly_wage: f64 = hourly_wage.trim().parse().expect("FAILED TO PARSE labourHours");
    
    println!("NUMBER = {}", employee_number);
    println!("SALARY = U$ {:.2}", labour_hours * hourly_wage);
    
}