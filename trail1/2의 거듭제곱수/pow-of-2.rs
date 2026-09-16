use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut n : i16 = buffer.trim().parse().unwrap();
    let mut cnt = 0;

    while n > 1 {
        n /= 2;
        cnt += 1;
    }

    println!("{}", cnt);
}