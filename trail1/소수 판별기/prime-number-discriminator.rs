use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i16 = buffer.trim().parse().unwrap();

    let mut satisfied = true;
    
    for i in 2..n {
        if n % i == 0 {
            satisfied = false;
        }
    }

    println!("{}", if satisfied { "P" } else { "C" });
}