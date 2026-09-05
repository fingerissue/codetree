use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let score : i8 = buffer.trim().parse().unwrap();

    println!("{}", if score == 100 { "pass" } else { "failure" });
}