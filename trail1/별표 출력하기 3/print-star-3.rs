use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();

    for i in 0..n {
        for _ in 0..i {
            print!("  ");
        }
        
        for _ in 0..2 * n - 2 * i - 1 {
            print!("* ");
        }
        print!("\n");
    }
}