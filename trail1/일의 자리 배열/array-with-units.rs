use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let a = tokens.next().unwrap().parse().unwrap();
    let b = tokens.next().unwrap().parse().unwrap();

    let mut result : [i8; 10] = [0; 10];
    result[0] = a;
    result[1] = b;

    for i in 2..10 {
        result[i] = (result[i - 1] + result[i - 2]) % 10;
    }

    for val in result {
        print!("{val} ");
    }
}