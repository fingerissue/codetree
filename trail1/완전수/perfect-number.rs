use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();
    
    let start : i16 = tokens.next().unwrap().parse().unwrap();
    let end : i16 = tokens.next().unwrap().parse().unwrap();

    let mut sum = 0;
    let mut cnt = 0;

    for i in start..=end {
        for j in 1..i {
            if i % j == 0 {
                sum += j;
            }
        }

        if sum == i {
            cnt += 1;
        }
        sum = 0;
    }

    println!("{}", cnt);
}