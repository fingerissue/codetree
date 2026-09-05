use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    
    let a : f32 = buffer.trim().parse().unwrap();

    if a >= 1.0 {
        println!("High");
    } else if a >= 0.5 {
        println!("Middle");
    } else {
        println!("Low");
    }
}