use zs_macros::{generate_matrix_constructors, generate_matrix_square, generate_vector, MatMult};

#[generate_vector(7)]
pub struct Vector7 {}

#[generate_matrix_square(7, Vector7)]
#[derive(MatMult)]
#[mult_args(7, 7, Matrix7, Matrix7)]
#[generate_matrix_constructors(Vector7)]
pub struct Matrix7 {}