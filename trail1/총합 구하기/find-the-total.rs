use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let a : i8 = tokens.next().unwrap().parse().unwrap();
    let b : i8 = tokens.next().unwrap().parse().unwrap();

    let mut sum : i16 = 0;
    
    for i in a.min(b)..=b.max(a) {
        if i % 6 == 0 && i % 8 != 0 {
            sum += i as i16;
        }
    }

    println!("{}", sum);
}