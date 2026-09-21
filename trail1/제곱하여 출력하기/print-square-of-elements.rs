use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : usize = buffer.trim().parse().unwrap();

    let mut input : [i32; 100] = [0; 100];
    let mut result : [i32; 100] = [0; 100];

    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();
    for (i, token) in tokens.enumerate() {
        input[i] = token.parse().unwrap();
        result[i] = input[i] * input[i];
    }

    for i in 0..n {
        print!("{} ", result[i]);
    }
}