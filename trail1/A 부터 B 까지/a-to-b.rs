use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let mut a : i8 = tokens.next().unwrap().parse().unwrap();
    let b : i8 = tokens.next().unwrap().parse().unwrap();

    while a <= b {
        print!("{} ", a);

        if a % 2 == 1 {
            a *= 2;
        } else {
            a += 3;
        }
    }
}