use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i16 = buffer.trim().parse().unwrap();

    let mut sum : i16 = 0;
    
    for i in 1..=n {
        if i != n && n % i == 0 {
            sum += i;
        }
    }

    println!("{}", if sum == n { "P" } else { "N" });
}