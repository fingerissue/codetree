use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let a : i8 = tokens.next().unwrap().parse().unwrap();
    let b : i8 = tokens.next().unwrap().parse().unwrap();

    for i in (2..9).step_by(2) {
        for j in (a.min(b)..=b.max(a)).rev() {
            print!("{} * {} = {}", j, i, i * j);
            if j > a.min(b) {
                print!(" / ");
            }
        }
        print!("\n");
    }
}