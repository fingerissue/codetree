use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let sex : i8 = tokens.next().unwrap().parse().unwrap();
    let age : i8 = tokens.next().unwrap().parse().unwrap();
    
    if sex == 0 {
        if age >= 19 {
            println!("MAN");
        } else {
            println!("BOY");
        }
    } else if sex == 1 {
        if age >= 19 {
            println!("WOMAN");
        } else {
            println!("GIRL");
        }
    }
}