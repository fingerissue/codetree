use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let a : i8 = tokens.next().unwrap().parse().unwrap();
    let b : i8 = tokens.next().unwrap().parse().unwrap();
    let c : i8 = tokens.next().unwrap().parse().unwrap();
    let d : i8 = tokens.next().unwrap().parse().unwrap();
    let e : i8 = tokens.next().unwrap().parse().unwrap();

    println!("{}", if a > b { "1" } else { "0" });
    println!("{}", if a > c { "1" } else { "0" });
    println!("{}", if a > d { "1" } else { "0" });
    println!("{}", if a > e { "1" } else { "0" });
}