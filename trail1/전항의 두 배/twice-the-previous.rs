use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let a1 : i32 = tokens.next().unwrap().parse().unwrap();
    let a2 : i32 = tokens.next().unwrap().parse().unwrap();

    let mut arr : [i32; 10] = [0; 10];
    arr[0] = a1;
    arr[1] = a2;

    for i in 2..10 {
        arr[i] = arr[i - 1] + 2 * arr[i - 2];
    }

    for val in arr {
        print!("{val} ");
    }
}