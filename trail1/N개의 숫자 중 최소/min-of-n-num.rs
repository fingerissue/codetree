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
                                                                                

    let mut min = arr[0];

    for &val in &arr[1..] {
        if min > val {
            min = val;
        }
    }

    println!("{min} {}", arr.iter().filter(|&&x| x == min).count());
}