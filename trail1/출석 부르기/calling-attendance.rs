use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    
    let n : u8 = buffer.trim().parse().unwrap();

    if n == 1 {
        println!("John");
    } else if n == 2 {
        println!("Tom");
    } else if n == 3 {
        println!("Paul");
    } else {
        println!("Vacancy")
    }
}