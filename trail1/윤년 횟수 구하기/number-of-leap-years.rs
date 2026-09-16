use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i16 = buffer.trim().parse().unwrap();

    let mut yun : i16 = 0;
    
    for i in 1..=n {
        if i % 4 == 0 {
            if i % 100 == 0 && i % 400 != 0 {
                yun += 0;
            } else {
                yun += 1;
            }
        }
    }

    println!("{}", yun);
}