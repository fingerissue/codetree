use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();

    let mut cnt = 2;

    for _ in 0..n {
        for _ in 0..n {
            print!("{} ", cnt);

            if cnt == 8 {
                cnt = 2;
            } else {
                cnt += 2;
            }
        }
        print!("\n");
    }
}