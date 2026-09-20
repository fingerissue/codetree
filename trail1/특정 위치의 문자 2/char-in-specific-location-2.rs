use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let mut arr : [char; 10] = [' '; 10];
    for i in 0..10 {
        let token : char = tokens.next().unwrap().parse().unwrap();
        arr[i] = token;
    }

    println!("{} {} {}", arr[1], arr[4], arr[7]);
}