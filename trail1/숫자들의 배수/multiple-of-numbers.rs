use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();

    let mut result : [i32; 100] = [0; 100];

    let mut i = 0;
    let mut cnt5 = 2;

    loop {
        result[i] = (i + 1) as i32 * n as i32;
        if result[i] % 5 == 0 {
            cnt5 -= 1;
        }

        if cnt5 == 0 {
            break;
        }
        i += 1;
    }

    for val in result {
        if val == 0 {
            break;
        }
        print!("{val} ");
    }
}