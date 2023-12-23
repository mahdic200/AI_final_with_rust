use std::cell::RefCell;

#[derive(Debug)]
pub struct Chromosome {
    pub gens: Vec<usize>,
    fitness: RefCell<Option<f32>>,
    fitness_ratio: RefCell<Option<f32>>,
    probability_range: RefCell<Option<[f32; 2]>>,
}

impl Chromosome {
    pub fn new(gens: Vec<usize>) -> Chromosome {
        let new_instance = Chromosome {
            gens,
            fitness: RefCell::new(None),
            fitness_ratio: RefCell::new(None),
            probability_range: RefCell::new(None),
        };
        new_instance.fitness();
        new_instance
    }

    pub fn intersects(&self) -> f32 {
        let mut fitness: f32 = 0.0;
        for i in 0..self.gens.len() - 1 {
            for j in &i + 1..self.gens.len() {
                if self.gens[i] == self.gens[j] {
                    fitness += 1.0;
                }
            }
        }
        fitness
    }

    pub fn fitness(&self) -> f32 {
        let mut self_fitness = self.fitness.borrow_mut();
        if let Some(fitness) = *self_fitness {
            fitness
        }
        else {
            let new_fitness = 1.0 / (self.intersects() + self.epsilon());
            self_fitness.replace(new_fitness);
            new_fitness
        }
    }

    pub fn fitness_ratio(&self, summation: &f32) -> f32 {
        let mut self_fitness_ratio = self.fitness_ratio.borrow_mut();
        if let Some(fitness_ratio) = *self_fitness_ratio {
            fitness_ratio
        }
        else {
            self_fitness_ratio.replace(&self.fitness() / summation);
            &self.fitness() / summation
        }
    }

    pub fn set_probability_range(&self, range: [f32; 2]) -> () {
        self.probability_range.borrow_mut().replace(range);
    }

    pub fn is_chosen(&self, number: f32) -> bool {
        if let Some(_) = *self.probability_range.borrow() {}
        else {
            panic!("probability range must be valid or not None in Chromosome structure !");
        }
        if number >= self.probability_range.borrow().unwrap()[0] && number < self.probability_range.borrow().unwrap()[1] {
            return true;
        }
        else {
            return false;
        }
    }

    pub fn epsilon(&self) -> &f32 {
        &0.00000001
    }

    pub fn print(&self) {
        #[derive(Debug)]
        #[allow(unused)]
        struct Print<T, U> {
            gens: T,
            intersects: U,
            fitness: U,
        }
        let print = Print {
            gens: &self.gens,
            intersects: &self.intersects(),
            fitness: &self.fitness(),
        };

        println!("{:?}", print);
    }
}

