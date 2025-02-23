#[cfg(test)]
mod tests {
    use zs_macros::generate_vector;
    use zs_core;
    
    #[generate_vector(7)]
    struct Vector7 {}

    #[test]
    fn it_works() {
        let t = Vector7::<f32>::new(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        let n = Vector7::<f32>::single(4.0);
        let x = t + n;
    }
}