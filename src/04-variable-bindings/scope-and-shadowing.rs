fn main() {
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);

        // shadowing other binding
        let long_lived_binding = 5_f32;
        println!("inner long: {}", long_lived_binding);
    }

    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);

    // also shadows previous binding
    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding);
}
