fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cubed = x_squared * x;

        // without ;, this is "returned" and assigned to y.
        x_cubed + x_squared + x
    };

    let z = {
        // with ;, this is not returned. z is nil
        // compiler warns unused :')
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
