use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();

    for _ in 1..=n {
        for i in (1..=n).rev() {
            print!("{} ", i);
        }
        print!("\n");
    }
}