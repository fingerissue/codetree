use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();

    let mut sum : i16 = 0;

    for i in 0..n {
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        let mut tokens = buffer.trim().split_whitespace();

        let a : i8 = tokens.next().unwrap().parse().unwrap();
        let b : i8 = tokens.next().unwrap().parse().unwrap();
        sum = 0;

        for j in a..=b {
            if j % 2 == 0 {
                sum += j as i16;
            }
        }
        println!("{}", sum);
    }
}