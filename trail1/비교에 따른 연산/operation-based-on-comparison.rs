use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();
    
    let a : i16 = tokens.next().unwrap().parse().unwrap();
    let b : i16 = tokens.next().unwrap().parse().unwrap();

    if a > b {
        println!("{}", a * b);
    } else {
        println!("{}", b / a);
    }
}