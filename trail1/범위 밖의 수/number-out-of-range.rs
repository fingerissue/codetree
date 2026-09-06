use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let a : i8 = buffer.trim().parse().unwrap();
    
    if a < 10 || a > 20 {
        println!("yes");
    } else {
        println!("no");
    }
}