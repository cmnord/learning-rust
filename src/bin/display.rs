use std::fmt;

#[derive(Debug)]
struct Complex {
    real: f32,
    imag: f32,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {
    let c = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("Display: {}", c);
    println!("Debug: {:?}", c);
}
