fn main() {
    println!("variables");
    // basic types
    let i: i32 = 15;
    let f: f64 = 15.0;
    println!("this is an integer: {}", i);
    println!("this is a floating number: {}", f);
    // mutability - by default variables are immutable
    let mut m: i32 = 1;
    println!("m is: {}", m);
    m = 10;
    println!("m is: {}", m);
    // max values signed integers
    println!("max value of i8 is: {}", std::i8::MAX);
    println!("max value of i32 is: {}", std::i32::MAX);
    println!("max value of i64 is: {}", std::i64::MAX);
    // max values unsigned integers
    println!("max value of u8 is: {}", std::u8::MAX);
    // max values floating point
    println!("max value of f32 is: {}", std::f32::MAX);
}
