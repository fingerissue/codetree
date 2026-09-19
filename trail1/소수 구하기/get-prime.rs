use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();

    let mut is_sosu = true;

    for i in 2..=n {
        for j in 2..i {
            if i % j == 0 {
                is_sosu = false;
                break;
            }
        }

        if is_sosu {
            print!("{} ", i);
        } else {
            is_sosu = true;
        }
    }
}