use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let c : char = tokens.next().unwrap().parse().unwrap();
    let n : i8 = tokens.next().unwrap().parse().unwrap();

    if c == 'A' {
        for i in 1..=n {
            print!("{} ", i);
        }
    } else if c == 'D' {
        for i in (1..=n).rev() {
            print!("{} ", i);
        }
    }
}