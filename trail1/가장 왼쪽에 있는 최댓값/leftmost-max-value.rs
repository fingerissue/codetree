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

    let mut pos : [usize; 1000] = [0; 1000];
    pos[0] = n;

    for i in 0..n {
        let mut cur = 0;

        for j in 0..pos[i] {
            if arr[cur] < arr[j] {
                cur = j;
            }
        }

        pos[i + 1] = cur;

        if pos[i + 1] == 0 {
            break;
        }
    }

    for &val in &pos[1..] {
        if val == 0 {
            break;
        }

        print!("{} ", val + 1);
    }
    print!("1");
}