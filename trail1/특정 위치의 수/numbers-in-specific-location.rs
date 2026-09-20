use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let mut arr : [i32; 10] = [0; 10];

    for i in 0..10 {
        let token : i32 = tokens.next().unwrap().parse().unwrap();
        arr[i] = token;
    }

    let sum = arr[2] + arr[4] + arr[9];
    println!("{sum}");
}