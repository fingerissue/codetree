use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let n : i8 = buffer.trim().parse().unwrap();
    
    if n <= 7 {
        if n % 2 == 1 {
            println!("31");
        } else {
            if n == 2 {
                println!("28");
            } else {
                println!("30");
            }
        }
    } else {
        if n % 2 == 0 {
            println!("31");
        } else {
            println!("30");
        }
    }
}