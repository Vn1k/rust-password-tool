pub fn entropy(long: f64, pool: f64) -> f64 {
    long * pool.log2()
}

#[test]
fn test_entropy() {
    use super::*;

    let result = entropy(6.0, 26.0);

    println!("{}", result);
}