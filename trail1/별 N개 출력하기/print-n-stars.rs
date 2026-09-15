use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut n : i8 = buffer.trim().parse().unwrap();

    while n > 0 {
        println!("*");
        n -= 1;
    }
}