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
    // multiple variables
    let (first_number, second_number) = (1, 2.0);
    println!("multiple variables: {}, {}", first_number, second_number);
    // large_number
    let large_number = 1_000_000_000;
    println!("large number: {}", large_number);
    // overflow number
    // let overflow_number : u8 = 256; // this will not run
    let overflow_number: u8 = 255;
    println!("overflow number: {}", overflow_number);
    //  number in different formats
    let format = 255;
    println!(
        "Octal: {:o}, Hex: {:X}, Binary: {:b}",
        format, format, format
    );
    // add int and floating point
    let int_num = 2;
    let float_num = 3.0;
    let add_int_float = int_num + float_num as i32;
    println!("add int and floating point: {}", add_int_float);
    // constant
    const  CONSTANT:u32= 2_000_032; // type (:u32) must be provided
    println!("CONSTANT: {}", CONSTANT);
}
