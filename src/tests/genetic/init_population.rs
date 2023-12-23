use crate::genetic::Genetic;

#[test]
fn correct_pop_len() {
    let genetic = Genetic::new(8, 2, 0.1, 10);
    assert_eq!(genetic.population.borrow().len(), 2);
}