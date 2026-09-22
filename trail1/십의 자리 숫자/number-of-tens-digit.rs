use std::io;

fn main () {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let mut arr : [i8; 10] = [0; 10];

    for token in tokens {
        let mut token : usize = token.parse().unwrap();
        if token == 0 {
            break;
        }
        token /= 10;
        arr[token] += 1;
    }

    for i in 1..10 {
        println!("{i} - {}", arr[i]);
    }
}