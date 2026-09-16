use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();

    buffer.clear();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let mut sum : i32 = 0;
    
    for token in tokens {
        let token : i32 = token.parse().unwrap();
        sum += token;
    }

    let avg : f32 = sum as f32 / n as f32;

    println!("{} {:.1}", sum, avg);
}