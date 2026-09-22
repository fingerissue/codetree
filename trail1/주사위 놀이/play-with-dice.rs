use std::io;

fn main () {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let mut dice : [i8; 7] = [0; 7];

    for token in tokens {
        let token : usize = token.parse().unwrap();
        dice[token] += 1;
    }

    for i in 1..7 {
        println!("{i} - {}", dice[i]);
    }
}