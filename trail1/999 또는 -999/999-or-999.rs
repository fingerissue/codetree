use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    
    let arr : Vec<i32> = buffer.trim().split_whitespace()
                                .map(|x| x.parse().unwrap())
                                .collect();
    
    let mut min = arr[0];
    let mut max = arr[0];

    for val in arr {
        if val == 999 || val == -999 {
            break;
        }

        if max < val {
            max = val;
        }

        if min > val {
            min = val;
        }
    }
    
    println!("{max} {min}");
}