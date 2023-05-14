use std::fmt::{self, Display, Formatter};

pub fn reverse_pair(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;

    return (bool_param, int_param);
}

#[derive(Debug)]
pub struct Matrix(pub f32, pub f32, pub f32, pub f32);

pub fn matrix_transpose(mat: Matrix) -> Matrix {
    let (a11, a12, a21, a22) = (mat.0, mat.1, mat.2, mat.3);

    return Matrix(a11, a21, a12, a22);
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        return write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3);
    }
}

pub fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}
