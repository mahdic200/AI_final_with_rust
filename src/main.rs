// pub mod plot;
pub mod genetic;
use genetic::Genetic;
use crate::genetic::enums::CrossoverType;

fn main() {

    let chr_len = 10;

    let pop_size = 10;

    let mut per_mut: String = String::new();
    println!("Enter permutation (between 0 and 1): ");
    std::io::stdin().read_line(&mut per_mut).unwrap();
    let per_mut = per_mut.trim().parse::<f32>().expect("invalid queens number");
    match &per_mut {
        x if *x > 0.0 && *x < 1.1 => (),
        _ => panic!("you cannot assign invalid value to permutation"),
    }

    let mut maxiter: String = String::new();
    println!("Enter maxiter : ");
    std::io::stdin().read_line(&mut maxiter).unwrap();
    let maxiter = maxiter.trim().parse::<u32>().expect("invalid queens number");

    let mut crossover_type: String = String::new();
    println!("Enter crossover type : ");
    println!("\nnotice : if you choose an invalid number two point crossover will be selected by default !\n");
    println!("(1) => one point ");
    println!("(2) => two point ");
    println!("(3) => uniform ");
    std::io::stdin().read_line(&mut crossover_type).unwrap();
    let crossover_type = crossover_type.trim().parse::<u32>().expect("invalid queens number");
    let crossover_type: CrossoverType = match crossover_type {
        x if x == 1 => CrossoverType::OnePoint,
        x if x == 2 => CrossoverType::TwoPoint,
        x if x == 3 => CrossoverType::Uniform,
        _ => CrossoverType::TwoPoint,
    };

    let genetic = Genetic::new(chr_len, pop_size, per_mut, maxiter, crossover_type);
    genetic.start_loop();
}
