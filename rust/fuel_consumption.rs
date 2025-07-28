use std::io;

fn main() {

    let mut total_distance: String = String::new();

    io::stdin()
    .read_line(&mut total_distance)
    .expect("FAILED TO WRITE ON VARIABLE TOTAL_DISTANCE");

    let total_distance: i32 = total_distance.trim().parse().expect("FAILED TO PARSE");

    let mut wasted_fuel: String = String::new();

    io::stdin()
    .read_line(&mut wasted_fuel)
    .expect("FAILED TO WRITE ON VARIABLE TOTAL_DISTANCE");

    let wasted_fuel: f64 = wasted_fuel.trim().parse().expect("FAILED TO PARSE");

    println!("{:.3} km/l", total_distance as f64 / wasted_fuel);

}