use crate::genetic::chromosome::Chromosome;

#[test]
pub fn fitness_on_best() {
    let gens = vec![4, 2, 7, 5, 1, 8, 6, 3];
    let instance = Chromosome::new(gens);
    assert_eq!(instance.fitness(), 1.0 / (0.0 + instance.epsilon()));
}

#[test]
pub fn fitness_on_bad() {
    let gens = vec![4, 5, 6, 1, 2, 3, 8, 7];
    let instance = Chromosome::new(gens);
    assert_eq!(instance.fitness(), 1.0 / (10.0 + instance.epsilon()));
}