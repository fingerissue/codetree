use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let a : i8 = tokens.next().unwrap().parse().unwrap();
    let b : i8 = tokens.next().unwrap().parse().unwrap();
    let c : i8 = tokens.next().unwrap().parse().unwrap();

    if a < b && b < c {
        println!("1");
    } else {
        println!("0");
    }
}