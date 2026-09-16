use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let mut sum : i16 = 0;
    let mut cnt : i16 = 0;
    
    for token in tokens {
        let token : i16 = token.parse().unwrap();
        if token >= 0 && token <= 200 {
            sum += token;
            cnt += 1;
        }
    }

    let avg : f32 = sum as f32 / cnt as f32;

    println!("{} {:.1}", sum, avg);
}