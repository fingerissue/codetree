use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let tokens = buffer.trim().split_whitespace();

    let mut sum = 0;

    for token in tokens {
        let n : i32 = token.parse().unwrap();
        sum += n;
    }

    println!("{}", sum);
}