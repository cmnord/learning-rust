extern crate rary;

// compile with `rustc mod.rs --extern rary=library.rlib`
fn main() {
    rary::public_function();

    // Error! `private_function` is private
    // rary::private_function();

    rary::indirect_access();
}
