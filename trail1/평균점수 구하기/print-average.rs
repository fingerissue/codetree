use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let mut sum = 0.0;

    for token in tokens {
        let token : f64 = token.parse().unwrap();
        sum += token;
    }

    let avg = sum / 8 as f64;

    println!("{avg:.1}");
}