use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let a_age : i8 = tokens.next().unwrap().parse().unwrap();
    let a_sex : &str = tokens.next().unwrap();
    let b_age : i8 = tokens.next().unwrap().parse().unwrap();
    let b_sex : &str = tokens.next().unwrap();

    if a_age >= 19 && a_sex == "M" || b_age >= 19 && b_sex == "M" {
        println!("1");
    } else {
        println!("0");
    }
}