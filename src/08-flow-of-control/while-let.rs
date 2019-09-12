fn example_bad() {
    let mut optional = Some(0);

    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
            }
            // quit loop when destructure fails... but why should this be required?
            _ => {
                break;
            }
        }
    }
}

fn example_good() {
    let mut optional = Some(0);

    while let Some(i) = optional {
        // less indentation. doesn't explicitly require handling the failing case.
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
}

fn main() {
    example_bad();
    example_good();
}
