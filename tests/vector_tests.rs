mod def;

#[cfg(test)]
mod vector_tests
{
    use zs_core::Vector;

    use crate::def::*;
    // use zs_core;
    
    const A: [[i32; 7]; 7] = [
        [6, 9, 13, 4, 50, 0, 2],
        [4, 9, 16, 32, 105, 2, 1],
        [1, 2, 3, 4, 5, 6, 7],
        [0, 0, 7, 9, 8, 4, 3],
        [12, 13, 14, 15, 16, 17, 18],
        [11, 10, 3, 4, 56, 3, 7],
        [0, 1, 3, 4, 5, 7, 8]
    ];
    
    #[test]
    fn construct_add()
    {
        let mut t = Vector7::<f32>::new(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        let n = Vector7::<f32>::single(4.0);
        let x = t + n;
        t += n;
        assert_eq!(x, t);
        assert_eq!(x, Vector7::from([4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]));
    }
    #[test]
    fn units()
    {
        let mat = Matrix7::<usize>::identity();
        
        assert_eq!(Vector7::<usize>::unit_i0(), mat.row0());
        assert_eq!(Vector7::<usize>::unit_i1(), mat.row1());
        assert_eq!(Vector7::<usize>::unit_i2(), mat.row2());
        assert_eq!(Vector7::<usize>::unit_i3(), mat.row3());
        assert_eq!(Vector7::<usize>::unit_i4(), mat.row4());
        assert_eq!(Vector7::<usize>::unit_i5(), mat.row5());
        assert_eq!(Vector7::<usize>::unit_i6(), mat.row6());
    }
    
    #[test]
    fn multiply_mat()
    {
        let r = Vector7::<i32>::new(143, 165, 191, 243, 758, 190, 225);
        
        let mat = Matrix7::from(A);
        let vec = Vector7::<i32>::new(1, 2, 3, 4, 5, 6, 7);
        
        let mult = vec * mat;
        assert_eq!(r, mult);
    }
    
    #[test]
    fn dot()
    {
        let l = Vector7::new(1, -4, 3, 7, 2, -19, 5);
        let r = Vector7::new(-3, 14, 15, 2, -7, 5, 4);
        
        let res = l.dot(r);
        let exp = -89;
        assert_eq!(res, exp);
    }
    #[test]
    fn squared_length()
    {
        let l = Vector7::new(1, -4, 3, 7, 2, -19, 5);
        
        let res = l.squared_length();
        let exp = 465;
        assert_eq!(res, exp);
    }
    #[test]
    fn squared_dist()
    {
        let l = Vector7::new(1, -4, 3, 7, 2, -19, 5);
        let r = Vector7::new(5, 6, 9, 2, 14, -3, 7);
        
        let res = l.squared_distance(r);
        let exp = 581;
        assert_eq!(res, exp);
    }
    #[test]
    fn lerp()
    {
        let l = Vector7::new(1, -4, 3, 7, 2, -19, 5);
        let r = Vector7::new(5, 6, 9, 2, 14, -3, 7);
        
        let res = l.lerp(r, 3);
        let exp = Vector7::new(13, 26, 21, -8, 38, 29, 11);
        assert_eq!(res, exp);
    }
}
