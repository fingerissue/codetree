use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let mut n : i16 = buffer.trim().parse().unwrap();
    let mut i = 1;

    loop {
        if (n / i) <= 1 {
            break;
        }
        n /= i;
        i += 1;
    }

    println!("{}", i);
}