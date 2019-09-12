#![allow(unreachable_code)]

fn main() {
    let mut count = 0u32;

    println!("let's count until infinity!");

    loop {
        count += 1;

        if count == 3 {
            println!("three");

            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("ok, that's enough");

            break;
        }
    }

    'outer: loop {
        println!("entered the outer loop");

        'inner: loop {
            println!("entered the inner loop");
            // this would break only the inner loop
            // break;

            // breaks the outer loop
            break 'outer;
        }

        println!("this point will never be reached");
    }

    println!("exited outer loop");

    let mut counter2 = 0;

    // like retrying an operation until it succeeds
    let result = loop {
        counter2 += 1;

        if counter2 == 10 {
            // break can return a value
            break counter2 * 2;
        }
    };

    assert_eq!(result, 20);
}
