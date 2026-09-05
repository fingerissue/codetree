use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i32 = buffer.trim().parse().unwrap();

    println!("{}", n * n);
    if n < 5 {
        println!("tiny");
    }
}