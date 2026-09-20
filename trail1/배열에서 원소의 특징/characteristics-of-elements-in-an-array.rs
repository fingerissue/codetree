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

    for i in 0..10 {
        if arr[i] % 3 == 0 {
            println!("{}", arr[i - 1]);
            break;
        }
    }
}