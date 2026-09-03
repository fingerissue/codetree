fn main() {
    let mut a : i8 = 2;
    let mut b : i8 = 5;

    let temp = a;
    a = b;
    b = temp;

    println!("{}\n{}", a, b);
}