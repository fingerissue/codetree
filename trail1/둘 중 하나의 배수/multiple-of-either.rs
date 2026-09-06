use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    
    let a : i16 = buffer.trim().parse().unwrap();

    println!("{}", if a % 3 == 0 || a % 5 == 0 { "1" } else { "0" });
}