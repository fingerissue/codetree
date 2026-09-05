use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    
    let n : u16 = buffer.trim().parse().unwrap();

    if n >= 3000 {
        println!("book");
    } else if n >= 1000 {
        println!("mask");
    } else {
        println!("no");
    }
}