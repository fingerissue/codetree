use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();
    
    for i in n..=100 {
        if i >= 90 {
            print!("A ");
        } else if i >= 80 {
            print!("B ");
        } else if i >= 70 {
            print!("C ");
        } else if i >= 60 {
            print!("D ");
        } else {
            print!("F ");
        }
    }
}