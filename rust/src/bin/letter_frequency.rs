use std::collections::HashMap;
use std::io;
// Neste problema estamos interessados na frequência das letras em uma dada linha de texto.
fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).unwrap();
    let n: usize = n_str.trim().parse().unwrap();

    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let mut freq: HashMap<char, usize> = HashMap::new();

        for ch in line.chars() {
            if ch.is_ascii_alphabetic() {
                let ch = ch.to_ascii_lowercase();
                *freq.entry(ch).or_insert(0) += 1;
            }
        }

        let max_freq = freq.values().copied().max().unwrap_or(0);

        let mut letters: Vec<char> = freq
            .iter()
            .filter(|(_, &count)| count == max_freq)
            .map(|(&ch, _)| ch)
            .collect();

        letters.sort();

        for ch in letters {
            print!("{}", ch);
        }
        println!();
    }
}
