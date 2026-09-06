use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    
    let m : i8 = buffer.trim().parse().unwrap();

    if 3 <= m && m <= 5 {
        println!("Spring");
    } else if 6 <= m && m <= 8 {
        println!("Summer");
    } else if 9 <= m && m <= 11 {
        println!("Fall");
    } else {
        println!("Winter");
    }
}