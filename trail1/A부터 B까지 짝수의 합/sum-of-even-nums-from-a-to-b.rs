use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let a : i16 = tokens.next().unwrap().parse().unwrap();
    let b : i16 = tokens.next().unwrap().parse().unwrap();

    let mut sum : i16 = 0;
    
    for i in a..=b {
        if i % 2 == 0 {
            sum += i;
        }
    }

    println!("{}", sum);
}