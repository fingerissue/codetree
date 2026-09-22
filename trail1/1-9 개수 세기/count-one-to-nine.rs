use std::io;

fn main () {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : usize = buffer.trim().parse().unwrap();

    let mut numbers : [i8; 10] = [0; 10];

    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    for token in tokens {
        let token : usize = token.parse().unwrap();
        numbers[token] += 1;
    }

    for &num in &numbers[1..] {
        println!("{num}");
    }
}