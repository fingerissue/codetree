use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();

    for i in 1..=n {
        for j in 0..i {
            print!("* ");
        }
        print!("\n");
    }
    for i in (1..n).rev() {
        for j in 0..i {
            print!("* ");
        }
        print!("\n");
    }
}