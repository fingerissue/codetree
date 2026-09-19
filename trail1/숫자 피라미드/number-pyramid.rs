use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();

    for i in 0..n {
        for j in 0..=i {
            if i >= j {
                print!("{} ", i + 1);
            }
        }
        print!("\n");
    }
}