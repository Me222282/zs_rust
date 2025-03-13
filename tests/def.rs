use zs_macros::{generate_matrix, generate_matrix_multiply, generate_vector};

#[generate_vector(7)]
pub struct Vector7 {}

#[generate_vector(6)]
pub struct Vector6 {}

#[generate_matrix(7, Vector7)]
#[matrix_square]
#[matrix_constructors(scale, 7, Vector7, 6, Vector6)]
#[matrix_constructors(trans, Vector6)]
#[matrix_constructors(rotate)]
pub struct Matrix7 {}

generate_matrix_multiply!(7, Matrix7);