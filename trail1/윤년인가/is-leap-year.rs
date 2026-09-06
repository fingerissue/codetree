use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let y : i16 = buffer.trim().parse().unwrap();
    
    if y % 100 == 0 && y % 400 != 0 {
        println!("false");
    } else if y % 4 == 0 {
        println!("true");
    } else {
        println!("false");
    }
}