use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let mut input : [i32; 100] = [0; 100];
    for (i, token) in tokens.enumerate() {
        let token : i32 = token.parse().unwrap();
        if token == 0 {
            break;
        }
        
        if token % 2 == 1 {
            input[i] = token + 3;
        } else {
            input[i] = token / 2;
        }
    }

    for val in input {
        if val == 0 {
            break;
        }
        print!("{val} ");
    }
}