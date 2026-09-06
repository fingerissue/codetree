use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let a_math : i8 = tokens.next().unwrap().parse().unwrap();
    let a_eng : i8 = tokens.next().unwrap().parse().unwrap();
    let b_math : i8 = tokens.next().unwrap().parse().unwrap();
    let b_eng : i8 = tokens.next().unwrap().parse().unwrap();

    if a_math > b_math {
        println!("A");
    } else if a_math < b_math {
        println!("B");
    } else if a_eng > b_eng {
        println!("A");
    } else {
        println!("B");
    }
}