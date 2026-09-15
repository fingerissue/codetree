use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let n : i8 = tokens.next().unwrap().parse().unwrap();
    let a : i8 = tokens.next().unwrap().parse().unwrap();

    let mut i = 1;
    while i <= n {
        if i % a == 0 {
            println!("1");
        } else {
            println!("0");
        }
        i += 1;
    }   
}