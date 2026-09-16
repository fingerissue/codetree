use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let mut satisfied = true;

    for token in tokens {
        let token : i8 = token.parse().unwrap();
        if token % 3 != 0 {
            satisfied = false;
        }
    }

    println!("{}", satisfied as u8);
}