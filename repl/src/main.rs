fn main() {
    let num: i32 = "5".parse().unwrap();
    println!("num: {}", num);

    // Split the floating point number into integer and fractional parts
    let num: f32 = 3.14;
    let int_part = num.trunc();
    let frac_part = num.fract();
    println!("int_part: {}", int_part);
    println!("frac_part: {}", frac_part);
}
