use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let mut cnt : i8 = 0;
    
    for token in tokens {
        let token : i16 = token.parse().unwrap();
        if token % 2 == 1 {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}