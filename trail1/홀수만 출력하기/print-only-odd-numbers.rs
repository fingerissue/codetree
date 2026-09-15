use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();

    buffer.clear();
    
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    for token in tokens {
        let token : i8 = token.parse().unwrap();
        if token % 2 == 1 && token % 3 == 0 {
            println!("{}", token);
        }
    }
}