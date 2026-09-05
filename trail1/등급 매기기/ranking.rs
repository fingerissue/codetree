use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    
    let n : u8 = buffer.trim().parse().unwrap();

    if n >= 90 {
        println!("A");
    } else if n >= 80 {
        println!("B");
    } else if n >= 70 {
        println!("C");
    } else if n >= 60 {
        println!("D")
    } else {
        println!("F");
    }
}