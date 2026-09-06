use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    
    let a : i16 = buffer.trim().parse().unwrap();

    println!("{}", if a % 13 == 0 || a % 19 == 0 { "True" } else { "False" });
}