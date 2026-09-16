use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i16 = buffer.trim().parse().unwrap();

    let mut sum = 0;

    for i in 1..=100 {
        sum += i;
        if sum >= n {
            println!("{}", i);
            break;
        }
    }
}