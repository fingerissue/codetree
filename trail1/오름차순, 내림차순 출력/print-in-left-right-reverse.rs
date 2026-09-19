use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();

    let mut cnt = 1;

    for i in 0..n {
        for j in 0..n {
            if i % 2 == 0 && j != 0 {
                cnt += 1;
            } else if i % 2 == 1 && j != 0 {
                cnt -= 1;
            }
            print!("{}", cnt);
        }
        print!("\n");
    }
}