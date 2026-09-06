use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let a_cold : &str = tokens.next().unwrap();
    let a_yeol : i8 = tokens.next().unwrap().parse().unwrap();
    let b_cold : &str = tokens.next().unwrap();
    let b_yeol : i8 = tokens.next().unwrap().parse().unwrap();
    let c_cold : &str = tokens.next().unwrap();
    let c_yeol : i8 = tokens.next().unwrap().parse().unwrap();

    let mut check = 0;
    
    if a_cold == "Y" && a_yeol >= 37 {
        check += 1;
    }
    if b_cold == "Y" && b_yeol >= 37 {
        check += 1;
    }
    if c_cold == "Y" && c_yeol >= 37 {
        check += 1;
    }

    if check >= 2 {
        println!("E");
    } else {
        println!("N");
    }
}