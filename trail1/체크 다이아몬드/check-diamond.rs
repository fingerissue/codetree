use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i16 = buffer.trim().parse().unwrap();

    for i in 0..n {
        for _ in 0..n - i - 1 {
            print!(" ");
        }
        for _ in 0..=i {
            print!("* ");
        }
        print!("\n");
    }

    for i in 0..n - 1 {
        for _ in 0..=i {
            print!(" ");
        }
        for _ in 0..n - i - 1 {
            print!("* ");
        }
        print!("\n");
    }
}