fn main () {
    let text: String = String::from("Rust");
    let _text1: String = text.clone();
    println!("still owner: {}", text)
}