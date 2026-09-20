use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i32 = buffer.trim().parse().unwrap();
    let mut cnt = 0;

    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();
    
    let mut arr : [i32; 100] = [0; 100];

    for i in 0..n {
        let token : i32 = tokens.next().unwrap().parse().unwrap();
        if token % 2 == 0 {
            arr[cnt] = token;
            cnt += 1;
        }
    }

    for i in (0..cnt).rev() {
        print!("{} ", arr[i]);
    }

}