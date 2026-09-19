use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();

    let mut cnt = 0;

    for i in 0..n {
        for j in 0..i {
            print!("  ");
        }
        for j in 1..=n - i {
            if cnt == 9 {
                cnt = 0;
            }
            cnt += 1;

            print!("{} ", cnt);
        }
        print!("\n");
    }
}