use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let a : i16 = tokens.next().unwrap().parse().unwrap();
    let b : i16 = tokens.next().unwrap().parse().unwrap();

    let mut is_gongyaksu = false;

    for i in a..=b {
        if 1920 % i == 0 && 2880 % i == 0 {
            is_gongyaksu = true;
            break;
        }
    }

    println!("{}", is_gongyaksu as u8);
}