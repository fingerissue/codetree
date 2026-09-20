use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let mut arr : [i32; 10] = [0; 10];
    let mut sum = 0;
    let mut cnt = 0;

    for i in 0..10 {
        let token : i32 = tokens.next().unwrap().parse().unwrap();
        
        if token == 0 {
            break;
        }
        arr[i] = token;
    }

    for val in arr {
        if val == 0 {
            break;
        }
        sum += val;
        cnt += 1;
    }

    let avg = sum as f64 / cnt as f64;
    println!("{sum} {avg:.1}");
}