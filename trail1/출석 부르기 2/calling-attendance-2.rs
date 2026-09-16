use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    for token in tokens {
        let token : i8 = token.parse().unwrap();

        if token == 1 {
            println!("John");
        } else if token == 2 {
            println!("Tom");
        } else if token == 3 {
            println!("Paul");
        } else if token == 4 {
            println!("Sam");
        } else {
            println!("Vacancy");
            break;
        }
    }
}