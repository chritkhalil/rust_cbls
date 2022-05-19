use crate::qap::*;
use crate::utils::*;
use rand::Rng;
use std::time;

pub struct IteratedLocalSearch {
    size: usize,
    fitness_function: fn(&Vec<usize>, &Vec<i32>, &Vec<i32>) -> i32,
    best_individual: Vec<usize>,
    best_fitness: f64,
}

impl IteratedLocalSearch {
    pub fn new(
        size: usize,
        fitness_function: fn(&Vec<usize>, &Vec<i32>, &Vec<i32>) -> i32,
    ) -> Self {
        IteratedLocalSearch {
            size: size,
            fitness_function: fitness_function,
            best_individual: vec![],
            best_fitness: f64::MAX,
        }
    }
}

pub fn solve(ils: &mut IteratedLocalSearch, time_limit: u64, w: &Vec<i32>, d: &Vec<i32>) {
    //let level_length = (ils.level_length_factor * (ils.size * ils.size - 1) as f64).ceil();
    //print_type_of(&level_length);
    start_ils(ils, time_limit, w, d);
}

fn start_ils(ils: &mut IteratedLocalSearch, time_limit: u64, w: &Vec<i32>, d: &Vec<i32>) {
    let fitness = ils.fitness_function;
    println!("Starting ILS");

    let restart_iterations = 50;
    let mut original_solution = Vec::with_capacity(ils.size as usize);
    let mut current_fitness = 0;

    let mut last_improvement = 0;

    let mut perturbations = rand::thread_rng().gen::<u8>() % 5;

    if original_solution.is_empty() {
        original_solution = random_solution(ils.size);
        print_type_of(&original_solution);
    }

    let mut current_solution = original_solution.clone();
    current_fitness = fitness(&original_solution, w, d);

    let mut best_solution = current_solution.clone();
    let mut best_fitness = current_fitness.clone();

    let mut prev_best_fitness = i32::MAX;

    let exec_time = time::SystemTime::now();
    while exec_time.elapsed().unwrap().as_secs() < time_limit {
        current_solution = perturbate_solution(current_solution, perturbations);

        current_fitness = fitness(&current_solution, w, d);

        if current_fitness <= best_fitness {
            best_fitness = current_fitness;
            best_solution = current_solution.clone();
            last_improvement = 0;
            perturbations = rand::thread_rng().gen::<u8>() % 5;
        } else if (last_improvement + 1 == restart_iterations) {
            //newSolution = paramSolution;
            current_solution = original_solution.clone();
            last_improvement = 0;
            perturbations = rand::thread_rng().gen::<u8>() % 5;
        } else {
            perturbations += 1;
            last_improvement += 1;
        }

        if (best_fitness < prev_best_fitness) {
            // # if (rand() < ils.emigration_freq)
            // #     elite_emigration!(ils, BestSolution)
            // # end
            // #add_configuration(sa.connector, tuple(BestSolution...))
            // put!(jobs, (BestSolution, BestFitness))
            // worker_host = gethostname()

            println!("BestFitness(ILS): {}", best_fitness);
            println!("SOL(ILS): {:?}", best_solution);

            //sa.best_fitness = best_fitness as f64;
            //sa.best_individual = best_solution.clone();

            ils.best_fitness = best_fitness as f64;
            ils.best_individual = best_solution.clone();
            prev_best_fitness = best_fitness;
        }
    }
}
