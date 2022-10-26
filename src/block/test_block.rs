use super::*;

#[test]
fn test_overlapping() {
    let block0 = Block::new(10, 10, 0.1, 0.1, (0.0, 0.0));
    let block0_copy = Block::new(10, 10, 0.1, 0.1, (0.0, 0.0));
    let block1 = Block::new(10, 10, 0.1, 0.1, (1.0, 0.0));
    let block2 = Block::new(10, 10, 0.1, 0.1, (1.1, 0.0));
    let block3 = Block::new(10, 10, 0.1, 0.1, (0.9, 0.9));
    let block4 = Block::new(10, 10, 0.1, 0.1, (-0.9, -0.9));
    let block5 = Block::new(10, 10, 0.1, 0.1, (-0.9, 0.9));
    assert!(overlapping(&block0, &block0_copy));
    assert!(overlapping(&block0, &block1));
    assert!(!overlapping(&block0, &block2));
    assert!(overlapping(&block0, &block3));
    assert!(overlapping(&block0, &block4));
    assert!(overlapping(&block0, &block5));
}
