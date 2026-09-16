use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let a : i8 = tokens.next().unwrap().parse().unwrap();
    let b : i8 = tokens.next().unwrap().parse().unwrap();

    let mut prod : i32 = 1;
    
    for i in 1..=b {
        if i % a == 0 {
            prod *= i as i32;
        }
    }

    println!("{}", prod);
}