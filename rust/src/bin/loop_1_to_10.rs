//36. Escreva um programa que exiba os números de 1 a 10 utilizando um laço de repetição

fn main() {
    // let mut counter: i32 = 1;
    // while counter <= 10 {
    //     println!("{}", counter);
    //     counter = counter + 1;
    // }

    //    let mut counter: i32 = 1;
    //    loop {
    //        if counter > 10 {
    //            break;
    //        } else {
    //            println!("{}", counter);
    //            counter = counter + 1;
    //            continue;
    //        }
    //    }
    
    for number in 1..=10 {
        println!("{}", number)
    }
}
