use std::io;

fn main () {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : i8 = buffer.trim().parse().unwrap();

    let mut arr : [i32; 100] = [0; 100];
    arr[0] = 1;
    arr[1] = n as i32;

    let mut satisfied = true;
    let mut i = 2;

    while satisfied {
        arr[i] = arr[i - 1] + arr[i - 2];
        if arr[i] > 100 {
            satisfied = false;
        }
        i += 1;
    }

    for val in arr {
        if val == 0 {
            break;
        }
        print!("{val} ");
    }
}