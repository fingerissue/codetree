use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let a : i16 = buffer.trim().parse().unwrap();

    if a >= 113 {
        println!("1");
    } else {
        println!("0");
    }
}