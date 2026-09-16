use std::io;

fn main() {
    let mut buffer = String::new();
    let mut sum : i16 = 0;
    let mut cnt = 0;

    loop {
        io::stdin().read_line(&mut buffer).unwrap();
        let n : i8 = buffer.trim().parse().unwrap();

        if n < 20 || n >= 30 {
            break;
        }

        sum += n as i16;
        cnt += 1;

        buffer.clear();
    }

    println!("{:.2}", sum as f32 / cnt as f32);
}