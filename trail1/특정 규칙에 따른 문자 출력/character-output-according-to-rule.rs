use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();

    for i in 0..n {
        for _ in 0..n - 1 - i {
            print!("  ");
        }
        for _ in 0..i + 1 {
            print!("@ ");
        }
        print!("\n");
    }

    for i in 0..n - 1 {
        for _ in 0..n - 1 - i {
            print!("@ ");
        }
        print!("\n");
    }
}