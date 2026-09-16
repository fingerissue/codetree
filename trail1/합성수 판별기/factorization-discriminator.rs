use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i16 = buffer.trim().parse().unwrap();

    let mut is_hapsungsu = false;

    for i in 2..n {
        if n % i == 0 {
            is_hapsungsu = true;
            break;
        }
    }

    println!("{}", if is_hapsungsu { "C" } else { "N" });
}