use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let mut a : i8 = tokens.next().unwrap().parse().unwrap();
    let n : i8 = tokens.next().unwrap().parse().unwrap();

    for _ in 0..n {
        a += n;
        println!("{}", a);
    }
}