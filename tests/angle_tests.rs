#[cfg(test)]
mod angle_tests
{
    use std::f32::consts;

    use zs_core::{Degree, Radian};

    #[test]
    fn rad_deg()
    {
        let rad = Radian::<f32>::from(consts::FRAC_PI_6);
        let deg: Degree::<f32> = rad.into();
        
        let exp = Degree::<f32>::from(30.0);
        assert_eq!(deg, exp);
    }
    #[test]
    fn deg_rad()
    {
        let deg = Degree::<f32>::from(45.0);
        let rad: Radian::<f32> = deg.into();
        
        let exp = Radian::<f32>::from(consts::FRAC_PI_4);
        assert_eq!(rad, exp);
    }
    // #[test]
    // fn deg_gra()
    // {
    //     let deg = Degree::<f32>::from(90.0);
    //     let gra: Gradian::<f32> = deg.into();
        
    //     let exp = Gradian::<f32>::from(100.0);
    //     assert_eq!(gra, exp);
    // }
}