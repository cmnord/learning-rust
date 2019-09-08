use std::fmt;

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    // fmt::Display automatically provides ToString.
    println!("{}", circle.to_string());

    // Parsing a string:
    // Parsing requires FromStr trait, which is implemtned for many std types.

    // idiomatic: provide type annotation
    let parsed: i32 = "5".parse().unwrap();
    // "turbofish" syntax
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}