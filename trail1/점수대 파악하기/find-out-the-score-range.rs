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
        } else if token < 10 {
            continue;
        }

        token /= 10;
        if token == 10 {
            arr[0] += 1;
        } else {
            arr[token] += 1;
        }
    }
    
    println!("100 - {}", arr[0]);
    for i in (1..10).rev() {
        println!("{i}0 - {}", arr[i]);
    }
}