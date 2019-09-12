enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

fn main() {
    // WITHOUT if let -- messy
    let optional = Some(7);

    // This is a lot of wasted space.
    match optional {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
        }
        _ => {}
    }

    // WITH if let -- cleaner

    // all have type `Option<i32>`
    let number = Some(7);
    let _letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // if `let` destructures `number` into `Some(i)`, evaluate the block.
    if let Some(i) = number {
        println!("Matched {:?}", i);
    } else {
        // Destructure failed. Change to the failure case:
        println!("Didn't match a number. Let's go with a letter!");
    };

    // altered failing condition
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    // Destructure failed. evaluate else if condition to see if the alternate
    // failure branch should be taken
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // condition evaluated false. this branch is the default:
        println!("I don't like letters. Let's go with an emoticon :)!");
    };

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Bar = b {
        println!("b is foobar");
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }
}
