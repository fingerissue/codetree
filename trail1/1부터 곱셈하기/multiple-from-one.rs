use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i32 = buffer.trim().parse().unwrap();

    let mut prod = 1;

    for i in 1..=10 {
        prod *= i;
        if prod >= n {
            println!("{}", i);
            break;
        }
    }
}