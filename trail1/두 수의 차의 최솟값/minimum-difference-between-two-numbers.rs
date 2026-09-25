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
                                
    let mut min = i32::MAX;

    for i in 0..n {
        for j in i + 1..n {
            let cha = arr[i].max(arr[j]) - arr[i].min(arr[j]);
            if min > cha {
                min = cha;
            }
        }
    }

    println!("{min}");
}