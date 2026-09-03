fn main() {
    let ft : f32 = 30.48;
    let mi : i32 = 160934;

    let a : f32 = 9.2;
    let b : f32 = 1.3;

    println!("{}ft = {:.1}cm", a, a * ft);
    println!("{}mi = {:.1}cm", b, b * mi as f32);
}