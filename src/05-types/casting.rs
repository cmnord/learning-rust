#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // no implicit conversion allowed
    // let integer: u8 = decimal;

    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as a u16 is {}", 1000 as u16);

    // under the hood first 8 LSB are kept, while the rest towards the MSB get
    // truncated.
    println!("-1 as u8 is {}", -1i8 as u8);

    // for pos numbers, this is the same as mod.
    println!("1000 mod 258 is {}", 1000 & 256);

    // when casting to a signed type, the bitwise result is the same as first casting
    // to the corresponding unsigned type. If the MSB of that value is 1, then the
    // value is negative.
    println!("128 as i16 is {}", 128 as i16);
    println!("128 as i8 is {}", 128 as i8);
    println!("1000 as u8 is {}", 1000 as u8);
    println!("232 as i8 is {}", 232 as i8);

    let x = 1f32;
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
}
