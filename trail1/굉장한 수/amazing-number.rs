use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    
    let a : i16 = buffer.trim().parse().unwrap();

    if a % 2 == 1 && a % 3 == 0 || a % 2 == 0 && a % 5 == 0 {
        println!("true")
    } else {
        println!("false");
    }
}