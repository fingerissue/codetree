use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();

    let mut i = 1;
    while i <= n {
        print!("{} ", i);
        i += 1;
    }
}