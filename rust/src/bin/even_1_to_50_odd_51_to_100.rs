// 39. Escreva um programa que exiba os números pares de 1 a 50
//e os números ímpares de 51 a 100 utilizando um laço de repetição.

fn main() {
    // let mut counter: i32 = 1;
    //while counter <= 100 {
    //        if counter % 2 == 0 && counter <= 50 {
    //            println!("{}", counter)
    //       } else if counter % 2 != 0 && counter >= 51 {
    //            println!("{}", counter)
    //        }
    //        counter = counter + 1;
    //}


    //--------------------------------

    // let mut counter: i32 = 1;
    //loop {
    //    if counter % 2 == 0 && counter <= 50 {
    //        println!("{}", counter);
    //    } else if counter % 2 != 0 && counter >= 51 && counter <= 100 {
    //        println!("{}", counter);
    //    }
    //    counter += 1;
    //    if counter > 100 {
    //        break;
    //    }
    //}
    

    //--------------------------------

    
    for number in (2..=50).step_by(2) {
        println!("{}", number);
    }
    
    for number in (51..=100).step_by(2) {
        println!("{}", number);
    }
}
