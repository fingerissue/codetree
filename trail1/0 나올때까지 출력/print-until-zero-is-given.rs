use std::io;

fn main() {
    let mut buffer = String::new();
    loop {
        io::stdin().read_line(&mut buffer).unwrap();
        let n : i8 = buffer.trim().parse().unwrap();

        if n == 0 {
            break;
        }

        println!("{}", n);
        buffer.clear();
    }
}