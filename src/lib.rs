mod vectors;

#[cfg(not(feature = "use_double"))]
pub type real = f32;
#[cfg(feature = "use_double")]
pub type real = f64;

pub use crate::vectors::*;