use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let a : i16 = tokens.next().unwrap().parse().unwrap();
    let b : i16 = tokens.next().unwrap().parse().unwrap();

    let mut sum : i16 = 0;
    let mut cnt : i8 = 0;
    
    for i in a..=b {
        if i % 5 == 0 || i % 7 == 0 {
            sum += i as i16;
            cnt += 1;
        }
    }

    let avg = sum as f32 / cnt as f32;

    println!("{} {:.1}", sum, avg);
}