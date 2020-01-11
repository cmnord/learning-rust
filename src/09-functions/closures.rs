// closures are also called lambdas

fn main() {
    // increment via closures and functions.
    fn function(i: i32) -> i32 {
        i + 1
    }

    // Closures are anonymous. here, we bind them to references.
    // Annotation is identical to fn annotation, but optional, as are the {} wrapping
    // the body.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // a closure taking no arguments.
    let one = || 1;
    println!("closure returning one: {}", one());

    closure_capture();
    closure_as_input_parameters();
    closure_type_anonymity();
    closure_input_functions();
    closure_output_functions();
}

fn closure_capture() {
    // Closures can capture references.
    use std::mem;

    let color = "green";

    let print = || println!("`color`: {}", color);

    print();
    print();

    let mut count = 0;

    // mut is required bc &mut is stored insire.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();
    inc();

    let reborrow = &mut count;
    println!("reborrow: {}", reborrow);

    // a non-copy type
    let movable = Box::new(3);

    // mem::drop requires `T`
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    consume();
    // cannot be called twice
    // consume();

    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // Variables were already moved. Can't reuse them now.
    // println!("There are {} elements in vec", haystack.len());
}

fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

fn closure_as_input_parameters() {
    // When taking closure as an input parameter, the complete type must be annotated
    // using a `trait`. In order of decreasing restriction:
    // - Fn: captures reference &T
    // - FnMut: captures reference &mut T
    // - FnOnce: captures by value T
    // Compiler will capture variables in the least restrictive manner possible.
    use std::mem;

    let greeting = "hello";
    // non-copy type.
    // `to_owned` creates owned data from borrowed one.
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captures by mutable reference. Now
        // requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}", farewell);
        println!("Now I can sleep. zzzzz");

        // manually calling drop forces `farewell` to be captures by value. Now
        // requires `FnOnce`.
        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}

fn apply2<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn closure_type_anonymity() {
    let x = 7;
    let print = || println!("{}", x);
    apply2(print);
}

// function that calls closure as a parameter
fn call_me<F: Fn()>(f: F) {
    f();
}

fn function() {
    println!("I'm a function!");
}

fn closure_input_functions() {
    let closure = || println!("I'm a closure!");
    // both closures and functions can be used as arguments!
    call_me(closure);
    call_me(function);
}

fn create_fn() -> Box<dyn Fn()> {
    let text = "Fn".to_owned();
    Box::new(move || println!("This is a: {}", text))
}

fn create_fnmut() -> Box<dyn FnMut()> {
    let text = "FnMut".to_owned();
    Box::new(move || println!("This is a: {}", text))
}

// problematic: Rust only supports returning non-generic types rn.
// Get around this via boxing.
fn closure_output_functions() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();

    fn_plain();
    fn_mut();
}
