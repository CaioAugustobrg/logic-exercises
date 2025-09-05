// 38. Escreva um programa que exiba todos os números pares de 1 a 100.

fn main() {
    // let mut counter: i32 = 1;
    // while counter <= 100 {
    //    if counter % 2 == 0 {
    //        println!("{}", counter);
    //    }
    //    counter = counter + 1;
    //

    // let mut counter: i32 = 2;
    // loop {
    //    if counter > 100 {
    //        break;
    //    } else {
    //        println!("{}", counter);
    //    }
    //    counter = counter + 2;
//  }
    
    for number in (2..=100).step_by(2) {
        println!("{}", number);
    }
}
