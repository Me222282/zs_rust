mod def;

#[cfg(test)]
mod matrix_tests
{
    use num_traits::Zero;
    use zs_core::Radian;
    // use zs_core;
    
    use crate::def::*;
    
    const A: [[i32; 7]; 7] = [
        [6, 9, 13, 4, 50, 0, 2],
        [4, 9, 16, 32, 105, 2, 1],
        [1, 2, 3, 4, 5, 6, 7],
        [0, 0, 7, 9, 8, 4, 3],
        [12, 13, 14, 15, 16, 17, 18],
        [11, 10, 3, 4, 56, 3, 7],
        [0, 1, 3, 4, 5, 7, 8]
    ];
    const B: [[i32; 7]; 7] = [
        [1, 2, 3, 4, 5, 6, 7],
        [8, 9, 10, 11, 12, 13, 14],
        [15, 16, 17, 18, 19, 20, 21],
        [22, 23, 24, 25, 26, 27, 28],
        [29, 30, 31, 32, 33, 34, 35],
        [36, 37, 38, 39, 40, 41, 42],
        [43, 44, 45, 46, 47, 48, 49]
    ];
    
    #[test]
    fn construct()
    {
        let t = Vector7::<f32>::new(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        let n = Vector7::<f32>::single(4.0);
        let x = Vector7::from([4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
        
        let q = Matrix7::<f32>::new(t, n, x, t, n, x, n);
        let p = Matrix7::<f32>::from([
            [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            [4.0; 7],
            [4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
            [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            [4.0; 7],
            [4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
            [4.0; 7]
        ]);
        
        assert_eq!(q, p);
    }
    #[test]
    fn eq()
    {
        let t = Vector7::<f32>::new(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        let n = Vector7::<f32>::single(4.0);
        let x = Vector7::from([4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
        
        let q = Matrix7::<f32>::new(t, n, x, t, n, x, n);
        let p = Matrix7::<f32>::new(t, n, x, t, n, x, n);
        assert_eq!(q != p, false);
        assert_eq!(q == p, true);
    }
    
    #[test]
    fn index1()
    {
        let mut array = [0; 49];
        for x in 0..49
        {
            array[x] = x;
        }
        let mat = Matrix7::from(&array);
        
        for i in 0..49
        {
            assert_eq!(array[i], mat[i]);
        }
    }
    #[test]
    fn index2()
    {
        let mut array = [[0; 7]; 7];
        for x in 0..7
        {
            for y in 0..7
            {
                array[x][y] = x * y + 2 * x + 3 * y;
            }
        }
        let mat = Matrix7::from(&array);
        
        for x in 0..7
        {
            for y in 0..7
            {
                assert_eq!(array[x][y], mat[[x, y]]);
            }
        }
    }
    #[test]
    fn row()
    {
        let mut array = [[0; 7]; 7];
        for x in 0..7
        {
            for y in 0..7
            {
                array[x][y] = x * y + 2 * x + 3 * y;
            }
        }
        let mat = Matrix7::from(&array);
        
        assert_eq!(Vector7::<usize>::from(array[0]), mat.row0());
        assert_eq!(Vector7::<usize>::from(array[1]), mat.row1());
        assert_eq!(Vector7::<usize>::from(array[2]), mat.row2());
        assert_eq!(Vector7::<usize>::from(array[3]), mat.row3());
        assert_eq!(Vector7::<usize>::from(array[4]), mat.row4());
        assert_eq!(Vector7::<usize>::from(array[5]), mat.row5());
        assert_eq!(Vector7::<usize>::from(array[6]), mat.row6());
    }
    #[test]
    fn ident()
    {
        let mat = Matrix7::<usize>::identity();
        for x in 0..7
        {
            for y in 0..7
            {
                let value = if x == y { 1 } else { 0 };
                assert_eq!(value, mat[[x, y]]);
            }
        }
    }
    #[test]
    fn col()
    {
        let mut array = [[0; 7]; 7];
        for x in 0..7
        {
            for y in 0..7
            {
                array[x][y] = x * y + 2 * x + 3 * y;
            }
        }
        let mat = Matrix7::from(&array);
        
        let mut trans = [[0; 7]; 7];
        for x in 0..7
        {
            for y in 0..7
            {
                trans[y][x] = array[x][y];
            }
        }
        
        assert_eq!(Vector7::<usize>::from(trans[0]), mat.col0());
        assert_eq!(Vector7::<usize>::from(trans[1]), mat.col1());
        assert_eq!(Vector7::<usize>::from(trans[2]), mat.col2());
        assert_eq!(Vector7::<usize>::from(trans[3]), mat.col3());
        assert_eq!(Vector7::<usize>::from(trans[4]), mat.col4());
        assert_eq!(Vector7::<usize>::from(trans[5]), mat.col5());
        assert_eq!(Vector7::<usize>::from(trans[6]), mat.col6());
    }
    #[test]
    fn trans()
    {
        let mut array = [[0; 7]; 7];
        for x in 0..7
        {
            for y in 0..7
            {
                array[x][y] = x * y + 2 * x + 3 * y;
            }
        }
        let mat = Matrix7::from(&array);
        let mat = mat.transpose();
        
        let mut trans = [[0; 7]; 7];
        for x in 0..7
        {
            for y in 0..7
            {
                trans[y][x] = array[x][y];
            }
        }
        
        let q = Matrix7::from(&trans);
        
        assert_eq!(q, mat);
    }
    #[test]
    fn zero()
    {
        let zero = Matrix7::from([[0; 7]; 7]);
        let mat = Matrix7::<usize>::zero();
        
        assert_eq!(zero, mat);
    }
    #[test]
    fn scale()
    {
        let s = 74;
        let mat = Matrix7::<usize>::create_scale(s);
        for x in 0..7
        {
            for y in 0..7
            {
                let value = if x == y { s } else { 0 };
                assert_eq!(value, mat[[x, y]]);
            }
        }
    }
    
    #[test]
    fn multiply()
    {
        let r = [
            [1897, 1981, 2065, 2149, 2233, 2317, 2401],
            [4180, 4349, 4518, 4687, 4856, 5025, 5194],
            [812, 840, 868, 896, 924, 952, 980],
            [808, 839, 870, 901, 932, 963, 994],
            [2506, 2611, 2716, 2821, 2926, 3031, 3136],
            [2257, 2351, 2445, 2539, 2633, 2727, 2821],
            [882, 910, 938, 966, 994, 1022, 1050]
        ];
        
        let mat_a = Matrix7::from(A);
        let mat_b = Matrix7::from(B);
        let mat_r = Matrix7::from(r);
        
        let mat_mult = mat_a * mat_b;
        assert_eq!(mat_r, mat_mult);
    }
    #[test]
    fn add()
    {
        let r = [
            [7, 11, 16, 8, 55, 6, 9],
            [12, 18, 26, 43, 117, 15, 15],
            [16, 18, 20, 22, 24, 26, 28],
            [22, 23, 31, 34, 34, 31, 31],
            [41, 43, 45, 47, 49, 51, 53],
            [47, 47, 41, 43, 96, 44, 49],
            [43, 45, 48, 50, 52, 55, 57]
        ];
        
        let mat_a = Matrix7::from(A);
        let mat_b = Matrix7::from(B);
        let mat_r = Matrix7::from(r);
        
        let mat_mult = mat_a + mat_b;
        assert_eq!(mat_r, mat_mult);
    }
    #[test]
    fn sub()
    {
        let r = [
            [5, 7, 10, 0, 45, -6, -5],
            [-4, 0, 6, 21, 93, -11, -13],
            [-14, -14, -14, -14, -14, -14, -14],
            [-22, -23, -17, -16, -18, -23, -25],
            [-17, -17, -17, -17, -17, -17, -17],
            [-25, -27, -35, -35, 16, -38, -35],
            [-43, -43, -42, -42, -42, -41, -41]
        ];
        
        let mat_a = Matrix7::from(A);
        let mat_b = Matrix7::from(B);
        let mat_r = Matrix7::from(r);
        
        let mat_mult = mat_a - mat_b;
        assert_eq!(mat_r, mat_mult);
    }
    
    #[test]
    fn multiply_vec()
    {
        let r = Vector7::<i32>::new(343, 742, 140, 142, 448, 403, 150);
        
        let mat = Matrix7::from(A);
        let vec = Vector7::<i32>::new(1, 2, 3, 4, 5, 6, 7);
        
        let mult = mat * vec;
        assert_eq!(r, mult);
    }
    #[test]
    fn rotate()
    {
        Matrix7::<f32>::create_rotation_i0i1(Radian::<f32>::degrees(30_f32));
    }
}