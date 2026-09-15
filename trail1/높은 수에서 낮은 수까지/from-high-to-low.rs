use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let a : i8 = tokens.next().unwrap().parse().unwrap();
    let b : i8 = tokens.next().unwrap().parse().unwrap();

    if a < b {
        for i in (a..=b).rev() {
            print!("{} ", i);
        }
    } else {
        for i in (b..=a).rev() {
            print!("{} ", i);
        }
    }
}