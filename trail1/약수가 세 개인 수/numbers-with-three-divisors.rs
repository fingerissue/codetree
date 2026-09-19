use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.trim().split_whitespace();
    
    let start : i16 = tokens.next().unwrap().parse().unwrap();
    let end : i16 = tokens.next().unwrap().parse().unwrap();

    let mut yak_cnt = 0;
    let mut result = 0;

    for i in start..=end {
        for j in 1..=i {
            if i % j == 0 {
                yak_cnt += 1;
            }
        }

        if yak_cnt == 3 {
            result += 1;
        }
        yak_cnt = 0;
    }

    println!("{}", result);
}