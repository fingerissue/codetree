use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();
    
    for i in 1..=n {
        if i % 2 == 1 {
            println!("*");
        } else {
            for _ in 0..i {
                print!("* ");
            }
            print!("\n");
        }
    }
}