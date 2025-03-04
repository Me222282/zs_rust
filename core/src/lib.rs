pub use num_traits::*;
use std::fmt::{Display, Formatter, Result};
use std::error::Error;

// pub trait VectorType<S: Num>
// {
//     pub fn dot(self, )
// }

#[derive(Debug, Copy, Clone)]
pub struct SliceToVectorError { }
impl Display for SliceToVectorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result
    {
        #[allow(deprecated)]
        return self.description().fmt(f);
    }
}
impl Error for SliceToVectorError {
    #[allow(deprecated)]
    fn description(&self) -> &str
    {
        "could not convert slice to vector"
    }
}