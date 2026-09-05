use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    
    let n : i16 = buffer.trim().parse().unwrap();

    if n < 0 {
        println!("ice");
    } else if n >= 100 {
        println!("vapor");
    } else {
        println!("water");
    }
}