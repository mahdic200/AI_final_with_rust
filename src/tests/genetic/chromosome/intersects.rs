use crate::genetic::chromosome::Chromosome;

#[test]
pub fn intersects_on_best() {
    let gens = vec![4, 2, 7, 5, 1, 8, 6, 3];
    let instance = Chromosome::new(gens);
    assert_eq!(instance.intersects(), 0.0);
}


#[test]
pub fn intersects_on_bad() {
    let gens = vec![4, 5, 6, 1, 2, 3, 8, 7];
    let instance = Chromosome::new(gens);
    assert_eq!(instance.intersects(), 10.0);
}
