// //mut struct Operator{
//     // Implement this
// //}

// //pub fn()

// /// modifies a solution by swapping two elements
// /// 
// #[path = "./helper.rs"]
// mod helper;
// // use helper::random;

// pub fn swap_two_elements(a: &mut Vec<usize>) -> Vec<usize> {
//     let index1 = random(0, a.len());
//     let mut index2 = random(0, a.len());
//     while index1 == index2 {
//         index2 = random(0, a.len());
//     }
//     a.swap(index1, index2);
//     return a.to_vec();
// }

// /// computes a new solution by swapping two elements n times
// pub fn perturbate_solution(
//     mut solution: Vec<usize>,
//     operator: fn(&mut Vec<usize>) -> Vec<usize>,
//     n_perturbations: u8,
// ) -> Vec<usize> {
//     for _ in 0..n_perturbations {
//         solution = operator(&mut solution);
//     }
//     solution
// }

// /// modifies a solution by inserting an element at a random position 
// pub fn insert_element(a: &mut Vec<usize>) -> Vec<usize> {
//     let index1 = random(0, a.len());
//     let temp_element = a[index1];
//     a.remove(index1);
//     let mut index2 = random(0, a.len());
//     while index1 == index2 {
//         index2 = random(0, a.len());
//     }
//     a.insert(temp_element, index2);
//     return a.to_vec();
// }

// /// modifies a solution by removing an element at a random position 
// pub fn remove_element(a: &mut Vec<usize>) -> Vec<usize> {
//     a.remove(random(0, a.len()));
//     return a.to_vec();
// }

// /// modifies a solution by reversing a subsequence of elements
// pub fn reverse_subsequence(a: &mut Vec<usize>) -> Vec<usize> {
//     let index1 = random(0, a.len());
//     let mut index2 = random(0, a.len());
//     while index1 == index2 {
//         index2 = random(0, a.len());
//     }
//     a[index1..index2].reverse();
//     return a.to_vec();
// }


// //// ========= Connectors =========
// /// should be moved to connector(s).rs
// /// 

// pub fn fork(sol: Vec<usize>, n:usize) -> Vec<Vec<usize>> {
//     let mut solutions: Vec<Vec<usize>> = Vec::new();
//     for i in 0..n {
//         solutions.push(sol.clone());
//     }
//     solutions
// }

// // method: {select random, select best, select combination (ex. crossover)}
// pub fn join(solutions: Vec<Vec<usize>>, method: Fn) -> Vec<usize> {
//     method(solutions);
// }

// // cliff
// // jump

// pub fn genetic_crossover(population: Vec<Vec<usize>>, pairing: &str, crossover: &str, mating_pool_factor: f64) -> Vec<Vec<usize>> {
//     let mut mating_pool: Vec<Vec<usize>> = Vec::new();
//     let mating_pool_size = (population.len() as f64 * mating_pool_factor) as usize;
//     let mut mating_pool_index: Vec<usize> = Vec::new();
//     for _ in 0..mating_pool_size {
//         let index = random(0, population.len());
//         mating_pool_index.push(index);
//         mating_pool.push(population[index].clone());
//     }
//     let mut new_population: Vec<Vec<usize>> = Vec::new();
//     for i in 0..population.len() {
//         if mating_pool_index.contains(&i) {
//             new_population.push(population[i].clone());
//         } else {
//             let parent1 = mating_pool[random(0, mating_pool.len())].clone();
//             let parent2 = mating_pool[random(0, mating_pool.len())].clone();
//             let child = crossover(parent1, parent2);
//             new_population.push(child);
//         }
//     }
//     new_population
// }


// pub fn genetic_mutation(population: Vec<Vec<usize>>, scale: f64, distribution: &str, beta: f64){
//     let acceleration = np.zeros((pop.num_agents, pop.num_dimensions));

//    let gravitation = gravity * (- alpha * pop.iteration).exp();

//     // Determine mass for each agent
//     let raw_masses = pop.fitness - pop.current_worst_fitness;
//     let masses = raw_masses / (raw_masses.sum() + 1e-23).reshape(pop.num_agents);
//     for agent in range(pop.num_agents){
//         let agent_mass = masses[agent];
//         let agent_position = pop.positions[agent];
//         let agent_velocity = pop.velocities[agent];
//         let agent_acceleration = np.zeros(pop.num_dimensions);
//         for other_agent in range(pop.num_agents){
//             if other_agent != agent:
//                 let other_agent_mass = masses[other_agent];
//                 let other_agent_position = pop.positions[other_agent];
//                 let other_agent_velocity = pop.velocities[other_agent];
//                 let distance = np.linalg.norm(agent_position - other_agent_position);
//                 let direction = (agent_position - other_agent_position) / (distance + 1e-23);
//                 let force = gravitation * agent_mass * other_agent_mass / (distance ** 2);
//                 let acceleration = force * direction;
//                 agent_acceleration += acceleration;
//         }
//         acceleration[agent] = agent_acceleration;
//     }
// }


// fn local_random_walk(pop: Vec<Vec<usize>>, probability: f64, scale: f64, distribution: String) -> () {
//     // Determine random numbers
//     let mut r_1: Vec<Vec<f64>> = Vec::new();
//     if distribution == "uniform" {
//         r_1 = rand::random::<Vec<Vec<f64>>>();
//     } else if distribution == "gaussian" {
//         r_1 = rand::random::<Vec<Vec<f64>>>();
//     } else if distribution == "levy" {
//         r_1 = random_levy(size=(pop.num_agents, pop.num_dimensions));
//     } else {
//         panic!("Invalid distribution!");
//     }
//     let mut r_2: Vec<Vec<f64>> = rand::random::<Vec<Vec<f64>>>();

//     // Move positions with a displacement due permutations and probabilities
//     pop.positions += scale * r_1 * (pop.positions[
//                                     np.random.permutation(pop.num_agents), :] - pop.positions[
//                                                                                 np.random.permutation(pop.num_agents),
//                                                                                 :]) * np.heaviside(r_2 - probability,
//                                                                                                    0.0);
// }