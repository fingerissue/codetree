use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut n : i16 = buffer.trim().parse().unwrap();
    let mut cnt = 0;

    while n < 1000 {
        if n % 2 == 0 {
            n = n * 3 + 1;
        } else {
            n = n * 2 + 2;
        }
        cnt += 1;
    }

    println!("{}", cnt);
}