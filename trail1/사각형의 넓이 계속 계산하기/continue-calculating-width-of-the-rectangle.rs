use std::io;

fn main() {
    let mut buffer = String::new();
    loop {
        io::stdin().read_line(&mut buffer).unwrap();
        let mut tokens = buffer.trim().split_whitespace();

        let a : i8 = tokens.next().unwrap().parse().unwrap();
        let b : i8 = tokens.next().unwrap().parse().unwrap();
        let c : char = tokens.next().unwrap().parse().unwrap();

        println!("{}", a as i32 * b as i32);
        
        if c == 'C' {
            break;
        }

        buffer.clear();
    }
}