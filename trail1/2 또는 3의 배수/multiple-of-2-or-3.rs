use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();

    for i in 1..=n {
        if i % 2 == 0 || i % 3 == 0 {
            print!("1 ");
        } else {
            print!("0 ");
        }
    }   
}