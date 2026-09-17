use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();

    for i in (0..n).rev() {
        for _ in (0..=i).rev() {
            print!("* ");
        }
        print!("\n");
    }

    for i in 1..n {
        for _ in (0..=i).rev() {
            print!("* ");
        }
        print!("\n");
    }
}