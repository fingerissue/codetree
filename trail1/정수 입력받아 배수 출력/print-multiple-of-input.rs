use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i16 = buffer.trim().parse().unwrap();

    for i in 1..=5 {
        print!("{} ", n * i);
    }
}