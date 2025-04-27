use std::io;

fn main () {
    let mut firstNumber: String = String::new();

    io::stdin()
    .read_line(&mut firstNumber)
    .expect("FAILED TO WRITE ON VARIABLE FIRSTNUMBER");

    let mut secondNumber: String = String::new();

    io::stdin()
    .read_line(&mut secondNumber)
    .expect("FAILED TO WRITE ON VARIABLE secondNumber");
    
    let firstNumber: i32 = firstNumber.trim().parse().expect("FAILED TO PARSE FIRSTNUMBER");
    let secondNumber: i32 = secondNumber.trim().parse().expect("FAILED TO PARSE SECONDNUMBER");

    
    let addition: i32 = firstNumber + secondNumber;
    let subtraction: i32 = firstNumber - secondNumber;
    let multiplication: i32 = firstNumber * secondNumber;
    let division: i32 = firstNumber / secondNumber;

    println!("Adition is {}", addition);
    println!("Subtraction is {}", subtraction);
    println!("Multiplication is {}", multiplication);
    println!("Division is {}", division );
    


}