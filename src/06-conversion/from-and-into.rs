use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number { value: value }
    }
}

fn main() {
    let my_str = "Hello";
    let _my_string = String::from(my_str);

    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    // requires type annotation to do Into
    let num2: Number = int.into();
    println!("My other number is {:?}", num2);
}
