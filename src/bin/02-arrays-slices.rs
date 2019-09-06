use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first elt of slice {}", slice[0]);
    println!("slice has {} elts", slice.len());
}

fn main() {
    let xs = [1, 2, 3, 4, 5];

    let ys = [0; 500];

    println!("first elt of arr {}", xs[0]);
    println!("second elt of arr {}", xs[1]);

    println!("array size: {}", xs.len());
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);

    // out of bounds indexing
    // println!("{}", xs[5]);
}
