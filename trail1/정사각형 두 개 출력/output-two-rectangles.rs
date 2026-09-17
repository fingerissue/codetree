use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();

    for _ in 0..n {
        for _ in 0..n {
            print!("*");
        }
        print!("\n");
    }
    print!("\n");
    for _ in 0..n {
        for _ in 0..n {
            print!("*");
        }
        print!("\n");
    }
}