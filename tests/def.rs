use zs_macros::{generate_matrix, generate_vector};

#[generate_vector(7)]
#[vector_mult(Matrix7)]
pub struct Vector7 {}

#[generate_vector(6)]
pub struct Vector6 {}

#[generate_matrix(7, Vector7)]
#[matrix_square]
#[matrix_mult]
#[matrix_constructors(scale, Vector7)]
#[matrix_constructors(trans, Vector6)]
pub struct Matrix7 {}