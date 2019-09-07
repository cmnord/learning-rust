#[allow(dead_code)]
use crate::List::*;

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff00000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

enum List {
    // TODO: significance of Box?
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        // TODO: why &self, *self?
        match *self {
            // TODO: what is ref?
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
            Nil => format!("Nil"),
        }
    }
}

fn main() {
    use crate::Status::{Poor, Rich};
    use crate::Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("money"),
        Poor => println!("no money"),
    }

    match work {
        Civilian => println!("work"),
        Soldier => println!("fight"),
    }

    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);

    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has len {}", list.len());
    println!("{}", list.stringify());
}
