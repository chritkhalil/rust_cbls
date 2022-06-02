// use crate::qap::*;
// use rand::Rng;
// use std::sync::MutexGuard;
// use std::sync::{Arc, Mutex};
// use std::time;

// pub struct TabuSearch {
//     size: usize,
//     fitness_function: fn(&Vec<usize>, &Vec<i32>, &Vec<i32>) -> i32,
//     neighbor_checks_factor: f64,
//     tabu_list_factor: f64,
//     best_individual: Vec<usize>,
//     best_fitness: f64,
// }

// impl TabuSearch {
//     pub fn new(
//         size: usize,
//         fitness_function: fn(&Vec<usize>, &Vec<i32>, &Vec<i32>) -> i32,
//         neighbor_checks_factor: f64,
//         tabu_list_factor: f64,
//     ) -> Self {
//         TabuSearch {
//             size: size,
//             fitness_function: fitness_function,
//             neighbor_checks_factor: neighbor_checks_factor,
//             tabu_list_factor: tabu_list_factor,
//             best_individual: vec![],
//             best_fitness: f64::MAX,
//         }
//     }
// }

// pub fn solve(
//     ts: &mut TabuSearch,
//     time_limit: u64,
//     w: &Vec<i32>,
//     d: &Vec<i32>,
//     sol_arc: &Arc<Mutex<Vec<Vec<usize>>>>,
// ) {
//     let neighbor_checks = (ts.neighbor_checks_factor * (ts.size * (ts.size - 1)) as f64).ceil();
//     let tabu_list_length = (tabu_list_factor * ts.size as f64).ceil();

//     start_ts(
//         ts,
//         time_limit,
//         neighbor_checks,
//         tabu_list_length,
//         w,
//         d,
//         sol_arc,
//     );
// }

// fn start_ts(
//     ts: &mut TabuSearch,
//     time_limit: u64,
//     neighbor_checks: f64,
//     tabu_list_length: usize,
//     w: &Vec<i32>,
//     d: &Vec<i32>,
//     sol_arc: &Arc<Mutex<Vec<Vec<usize>>>>,
// ) {
//     let MAX_TS_ITER_SIZE = 500;

//     let fitness = ts.fitness_function;
//     println!("Starting Tabu Search...");

//     let restart_iterations = 50;
//     let mut original_solution = Vec::with_capacity(ts.size as usize);
//     let mut current_fitness = 0;

//     let mut last_improvement = 0;

//     let mut perturbations = rand::thread_rng().gen::<u8>() % 5;

//     let mut prev_best_fitness = i32::MAX;

//     let next_tabu = [-1, 1];

//     let exec_time = time::SystemTime::now();
//     while exec_time.elapsed().unwrap().as_secs() < time_limit {
//         if original_solution.is_empty() {
//             original_solution = random_solution(ts.size);
//             //print_type_of(&original_solution);
//         }
//         let mut current_solution = original_solution.clone();
//         current_fitness = fitness(&original_solution, w, d);
//         let mut best_solution = current_solution.clone();
//         let mut best_fitness = current_fitness.clone();

//         for i in 0..MAX_TS_ITER_SIZE {
//             println!("Iteration: {}", i);

//             let mut counter = 0;
//             let next_solution = None;
//             let next_fitness = i32::MAX;
//             let next_tabu = [-1, -1];
//             let success = false;
//         }
//         current_solution = perturbate_solution(current_solution, perturbations);

//         current_fitness = fitness(&current_solution, w, d);

//         // if current_fitness <= best_fitness {
//         //     best_fitness = current_fitness;
//         //     best_solution = current_solution.clone();
//         //     last_improvement = 0;
//         //     perturbations = rand::thread_rng().gen::<u8>() % 5;
//         // } else if (last_improvement + 1 == restart_iterations) {
//         //     //newSolution = paramSolution;
//         //     current_solution = original_solution.clone();
//         //     last_improvement = 0;
//         //     perturbations = rand::thread_rng().gen::<u8>() % 5;
//         // } else {
//         //     perturbations += 1;
//         //     last_improvement += 1;
//         // }

//         // if (best_fitness < prev_best_fitness) {
//         //     // # if (rand() < ts.emigration_freq)
//         //     // #     elite_emigration!(ts, BestSolution)
//         //     // # end
//         //     // #add_configuration(sa.connector, tuple(BestSolution...))
//         //     // put!(jobs, (BestSolution, BestFitness))
//         //     // worker_host = gethostname()

//         //     println!("BestFitness(ts): {}", best_fitness);
//         //     println!("SOL(ts): {:?}", best_solution);

//         //     //sa.best_fitness = best_fitness as f64;
//         //     //sa.best_individual = best_solution.clone();

//         //     ts.best_fitness = best_fitness as f64;
//         //     ts.best_individual = best_solution.clone();
//         //     prev_best_fitness = best_fitness;
//         // }
//     }
//     let mut v = sol_arc.lock().unwrap();
//     v.push(ts.best_individual.clone());
// }
