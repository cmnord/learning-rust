use std::fmt;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;

    (boolean, integer)
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn main() {
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    println!("long tuple 1st val: {}", long_tuple.0);
    println!("long tuple 2nd val: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8, -2i16));

    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9);
    println!("too long tuple: {:?}", too_long_tuple);

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("reversed pair: {:?}", reverse(pair));

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);

    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}
