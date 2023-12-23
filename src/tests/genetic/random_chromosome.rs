use crate::genetic::Genetic;

#[test]
fn length() {
    let genetic = Genetic::new(8, 2, 0.1, 10);
    assert_eq!(genetic.random_chromosome().gens.len(), 8);
}

#[test]
fn valid_range_for_gens() {
    let genetic = Genetic::new(8, 2, 0.1, 10);
    for gen in &genetic.random_chromosome().gens {
        if *gen < 1 || *gen > 8 {
            assert!(false);
        }
    }
    assert!(true);
}

#[test]
fn unique_gens() {
    let genetic = Genetic::new(8, 2, 0.1, 10);
    let mut conflicts = 0;
    for gen in &genetic.random_chromosome().gens {
        let mut count: u8 = 0;
        for gen_2 in &genetic.random_chromosome().gens {
            if gen == gen_2 {
                count += 1;
            }
        }
        if count > 1 {
            conflicts += 1;
        }
    }
    assert!(conflicts > 0);
}