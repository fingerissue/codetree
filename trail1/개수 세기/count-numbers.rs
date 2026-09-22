use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let mut n : i32 = tokens.next().unwrap().parse().unwrap();
    let mut m : i32 = tokens.next().unwrap().parse().unwrap();

    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let cnt = buffer.trim().split_whitespace().take(n as usize)
                    .map(|token| token.parse::<i32>().unwrap())
                    .filter(|&i| i == m).count();

    println!("{cnt}");
}