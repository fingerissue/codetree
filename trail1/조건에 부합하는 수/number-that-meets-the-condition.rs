use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let a : i8 = buffer.trim().parse().unwrap();
    
    for i in 1..=a {
        if i % 2 == 0 && i % 4 != 0 {
            continue;
        }
        if (i / 8) % 2 == 0 {
            continue;
        }
        if (i % 7) < 4 {
            continue;
        }
        print!("{} ", i);
    }
}