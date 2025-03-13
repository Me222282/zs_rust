mod float_rad;
mod num_rad;
mod float_deg;
mod num_deg;
mod float_gra;
mod num_gra;

// pub use float_rad::*;
// pub use num_rad::*;
// pub use float_deg::*;
// pub use num_deg::*;
// pub use float_gra::*;
// pub use num_gra::*;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct Radian<S>(S);
#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct Degree<S>(S);
#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct Gradian<S>(S);

const RAD_DEG_32: f32 = 180.0 / std::f32::consts::PI;
const RAD_GRA_32: f32 = 200.0 / std::f32::consts::PI;
const DEG_RAD_32: f32 = std::f32::consts::PI / 180.0;
const DEG_GRA_32: f32 = 200.0 / 180.0;
const GRA_RAD_32: f32 = std::f32::consts::PI / 200.0;
const GRA_DEG_32: f32 = 180.0 / 200.0;

const RAD_DEG_64: f64 = 180.0 / std::f64::consts::PI;
const RAD_GRA_64: f64 = 200.0 / std::f64::consts::PI;
const DEG_RAD_64: f64 = std::f64::consts::PI / 180.0;
const DEG_GRA_64: f64 = 200.0 / 180.0;
const GRA_RAD_64: f64 = std::f64::consts::PI / 200.0;
const GRA_DEG_64: f64 = 180.0 / 200.0;

impl Radian<f32>
{
    pub const fn degrees(value: f32) -> Self
    {
        Self(value * DEG_RAD_32)
    }
    pub const fn gradians(value: f32) -> Self
    {
        Self(value * GRA_RAD_32)
    }
}
impl Radian<f64>
{
    pub const fn degrees(value: f64) -> Self
    {
        Self(value * DEG_RAD_64)
    }
    pub const fn gradians(value: f64) -> Self
    {
        Self(value * GRA_RAD_64)
    }
}
impl<S> From<S> for Radian<S>
{
    fn from(value: S) -> Self
    {
        Self(value)
    }
}
impl From<Degree<f32>> for Radian<f32>
{
    fn from(value: Degree<f32>) -> Self
    {
        Self(value.0 * DEG_RAD_32)
    }
}
impl From<Degree<f64>> for Radian<f64>
{
    fn from(value: Degree<f64>) -> Self
    {
        Self(value.0 * DEG_RAD_64)
    }
}
impl From<Gradian<f32>> for Radian<f32>
{
    fn from(value: Gradian<f32>) -> Self
    {
        Self(value.0 * GRA_RAD_32)
    }
}
impl From<Gradian<f64>> for Radian<f64>
{
    fn from(value: Gradian<f64>) -> Self
    {
        Self(value.0 * GRA_RAD_64)
    }
}
impl Into<f32> for Radian<f32>
{
    fn into(self) -> f32
    {
        self.0
    }
}
impl Into<f64> for Radian<f64>
{
    fn into(self) -> f64
    {
        self.0
    }
}

impl Degree<f32>
{
    pub const fn radians(value: f32) -> Self
    {
        Self(value * RAD_DEG_32)
    }
    pub const fn gradians(value: f32) -> Self
    {
        Self(value * GRA_DEG_32)
    }
}
impl Degree<f64>
{
    pub const fn radians(value: f64) -> Self
    {
        Self(value * RAD_DEG_64)
    }
    pub const fn gradians(value: f64) -> Self
    {
        Self(value * GRA_DEG_64)
    }
}
impl<S> From<S> for Degree<S>
{
    fn from(value: S) -> Self
    {
        Self(value)
    }
}
impl From<Radian<f32>> for Degree<f32>
{
    fn from(value: Radian<f32>) -> Self
    {
        Self(value.0 * RAD_DEG_32)
    }
}
impl From<Radian<f64>> for Degree<f64>
{
    fn from(value: Radian<f64>) -> Self
    {
        Self(value.0 * RAD_DEG_64)
    }
}
impl From<Gradian<f32>> for Degree<f32>
{
    fn from(value: Gradian<f32>) -> Self
    {
        Self(value.0 * GRA_DEG_32)
    }
}
impl From<Gradian<f64>> for Degree<f64>
{
    fn from(value: Gradian<f64>) -> Self
    {
        Self(value.0 * GRA_DEG_64)
    }
}
impl Into<f32> for Degree<f32>
{
    fn into(self) -> f32
    {
        self.0
    }
}
impl Into<f64> for Degree<f64>
{
    fn into(self) -> f64
    {
        self.0
    }
}

impl Gradian<f32>
{
    pub const fn radians(value: f32) -> Self
    {
        Self(value * RAD_GRA_32)
    }
    pub const fn degrees(value: f32) -> Self
    {
        Self(value * DEG_GRA_32)
    }
}
impl Gradian<f64>
{
    pub const fn radians(value: f64) -> Self
    {
        Self(value * RAD_GRA_64)
    }
    pub const fn degrees(value: f64) -> Self
    {
        Self(value * DEG_GRA_64)
    }
}
impl<S> From<S> for Gradian<S>
{
    fn from(value: S) -> Self
    {
        Self(value)
    }
}
impl From<Radian<f32>> for Gradian<f32>
{
    fn from(value: Radian<f32>) -> Self
    {
        Self(value.0 * RAD_GRA_32)
    }
}
impl From<Radian<f64>> for Gradian<f64>
{
    fn from(value: Radian<f64>) -> Self
    {
        Self(value.0 * RAD_GRA_64)
    }
}
impl From<Degree<f32>> for Gradian<f32>
{
    fn from(value: Degree<f32>) -> Self
    {
        Self(value.0 * DEG_GRA_32)
    }
}
impl From<Degree<f64>> for Gradian<f64>
{
    fn from(value: Degree<f64>) -> Self
    {
        Self(value.0 * DEG_GRA_64)
    }
}
impl Into<f32> for Gradian<f32>
{
    fn into(self) -> f32
    {
        self.0
    }
}
impl Into<f64> for Gradian<f64>
{
    fn into(self) -> f64
    {
        self.0
    }
}