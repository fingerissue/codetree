use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let a : i16 = buffer.trim().parse().unwrap();

    if a == 5 {
        println!("A");
    }

    if a % 2 == 0 {
        println!("B");
    }
}