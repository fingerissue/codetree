use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let mut arr : [i32; 100] = [0; 100];
    let mut zero_idx = 0;

    for (i, token) in tokens.enumerate() {
        let token : i32 = token.parse().unwrap();
        arr[i] = token;
        if arr[i] == 0 {
            zero_idx = i;
            break;
        }
    }

    let sum = arr[zero_idx - 1] + arr[zero_idx - 2] + arr[zero_idx - 3];

    println!("{sum}");
}