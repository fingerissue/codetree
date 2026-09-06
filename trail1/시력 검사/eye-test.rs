use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();
    
    let a : f32 = tokens.next().unwrap().parse().unwrap();
    let b : f32 = tokens.next().unwrap().parse().unwrap();

    if a >= 1.0 && b >= 1.0 {
        println!("High");
    } else if a >= 0.5 && b >= 0.5 {
        println!("Middle");
    } else {
        println!("Low");
    }
}