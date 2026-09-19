use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i16 = buffer.trim().parse().unwrap();

    let mut cnt = 1;

    for i in 0..n {
        for j in 0..n {
            if j == 0 {
                if i % 2 == 0 {
                    cnt = n * i + 1;
                } else {
                    cnt = n * (i + 1);
                }
            }

            print!("{} ", cnt);

            if i % 2 == 0 {
                cnt += 1;
            } else {
                cnt -= 1;
            }
        }
        print!("\n");
    }
}