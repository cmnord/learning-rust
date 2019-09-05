use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RGB ({r}, {g}, {b}) 0x{r:02X}{g:02X}{b:02X}", r=self.red, g=self.green, b=self.blue)
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    for color in [
        Color { red: 128, green: 255, blue: 90},
        Color { red: 0, green: 3, blue: 254},
        Color { red: 0, green: 0, blue: 0}
    ].iter() {
        println!("{}", *color);
    }
}