use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();

    let mut mul : i32 = 1;

    for i in 0..n {
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        let mut tokens = buffer.trim().split_whitespace();

        let a : i8 = tokens.next().unwrap().parse().unwrap();
        let b : i8 = tokens.next().unwrap().parse().unwrap();
        mul = 1;

        for i in a..=b {
            mul *= i as i32;
        }
        println!("{}", mul);
    }
}