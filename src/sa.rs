use crate::qap::*;
use crate::utils::*;
use rand::Rng;
use std::sync::MutexGuard;
use std::sync::{Arc, Mutex};
use std::time;

pub struct SimulatedAnnealing {
    size: usize,
    fitness_function: fn(&Vec<usize>, &Vec<i32>, &Vec<i32>) -> i32,
    initial_solutions_count: usize,
    level_length_factor: f64,
    temp_reduction: f64,
    best_individual: Vec<usize>,
    best_fitness: f64,
}

impl SimulatedAnnealing {
    pub fn new(
        size: usize,
        fitness_function: fn(&Vec<usize>, &Vec<i32>, &Vec<i32>) -> i32,
        initial_solutions_count: usize,
        level_length_factor: f64,
        temp_reduction: f64,
    ) -> Self {
        SimulatedAnnealing {
            size: size,
            fitness_function: fitness_function,
            initial_solutions_count: initial_solutions_count,
            level_length_factor: level_length_factor,
            temp_reduction: temp_reduction,
            best_individual: vec![],
            best_fitness: f64::MAX,
        }
    }
}

pub fn solve(
    sa: &mut SimulatedAnnealing,
    time_limit: u64,
    w: &Vec<i32>,
    d: &Vec<i32>,
    sol_arc: &Arc<Mutex<Vec<Vec<usize>>>>,
) {
    let level_length = (sa.level_length_factor * (sa.size * (sa.size - 1)) as f64).ceil();
    //print_type_of(&level_length);
    start_sa(sa, level_length, time_limit, w, d, &sol_arc);
}

fn start_sa(
    sa: &mut SimulatedAnnealing,
    level_length: f64,
    time_limit: u64,
    w: &Vec<i32>,
    d: &Vec<i32>,
    sol_arc: &Arc<Mutex<Vec<Vec<usize>>>>,
) {
    let fitness = sa.fitness_function;
    println!("Starting Simulated Annealing...");
    let mut solutions = Vec::with_capacity(sa.initial_solutions_count);
    let mut solutions_fitness = Vec::with_capacity(sa.initial_solutions_count);

    // let iteration_start_time = 0;
    // let iteration_time = 0;
    // let max_iteration_time = 0;
    // # Generate initial solutions and select the one with the best fitness.
    // # This is also used to choose an initial temperature value.
    let mut current_solution = Vec::with_capacity(sa.size as usize);

    let mut current_fitness = 0;
    let exec_time = time::SystemTime::now();
    while exec_time.elapsed().unwrap().as_secs() < time_limit {
        while solutions.len() < sa.initial_solutions_count {
            let random_solution = random_solution(sa.size);
            solutions.extend([random_solution]);
        }
        solutions.sort_by(|a, b| fitness(&b, w, d).cmp(&fitness(&a, w, d)));

        if current_solution.len() == 0 {
            current_solution = solutions[0].clone();
            print_type_of(&current_solution);
            current_fitness = fitness(&current_solution, w, d);
        }

        for i in 0..solutions.len() {
            solutions_fitness.push(fitness(&solutions[i], w, d) as f64);
        }

        let mut temperature = std_deviation(solutions_fitness.as_slice());

        let mut best_solution = current_solution.clone();
        let mut best_fitness = current_fitness;
        // This is equivalent to temperature.unwrap() > 0_f64, which does not work with precision
        while !approx_equal(temperature.unwrap(), 0_f64, u8::MAX) {
            let improved_fitness = best_fitness;

            for _ in 0..level_length.floor() as usize {
                let new_solution = swap_two_elements(&mut current_solution);

                let new_fitness = fitness(&new_solution, w, d);
                let fitness_diff = new_fitness - current_fitness;

                if fitness_diff <= 0 {
                    if new_fitness < best_fitness {
                        best_solution = new_solution.to_vec();
                        best_fitness = new_fitness;
                    }
                    current_solution = new_solution.to_vec();
                    current_fitness = new_fitness;
                } else {
                    let u = rand::thread_rng().gen::<f64>();
                    if fitness_diff < 0 || u <= (-fitness_diff as f64 / temperature.unwrap()).exp()
                    {
                        if new_fitness < best_fitness {
                            best_solution = new_solution.to_vec();
                            best_fitness = new_fitness;
                        }
                        current_solution = new_solution.to_vec();
                        current_fitness = new_fitness;
                    }
                }
            }
            temperature = Some(temperature.unwrap() * sa.temp_reduction);

            if best_fitness < improved_fitness {
                println!("BestFitness(SA): {}", best_fitness);
                println!("SOL(SA): {:?}", best_solution);

                sa.best_fitness = best_fitness as f64;
                sa.best_individual = best_solution.clone();
            }
        }
    }
    let mut v = sol_arc.lock().unwrap();
    //v.push(sa.best_individual.clone());
}
