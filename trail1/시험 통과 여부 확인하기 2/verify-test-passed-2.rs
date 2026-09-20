use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : usize = buffer.trim().parse().unwrap();

    let mut scores : [bool; 10] = [false; 10];

    for i in 0..n {
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        let mut tokens = buffer.trim().split_whitespace();

        let mut sum = 0;

        for i in 0..4 {
            let token : i32 = tokens.next().unwrap().parse().unwrap();
            sum += token;
        }

        if sum / 4 >= 60 {
            scores[i] = true;
        }
    }

    let mut cnt = 0;
    for i in 0..n {
        println!("{}", if scores[i] { "pass" } else { "fail" });
        if scores[i] {
            cnt += 1;
        }
    }
    println!("{cnt}");
}