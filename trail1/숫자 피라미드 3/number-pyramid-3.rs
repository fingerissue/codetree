use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();

    let mut cnt : i16 = 0;

    for i in 1..=n {
        cnt = i as i16;
        for j in 0..i {
            print!("{} ", cnt);
            cnt += i as i16;
        }
        print!("\n");
    }
}