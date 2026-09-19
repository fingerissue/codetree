use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();

    let mut sum : i16 = 1;

    for i in 0..n {
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        let mut tokens = buffer.trim().split_whitespace();

        let a : i8 = tokens.next().unwrap().parse().unwrap();
        let b : i8 = tokens.next().unwrap().parse().unwrap();
        sum = 0;

        for i in a..=b {
            if i % 2 == 0 {
                sum += i as i16;
            }
        }
        println!("{}", sum);
    }
}