use super::*;

#[test]
fn test_intersects() {
    let a = (0.0, 0.0);
    let b = (1.0, 1.0);
    let c = (0.0, 1.0);
    let d = (1.0, 0.0);
    assert!(intersects(a, b, c, d));
    assert!(!intersects(a, d, b, c));
}
