use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let mut result : [char; 10] = [' '; 10];

    for i in 0..10 {
        let input : char = tokens.next().unwrap().parse().unwrap();
        result[i] = input;
    }

    for i in (0..10).rev() {
        print!("{}", result[i]);
    }
}