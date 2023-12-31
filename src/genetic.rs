pub mod chromosome;
pub mod enums;

use rand::Rng;
use std::rc::Rc;
use std::cell::RefCell;
use enums::CrossoverType;
use crate::genetic::chromosome::Chromosome;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Genetic {
    chromosome_length: usize,
    population_size: u32,
    per_mutation: f32,
    pub population: RefCell<Vec<Rc<Chromosome>>>,
    pub maxiter: u32,
    crossover_type: CrossoverType,
}

impl Genetic {
    pub fn new(chromosome_length: usize, population_size: u32, per_mutation: f32, maxiter: u32, crossover_type: CrossoverType) -> Genetic {
        if per_mutation > 1.0 {
            panic!("per_mutation argument could not be greater than 1 !");
        }
        let genetic = Genetic {
            chromosome_length,
            population_size,
            per_mutation,
            population: RefCell::new(vec![]),
            maxiter,
            crossover_type,
        };
        genetic.init_population();
        genetic
    }

    fn init_population(&self) -> () {
        // for _i in 1..=self.population_size {
        //     self.population.borrow_mut().push(Rc::clone(&self.random_chromosome()));
        // }
        let dataset: Vec<Vec<usize>> = vec![
            vec![5, 5, 15, 15, 2, 2, 7, 7, 17, 17],
            vec![1, 15, 15, 25, 25, 25, 12, 17, 22, 3],
            vec![10, 10, 10, 20, 20, 7, 2, 12, 12, 3],
            vec![1,10,15,15,2,2,7,7,12,22],
            vec![5,5,20,25,20,25,12,12,17,3],
            vec![10,10,20,15,25,25,2,7,22,22],
            vec![5,5,15,20,20,7,7,7,12,22],
            vec![10,15,15,25,20,2,2,12,17,17],
            vec![10,10,20,25,25,2,7,12,12,17],
            vec![5,15,20,20,2,2,2,22,22,17],
        ];
        for item in dataset {
            self.population.borrow_mut().push(Rc::clone(&Rc::new(Chromosome::new(item))));
        }
    }

    pub fn random_chromosome(&self) -> Rc<Chromosome> {
        let mut rand_vec: Vec<usize> = vec![];
        for _ in 1..=self.chromosome_length {
            let rn: usize = rand::thread_rng().gen_range(0..=25);
            rand_vec.push(rn);
        }

        let chromosome = Chromosome::new(rand_vec);

        Rc::new(chromosome)
    }

    fn fitness_summation(&self) -> f32 {
        let mut sum: f32 = 0.0;
        for chromosome in &*self.population.borrow() {
            sum += &chromosome.fitness();
        }
        for chromosome in &*self.population.borrow() {
            chromosome.fitness_ratio(&sum);
        }
        sum
    }

    fn init_probability_range(&self) -> () {

        let mut temp: f32 = 0.0;
        for chromo in &*self.population.borrow() {
            let temp_ = temp + chromo.fitness_ratio(&1.0);
            chromo.set_probability_range([temp, temp_]);
            temp = temp_;
        }
    }

    pub fn parent_selection(&self) -> Vec<Rc<Chromosome>> {
        self.fitness_summation();

        self.init_probability_range();

        let mut new_parents: Vec<Rc<Chromosome>> = vec![];
        while new_parents.len() < self.population_size as usize {
            for chromo in &*self.population.borrow() {
                let rand_no = rand::thread_rng().gen_range(0.0..1.0);
                if chromo.is_chosen(rand_no) {
                    new_parents.push(Rc::clone(chromo));
                    break;
                }
            }
        }
        
        new_parents
    }

    fn one_point_crossover(&self, parent_1: Rc<Chromosome>, parent_2: Rc<Chromosome>) -> (Rc<Chromosome>, Rc<Chromosome>) {
        let parent_1 = &parent_1.gens;
        let parent_2 = &parent_2.gens;

        let crossover_point: usize = (rand::thread_rng().gen_range(0..100) % self.chromosome_length) as usize;

        let new_child_1: Vec<usize> = [&parent_1[..crossover_point], &parent_2[crossover_point..]].concat();
        let new_child_2: Vec<usize> = [&parent_2[..crossover_point], &parent_1[crossover_point..]].concat();


        let new_child_1 = Chromosome::new(new_child_1);
        let new_child_2 = Chromosome::new(new_child_2);

        (Rc::new(new_child_1), Rc::new(new_child_2))
    }

    fn two_point_crossover(&self, parent_1: Rc<Chromosome>, parent_2: Rc<Chromosome>) -> (Rc<Chromosome>, Rc<Chromosome>) {
        let parent_1 = &parent_1.gens;
        let parent_2 = &parent_2.gens;

        let mut ind_1: usize = (rand::thread_rng().gen_range(0..100) % self.chromosome_length) as usize;
        let mut ind_2: usize = (rand::thread_rng().gen_range(0..100) % (self.chromosome_length - ind_1)) + ind_1 as usize;

        if ind_2 < ind_1 {let t = ind_2;ind_2 = ind_1;ind_1 = t;}

        let mut new_child_1: Vec<usize> = Vec::new();
        let mut new_child_2: Vec<usize> = Vec::new();
        for _i in 0..self.chromosome_length {
            new_child_1.push(0);
            new_child_2.push(0);
        }

        for i in ind_1..=ind_2 {
            new_child_1[i] = parent_1[i];
            new_child_2[i] = parent_2[i];
        }


        let new_child_1 = Chromosome::new(new_child_1);
        let new_child_2 = Chromosome::new(new_child_2);

        (Rc::new(new_child_1), Rc::new(new_child_2))
    }

    fn uniform_crossover(&self, parent_1: Rc<Chromosome>, parent_2: Rc<Chromosome>) -> (Rc<Chromosome>, Rc<Chromosome>) {
        
        let length = parent_1.gens.len();
        let parent_1 = &parent_1.gens;
        let parent_2 = &parent_2.gens;

        let mut child1 = Vec::with_capacity(length);
        let mut child2 = Vec::with_capacity(length);

        for i in 0..length {
            let rand_bit = rand::random::<bool>();
            if rand_bit {
            child1.push(parent_1[i]);
            child2.push(parent_2[i]);
            } else {
            child1.push(parent_2[i]);
            child2.push(parent_1[i]);
            }
        }

        let child1 = Chromosome::new(child1);
        let child2 = Chromosome::new(child2);

        (Rc::new(child1), Rc::new(child2))
    }

    fn crossover(&self, parent_1: Rc<Chromosome>, parent_2: Rc<Chromosome>) -> (Rc<Chromosome>, Rc<Chromosome>) {
        match self.crossover_type {
            CrossoverType::OnePoint => self.one_point_crossover(parent_1, parent_2),
            CrossoverType::TwoPoint => self.two_point_crossover(parent_1, parent_2),
            CrossoverType::Uniform => self.uniform_crossover(parent_1, parent_2),
        }
    }

    fn recombination(&self, parents: &Vec<Rc<Chromosome>>) -> Vec<Rc<Chromosome>> {
        let mut offsprings: Vec<Rc<Chromosome>> = Vec::new();
        for i in (0..(parents.len() - 1)).step_by(2) {
            let (child1, child2) =
            self.crossover(
                Rc::clone(&parents[i]),
                Rc::clone(&parents[i + 1])
            );
            offsprings.push(Rc::clone(&child1));
            offsprings.push(Rc::clone(&child2));
        }
        
        let offsprings = offsprings;
        offsprings
    }

    fn swap_mutation(&self, chromosome: &Rc<Chromosome>) -> Rc<Chromosome> {
        let mut new_gens: Vec<usize> = chromosome.gens.clone();
        if rand::thread_rng().gen_range(0.0..1.0) <= self.per_mutation {
            let i = rand::thread_rng().gen_range(0..new_gens.len());
            let j = rand::thread_rng().gen_range(0..new_gens.len());
            let t = new_gens[i];
            new_gens[i] = new_gens[j];
            new_gens[j] = t;
        }

        let chromosome = Chromosome::new(new_gens);
        Rc::new(chromosome)
    }

    fn mutation(&self, offsprings: Vec<Rc<Chromosome>>) -> Vec<Rc<Chromosome>> {
        let mut offsprings = offsprings;
        for i in 0..offsprings.len() {
            offsprings[i] = self.swap_mutation(&offsprings[i]);
        }
        offsprings
    }

    pub fn maximum_fitness(&self, population: &Vec<Rc<Chromosome>>) -> (usize, f32) {
        let mut max_i = 0;
        let mut max_fit = population[0].fitness();
        for i in 0..population.len() {
            if population[i].fitness() > max_fit {
                max_fit = population[i].fitness();
                max_i = i;
            }
        }
        (max_i, max_fit)
    }

    pub fn start_loop(&self) -> (Rc<Chromosome>, Vec<f32>) {
        let mut best_fitnesses: Vec<f32> = Vec::new();
        let mut best: Rc<Chromosome> = self.random_chromosome();
        for _i in 1..=self.maxiter {
            let parents = self.parent_selection();
            let mut offsprings = self.recombination(&parents);
            offsprings = self.mutation(offsprings);
            self.population.replace(offsprings);
            let self_population = self.population.borrow_mut();
            let (best_index, best_fitness) = self.maximum_fitness(&self_population);
            best_fitnesses.push(best_fitness);
            if best.fitness() < self_population[best_index].fitness() {
                best = Rc::clone(&self_population[best_index]);
            }
        }
        (best, best_fitnesses)
    }
}
