use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n : usize = buffer.trim().parse().unwrap();

    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let arr : Vec<i32> = buffer.trim().split_whitespace().take(n)
                                .map(|x| x.parse().unwrap())
                                .collect();
                                
    let mut max = 0;

    for i in (1..n).rev() {
        for j in 0..i {
            if arr[i] > arr [j] {
                let cha = arr[i] - arr[j];
                if max < cha {
                    max = cha;
                }
            }
        }
    }

    println!("{max}");
}