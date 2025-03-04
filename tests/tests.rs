#[cfg(test)]
mod tests {
    use zs_macros::{generate_matrix, generate_vector};
    // use zs_core;
    
    #[generate_vector(7)]
    struct Vector7 {}
    
    #[generate_matrix(7, 7)]
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
    fn matrix_construct()
    {
        let t = Vector7::<f32>::new(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        let n = Vector7::<f32>::single(4.0);
        let x = Vector7::from([4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
        
        let q = Matrix7::<f32>::new(t, n, x, t, n, x, n);
        let Q = Matrix7::<f32>::from([
            [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            [4.0; 7],
            [4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
            [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            [4.0; 7],
            [4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
            [4.0; 7]
        ]);
        
        assert_eq!(q, Q);
    }
    #[test]
    fn matrix_eq()
    {
        let t = Vector7::<f32>::new(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        let n = Vector7::<f32>::single(4.0);
        let x = Vector7::from([4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
        
        let q = Matrix7::<f32>::new(t, n, x, t, n, x, n);
        let Q = Matrix7::<f32>::new(t, n, x, t, n, x, n);
        assert_eq!(q != Q, false);
        assert_eq!(q == Q, true);
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
                array[x][y] = x * y + x + y;
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
}