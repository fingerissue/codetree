use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();

    let mut cnt = 0;

    for i in 0..n {
        for _ in 0..n {
            if i % 2 == 0 {
                cnt += 1;
            } else {
                cnt += 2;
            }
            print!("{} ", cnt);
        }
        print!("\n");
    }
}