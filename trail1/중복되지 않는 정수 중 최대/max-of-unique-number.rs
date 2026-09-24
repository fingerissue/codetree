use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : usize = buffer.trim().parse().unwrap();

    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut arr : Vec<i32> = buffer.trim().split_whitespace().take(n)
                                .map(|x| x.parse().unwrap())
                                .collect();
    
    for i in 0..n {
        let no_jungbok = arr[i];

        for j in 0..n {
            if j == i {
                continue;
            }

            if no_jungbok == arr[j] {
                arr[i] = -1;
                arr[j] = -1;
            }
        }
    }

    let mut max = -1;
    for var in arr {
        if max < var {
            max = var;
        }
    }

    println!("{max}");
}