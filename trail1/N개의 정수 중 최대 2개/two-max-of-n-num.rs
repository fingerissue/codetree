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

    let mut tmp = 0;
    for _ in 0..n{
        for i in 0..n - 1 {
            if arr[i] < arr[i + 1] {
                tmp = arr[i];
                arr[i] = arr[i + 1];
                arr[i + 1] = tmp;
            }
        }
    }

    println!("{} {}", arr[0], arr[1]);
}