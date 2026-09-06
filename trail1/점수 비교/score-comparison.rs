use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let a_math : u8 = tokens.next().unwrap().parse().unwrap();
    let a_eng : u8 = tokens.next().unwrap().parse().unwrap();
    
    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    tokens = buffer.trim().split_whitespace();

    let b_math : u8 = tokens.next().unwrap().parse().unwrap();
    let b_eng : u8 = tokens.next().unwrap().parse().unwrap();
    
    if a_math > b_math && a_eng > b_eng {
        println!("1");
    } else {
        println!("0");
    }
}