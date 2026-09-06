use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let a : i8 = tokens.next().unwrap().parse().unwrap();
    let b : i8 = tokens.next().unwrap().parse().unwrap();

    if a < 90 {
        println!("0");
    } else if b >= 95 {
        println!("100000");
    } else if b >= 90 {
        println!("50000");
    } else {
        println!("0");
    }
}