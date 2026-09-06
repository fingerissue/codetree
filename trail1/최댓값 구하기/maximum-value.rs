use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let a : i8 = tokens.next().unwrap().parse().unwrap();
    let b : i8 = tokens.next().unwrap().parse().unwrap();
    let c : i8 = tokens.next().unwrap().parse().unwrap();
    
    if a >= b && a >= c {
        println!("{}", a);
    } else if b >= a && b >= c {
        println!("{}", b);
    } else {
        println!("{}", c);
    }
}