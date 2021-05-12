use std::fmt;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n({}, {})\n({}, {})", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: &Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

fn main() {
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // this doesn't work'
    // println!("too long: {:?}", too_long_tuple);
    let pair = (1, true);
    println!("pair is {:?}", pair);
    let one_element = (true,);
    println!("one element tuple: {:?}", one_element);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix debug format: {:?}", matrix);
    println!("Matrix display format: {}", matrix);
    println!("Matrix transpose: {}", transpose(&matrix));
}
