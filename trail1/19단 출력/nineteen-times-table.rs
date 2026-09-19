fn main() {
    for i in 1..20 {
        for j in 1..20 {
            print!("{} * {} = {} ", i, j, i * j);
            if j == 19 {
                continue;
            } else if j % 2 == 1 {
                print!("/ ");
            } else {
                print!("\n");
            }
        }
        print!("\n");
    }
}