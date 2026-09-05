use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : u8 = buffer.trim().parse().unwrap();
    
    if n >= 80 {
        println!("pass");
    } else {
        println!("{} more score", 80 - n);
    }
}