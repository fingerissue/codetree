use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let mut a : i16 = tokens.next().unwrap().parse().unwrap();
    let mut b : i16 = tokens.next().unwrap().parse().unwrap();

    print!("{}.", a / b);
    a %= b;
    for _ in 0..20 {
        a *= 10;
        print!("{}", a / b);
        a %= b;
    }
}