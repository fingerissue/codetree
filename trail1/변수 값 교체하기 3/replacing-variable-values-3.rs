fn main() {
    let mut a : i8 = 3;
    let mut b : i8 = 5;

    let temp : i8 = a;
    a = b;
    b = temp;

    println!("{}\n{}", a, b);
}