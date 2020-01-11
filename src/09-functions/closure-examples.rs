/*
pub trait Iterator {
    // the type being iterated over.
    type Item;

    // `any` takes `&mut self`, meaning the caller may be borrowed and modified, but
    // not consumed.
    fn any<F>(&mut self, f: F) -> bool
    where
        // `FnMut` means any captured variable may at most be modified, but not consumed.
        // `Self::Item` states it takes arguments to the closure by value.
        F: FnMut(Self::Item) -> bool {}
}

pub trait Iterator {
    type Item;

    fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
    P: FnMut(&Self::Item) -> bool {}
}
*/

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let vec3 = vec![1, 2, 3];
    let vec4 = vec![4, 5, 6];

    // iter() for vecs yields &i32. Destructure to i32.
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));

    // into_iter() for vecs yields i32. No destructuring required.
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    // a reference to what is yielded is &&i32. Destructure to i32.
    println!("Find 2 in vec3: {:?}", vec3.iter().find(|&&x| x == 2));

    // A reference to what is yielded is &i32. Destructure to i32.
    println!("Find 2 in vec4: {:?}", vec4.into_iter().find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    let array3 = [1, 2, 3];
    let array4 = [4, 5, 6];

    // iter() for arrays yields &i32.
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));

    // into_iter() for arrays UNUSUALLY yields &i32...
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));

    println!("Find 2 in array1: {:?}", array3.iter().find(|&&x| x == 2));

    println!(
        "Find 2 in array2: {:?}",
        array4.into_iter().find(|&&x| x == 2)
    );
}
