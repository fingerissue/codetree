use std::io;

fn main() {
    let mut buffer = String::new();
    let mut cnt = 0;
    loop {
        io::stdin().read_line(&mut buffer).unwrap();
        let n : i8 = buffer.trim().parse().unwrap();

        if n % 2 == 0 {
            if cnt == 3 {
                break;
            }
            println!("{}", n / 2);
            cnt += 1;
        }

        if cnt == 3 {
            break;
        }

        buffer.clear();
    }
}