pub struct Radian<S>(S);
pub struct Degree<S>(S);
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