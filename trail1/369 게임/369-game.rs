use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();

    for i in 1..=n {
        if i % 3 == 0 {
            print!("0 ");
        } else if i % 10 == 3 || i % 10 == 6 || i % 10 == 9 {
            print!("0 ");
        } else if i / 10 == 3 || i / 10 == 6 || i / 10 == 9 {
            print!("0 ");
        } else {
            print!("{} ", i);
        }
    }
}