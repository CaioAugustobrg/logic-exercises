// Original code (with ownership error)

/*
fn print_name(name: String) {
    println!("name: {}", name);
}

fn main() {
    let my_name = String::from("Alice");
    print_name(my_name);
    println!("NAME LENGTH: {}", my_name.len());
}
*/

// -----------------------------------------------------

// Corrected code (passing a reference)

fn print_name(name: &String) {
    println!("name: {}", name);
}

fn main() {
    let my_name = String::from("Alice");
    print_name(&my_name);
    println!("NAME LENGTH: {}", my_name.len());
}
