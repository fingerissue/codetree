use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();

    let mut cnt = 0;

    for i in (0..n).rev() {
        for j in 0..n - i - 1 {
            print!("  ");
        }
        for j in 0..=i {
            print!("{} ", (cnt as u8 + b'A') as char);
            
            cnt += 1;
            if (cnt - 1 as u8 + b'A') as char == 'Z' {
                cnt = 0;
            }
        }
        print!("\n");
    }
}