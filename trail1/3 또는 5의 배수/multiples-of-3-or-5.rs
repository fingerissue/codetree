use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let a : i8 = buffer.trim().parse().unwrap();
    
    if a % 3 == 0 {
        println!("YES");
    } else {
        println!("NO");
    }

    if a % 5 == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}