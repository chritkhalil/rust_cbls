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
    pub fn new(size: usize, time_penalty: u16, best_individual: Vec<usize>, best_fitness: f64) -> Self {
        IteratedLocalSearch {
            size: size,
            fitness_function: fitness_function,
            best_individual: vec![],
            best_fitness: f64::MAX,
        }
    }
}

pub fn solve(ils: &mut IteratedLocalSearch, time_limit: u64, w: &Vec<i32>, d: &Vec<i32>) {
    let level_length = (ils.level_length_factor * (ils.size * ils.size - 1) as f64).ceil();
    //print_type_of(&level_length);
    start_ils(ils, level_length, time_limit, w, d);
}

fn start_ils(ils: &mut IteratedLocalSearch, level_length: f64, time_limit: u64, w: &Vec<i32>, d: &Vec<i32>) {
    let fitness = ils.fitness_function;
    println!("Starting ILS");

    let mut current_solution = Vec::with_capacity(ils.size as usize);
    let mut current_fitness = 0;

    let mut last_improvement = 0;

    let perturbations = rand::thread_rng().gen::<u8>() % 5;

    let exec_time = time::SystemTime::now();
    while exec_time.elapsed().unwrap().as_secs() < time_limit {
        if current_solution.len() == 0 {
            current_solution = random_solution(ils.size);
            print_type_of(&current_solution);
        }

        current_solution = perturbate_solution(&current_solution, perturbations);

        current_fitness = fitness(&current_solution, w, d);

        let mut best_solution = current_solution.clone();
        let mut best_fitness = current_fitness;

        if current_fitness <= best_fitness {
            best_fitness = current_fitness;
            best_solution = current_solution.clone()
            last_improvement = 0;
            perturbations = rand::thread_rng().gen::<u8>() % 5;
        } elseif last_improvement + 1 == restart_iterations) {
            newSolution = paramSolution;
            solution = copy(newSolution)
            last_improvement = 0;
            perturbations = rand::thread_rng().gen::<u8>() % 5;
        }
        else {
            perturbation+=1;
            last_improvement+=1;
        }


    }
}

