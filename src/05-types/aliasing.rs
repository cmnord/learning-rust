type NanoSecond = u64;
type Inch = u64;

#[allow(non_camel_case_types)]
type u64_t = u64;

fn main() {
    // Aliases do NOT provide extra type safety!
    // Aliases are NOT new types.
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    print!(
        "{}ns + {}in = {} ??",
        nanoseconds,
        inches,
        nanoseconds + inches
    );

    // Aliases only reduce boilerplate.
}
