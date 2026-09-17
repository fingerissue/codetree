use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();

    for i in 0..2 * n + 1 {
        for j in 0..2 * n + 1 {
            if i % 2 == 1 && j % 2 == 1 {
                print!("  ");
            } else {
                print!("* ");
            }
        }
        print!("\n");
    }
}