use crate::genetic::Genetic;
use crate::genetic::enums::CrossoverType;

#[test]
fn correct_pop_len() {
    let genetic = Genetic::new(8, 2, 0.1, 10, CrossoverType::TwoPoint);
    assert_eq!(genetic.population.borrow().len(), 2);
}