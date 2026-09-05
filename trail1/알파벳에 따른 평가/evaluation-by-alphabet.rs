use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    
    let score : &str = buffer.trim();

    if score == "S" {
        println!("Superior");
    } else if score == "A" {
        println!("Excellent");
    } else if score == "B" {
        println!("Good");
    } else if score == "C" {
        println!("Usually");
    } else if score == "D" {
        println!("Effort");
    } else {
        println!("Failure");
    }
}