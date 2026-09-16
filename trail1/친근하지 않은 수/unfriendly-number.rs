use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i16 = buffer.trim().parse().unwrap();

    let mut cnt : i16 = 0;
    
    for i in 1..=n {
        if i % 2 == 0 || i % 3 == 0 || i % 5 == 0 {
            continue;
        }
        cnt += 1;
    }

    println!("{}", cnt);
}