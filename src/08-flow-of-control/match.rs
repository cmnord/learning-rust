fn main() {
    let number = 13;

    println!("Tell me about {}", number);

    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // exclusive range is an experimental feature!
        // 13..19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;

    let binary = match boolean {
        // the arms of a match must cover all possible values
        false => 0,
        true => 1, // i.e. you cannot comment out one of these arms.
    };

    println!("{} -> {}", boolean, binary);

    destructure_tuples();
    destructure_enum();
    destructure_references();
    destructure_struct();
    guard();
    binding();
}

fn destructure_tuples() {
    let pair = (0, -2);
    println!("Tell me about {:?}", pair);

    match pair {
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        _ => println!("It doesn't matter what they are"),
    }
}

#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn destructure_enum() {
    let color = Color::RGB(122, 17, 40);
    println!("What color is it?");

    match color {
        Color::Red => println!("red"),
        Color::Blue => println!("blue"),
        Color::Green => println!("green"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, blue: {}", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}", c, m, y),
        Color::CMYK(c, m, y, k) => println!(
            "Cyan: {}, magenta: {}, yellow: {}: key(black): {}",
            c, m, y, k
        ),
    }
}

fn destructure_references() {
    let reference = &4;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // start with non-reference
    let _not_a_reference = 3;
    // `ref` modifies the assignment such that a reference is created for the element.
    // This reference is assigned (to?)
    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            // got a reference. must dereference before adding anything to it
            *m += 10;
            println!("we added 10. `mut_value`: {:?}", m);
        }
    }
}

fn destructure_struct() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {}, y = {}", b, y),
        // order doesn't matter
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
        // can ignore some variables
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // but can't NOT mention some
        // Foo { y } => println!("y = {}", y),
    }
}

fn guard() {
    let pair = (2, -2);
    println!("Tell me about {:?}", pair);

    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}

fn age() -> u32 {
    22
}

fn binding() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I'm not born yet I guess"),
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }
}
