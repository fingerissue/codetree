use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let a : i8 = tokens.next().unwrap().parse().unwrap();
    let b : i8 = tokens.next().unwrap().parse().unwrap();
    let c : i8 = tokens.next().unwrap().parse().unwrap();

    let mut is_c_baesu = false;

    for i in a..=b {
        if i % c == 0 {
            is_c_baesu = true;
            break;
        }
    }

    println!("{}", if is_c_baesu { "YES" } else { "NO" });
}