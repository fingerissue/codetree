use std::io;

fn main() {
    let mut buffer = String::new();
    loop {
        io::stdin().read_line(&mut buffer).unwrap();
        let n : i8 = buffer.trim().parse().unwrap();
        
        if n < 25 {
            println!("Higher");
        } else if n > 25 {
            println!("Lower");
        } else {
            println!("Good");
            break;
        }

        buffer.clear();
    }
}