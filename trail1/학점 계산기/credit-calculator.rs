use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : usize = buffer.trim().parse().unwrap();

    let mut scores : [f64; 5] = [0.0; 5];
    let mut sum = 0.0;

    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();

    for i in 0..n {
        let token : f64 = tokens.next().unwrap().parse().unwrap();
        scores[i] = token;
    }

    for score in scores {
        sum += score;
    }

    let avg = sum / n as f64;
    println!("{avg:.1}");

    if avg >= 4.0 {
        println!("Perfect");
    } else if avg >= 3.0 {
        println!("Good");
    } else {
        println!("Poor");
    }
}