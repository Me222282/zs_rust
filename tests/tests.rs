#[cfg(test)]
mod tests {
    use zs_macros::{generate_matrix, generate_matrix_square, generate_vector};
    use num_traits::Zero;
    // use zs_core;
    
    #[generate_vector(7)]
    struct Vector7 {}
    
    #[generate_matrix_square(7)]
    struct Matrix7 {}

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
    fn vector_units()
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
    fn matrix_construct()
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
    fn matrix_eq()
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
    fn matrix_index1()
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
    fn matrix_index2()
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
    fn matrix_row()
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
        
        assert_eq!(<[usize; 7] as Into<Vector7<usize>>>::into(array[0]), mat.row0());
        assert_eq!(<[usize; 7] as Into<Vector7<usize>>>::into(array[1]), mat.row1());
        assert_eq!(<[usize; 7] as Into<Vector7<usize>>>::into(array[2]), mat.row2());
        assert_eq!(<[usize; 7] as Into<Vector7<usize>>>::into(array[3]), mat.row3());
        assert_eq!(<[usize; 7] as Into<Vector7<usize>>>::into(array[4]), mat.row4());
        assert_eq!(<[usize; 7] as Into<Vector7<usize>>>::into(array[5]), mat.row5());
        assert_eq!(<[usize; 7] as Into<Vector7<usize>>>::into(array[6]), mat.row6());
    }
    #[test]
    fn matrix_ident()
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
    fn matrix_col()
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
        
        assert_eq!(<[usize; 7] as Into<Vector7<usize>>>::into(trans[0]), mat.col0());
        assert_eq!(<[usize; 7] as Into<Vector7<usize>>>::into(trans[1]), mat.col1());
        assert_eq!(<[usize; 7] as Into<Vector7<usize>>>::into(trans[2]), mat.col2());
        assert_eq!(<[usize; 7] as Into<Vector7<usize>>>::into(trans[3]), mat.col3());
        assert_eq!(<[usize; 7] as Into<Vector7<usize>>>::into(trans[4]), mat.col4());
        assert_eq!(<[usize; 7] as Into<Vector7<usize>>>::into(trans[5]), mat.col5());
        assert_eq!(<[usize; 7] as Into<Vector7<usize>>>::into(trans[6]), mat.col6());
    }
    #[test]
    fn matrix_trans()
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
    fn matrix_zero()
    {
        let zero = Matrix7::from([[0; 7]; 7]);
        let mat = Matrix7::<usize>::zero();
        
        assert_eq!(zero, mat);
    }
}