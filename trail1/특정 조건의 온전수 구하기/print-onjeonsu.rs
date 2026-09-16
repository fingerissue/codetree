use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i16 = buffer.trim().parse().unwrap();
    
    for i in 1..=n {
        if i % 2 == 0 || i % 10 == 5 || (i % 3 == 0 && i % 9 != 0) {
            continue;
        }
        print!("{} ", i);
    }
}