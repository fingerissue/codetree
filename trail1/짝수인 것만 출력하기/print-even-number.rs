use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : usize = buffer.trim().parse().unwrap();

    let mut input : [i8; 100] = [0; 100];

    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();
    for (i, token) in tokens.enumerate() {
        input[i] = token.parse().unwrap();
    }

    for &val in &input[0..n] {
        if val % 2 == 0 {
            print!("{val} ");
        }
    }
}