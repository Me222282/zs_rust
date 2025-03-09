use zs_macros::{generate_matrix_constructors, generate_matrix_square, generate_vector, MatMult, VecMult};

#[generate_vector(7)]
#[derive(VecMult)]
#[mult_vec_args(Matrix7)]
pub struct Vector7 {}

#[generate_vector(6)]
pub struct Vector6 {}

#[generate_matrix_square(7, Vector7)]
#[derive(MatMult)]
#[mult_mat_args]
#[generate_matrix_constructors(Vector7, Vector6)]
pub struct Matrix7 {}