use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();

    let mut sum : i16 = 0;
    
    for i in n..=100 {
        sum += i as i16;
    }

    println!("{}", sum);
}