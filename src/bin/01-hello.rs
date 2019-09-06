use std::fmt;

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    println!("Hello World!");
    println!("I'm a Rustacean!");

    let x = 5 + 90 + 5;
    println!("Is `x` 10 or 100? x = {}", x);

    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}.", "Claire", "Noah");
    println!("{s} {v} {o}", o = "dog", s = "fox", v = "jump over");
    println!(
        "there are {:b} types of people: those who know binary and those who don't.",
        2
    );

    let pi = 3.14159265;

    macro_rules! pi_print {
        () => {
            println!("Pi is roughly {:.3}", pi);
        };
    }

    pi_print!();

    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print too!", Deep(Structure(7)));

    let name = "Claire";
    let age = 21;
    let claire = Person { name, age };
    println!("{:#?}", claire);
}
