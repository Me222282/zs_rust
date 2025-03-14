#[cfg(test)]
mod line_tests
{
    use zene_structs::Line2;

    #[test]
    fn perpendicular()
    {
        let a = Line2::<f32>::new([4.0, 2.0].into(), [1.0, 1.0].into());
        let b = Line2::<f32>::new([-2.0, 4.0].into(), [1.0, 1.0].into());
        
        let a = a.perpendicular();
        
        assert!(a.geometrically_equals(b));
    }
    
    #[test]
    fn geometrically_equals()
    {
        let a = Line2::<f32>::new([4.0, 2.0].into(), [1.0, 1.0].into());
        let b = Line2::<f32>::new([-4.0, -2.0].into(), [1.0, 1.0].into());
        
        let c = Line2::<f32>::new([4.0, 2.0].into(), [2.0, 1.0].into());
        let d = Line2::<f32>::new([4.0, 2.0].into(), [5.0, 3.0].into());
        
        assert!(a.geometrically_equals(b));
        assert!(!a.geometrically_equals(c));
        assert!(!b.geometrically_equals(c));
        assert!(a.geometrically_equals(d));
        assert!(d.geometrically_equals(b));
    }
    
    #[test]
    fn intersects()
    {
        let p = [1.2, 1.5].into();
        
        let a = Line2::<f32>::new([4.0, 2.0].into(), p);
        let b = Line2::<f32>::new([17.0, -3.0].into(), p);
        let c = Line2::<f32>::new([4.0, 2.0].into(), p * 2.0);
        
        assert_eq!(a.intersects(b), Some(p));
        assert_eq!(a.intersects(c), None);
    }
    
    #[test]
    fn contains()
    {
        let a = Line2::<f32>::new([4.0, 2.0].into(), [1.0, 1.0].into());
        let p1 = [5.0, 3.0].into();
        let p2 = [-57.0, 13.0].into();
        
        assert!(a.contains(p1));
        assert!(!a.contains(p2));
    }
}