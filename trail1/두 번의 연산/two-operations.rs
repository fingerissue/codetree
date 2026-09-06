use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let mut a : i8 = buffer.trim().parse().unwrap();

    if a % 2 != 0 {
        a += 3;
    }

    if a % 3 == 0 {
        a /= 3;
    }

    println!("{}", a);
}