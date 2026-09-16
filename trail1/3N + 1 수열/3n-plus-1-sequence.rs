use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut n : i16 = buffer.trim().parse().unwrap();
    let mut cnt = 0;

    loop {
        if n == 1 {
            break;
        }

        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }

        cnt += 1;
    }

    println!("{}", cnt);
}