use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();

    for i in 0..n {
        for j in 0..i {
            print!("  ");
        }
        for j in 0..n - i {
            if i == 0 {
                print!("* ");
            } else if i % 2 == 1 && j % 2 == 1 {
                print!("  ");
            } else if i % 2 == 0 && j % 2 == 0 {
                print!("  ");
            } else {
                print!("* ");
            }
        }
        print!("\n");
    }
}