fn main() {
    // a..b is range [a, b)
    // a..=b is range [a, b]
    for n in 1..101 {
        // same as 1..=100
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    demo_iter();
    demo_iter_mut();
}

fn demo_iter() {
    let names = vec!["Alice", "Bob", "Ferris"];

    // into_iter consumes the collection so that on each iteration the exact data is
    // provided. Once the collection has been consumed it is no longer reusable. it's
    // been 'moved' w/in the loop.
    // for name in names.into_iter() {

    // iter borrows each element of the collection through each iteration.
    // thus, collection is untouched and available for reuse after loop.
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}

fn demo_iter_mut() {
    let mut names = vec!["Alice", "Bob", "Ferris"];

    // iter_mut mutably borrows each elt of the collection, allowing for collection
    // to be modified in place.
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}
