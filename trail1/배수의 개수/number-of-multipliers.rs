use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let mut cnt3 : i8 = 0;
    let mut cnt5 : i8 = 0;
    
    for token in tokens {
        let token : i16 = token.parse().unwrap();
        
        if token % 3 == 0 {
            cnt3 += 1;
        }
        if token % 5 == 0 {
            cnt5 += 1;
        }
    }

    println!("{} {}", cnt3, cnt5);
}