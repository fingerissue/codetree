use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let mut b : i8 = tokens.next().unwrap().parse().unwrap();
    let a : i8 = tokens.next().unwrap().parse().unwrap();

    while a <= b {
        print!("{} ", b);
        b -= 2;
    }
}