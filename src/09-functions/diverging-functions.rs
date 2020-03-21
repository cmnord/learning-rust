#![feature(never_type)]

fn foo() -> ! {
    panic!("This call never returns.");
}

fn some_fn() {
    ()
}

fn sum_odd_numbers(up_to: u32) -> u32 {
    let mut acc = 0;
    for i in 0..up_to {
        // the return type of this match must be u32 because of the type of the
        // `addition` variable
        let addition: u32 = match i % 2 == 1 {
            true => i,
            false => continue,
        };
        acc += addition;
    }
    acc
}

fn main() {
    let _a: () = some_fn();
    println!("This function returns and you can see this line.");

    println!(
        "Sum of odd numbers up to 9 (exclusive) = {}",
        sum_odd_numbers(9)
    );

    let _x: ! = panic!("This call never returns.");
    println!("You will never see this line!");
}
