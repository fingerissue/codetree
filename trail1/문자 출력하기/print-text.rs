use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let input : char = buffer.trim().parse().unwrap();

    for i in 0..8 {
        print!("{}", input);
    }
}