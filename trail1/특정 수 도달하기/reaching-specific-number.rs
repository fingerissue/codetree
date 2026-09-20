use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    let mut input : [i32; 10] = [0; 10];
    let mut sum = 0;
    let mut cnt = 0;

    for i in 0..10 {
        let token : i32 = tokens.next().unwrap().parse().unwrap();
        input[i] = token;
    }

    for n in input {
        if n >= 250 {
            break;   
        }
        sum += n;
        cnt += 1;
    }

    let avg : f32 = sum as f32 / cnt as f32;
    println!("{} {:.1}", sum, avg);
}