use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    
    let a : i8 = buffer.trim().parse().unwrap();

    println!("{}", if a == 1 { "t" } else { "f" });
}