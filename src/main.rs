/*
 The code below is a test code.
 It is used to test the implementation of the SA algorithm.
 It is not used in the final implementation.
*/
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::fs; // 0.6.5
use std::time;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

// function get_neighbor(assignment)
//     return allunique(assignment) ? SwapTwoElements(assignment) : IncrementOrDecrementVar(assignment)
// end

fn main() {
    let instance_name: &str = "had12.dat"; // bks = 1652, tai12a bks = 224416
    let file: &str = "/Users/pro/development/CSP/instances/";
    let file_name = format!("{}{}", file, instance_name);

    println!("Reading file: {}", file_name);

    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    let numbers: Vec<i32> = contents // (1)
        .split(' ')
        .map(|s| s.trim()) // (2)
        .filter(|s| !s.is_empty()) // (3)
        .map(|s| s.parse().unwrap()) // (4)
        .collect(); // (5)
    let n: usize = numbers[0] as usize;
    let _w = numbers[1..n * n + 1].to_vec();
    let _d = numbers[n * n + 1..n * n * 2 + 1].to_vec();

    let mut x = vec![3, 10, 11, 2, 12, 5, 6, 7, 8, 1, 4, 9];
    x = x.into_iter().map(|x| x - 1).collect();

    let mut f = fitness(&x, &_w, &_d);
    println!("fitness: {}", f);
    println!("random solution: {:?}", random_solution(n));

    let mut best_individual = vec![];
    let mut best_fitness = f64::MAX;

    //          size::Int,
    //         fitness_function::Function,
    //         #connector::Connector,
    //         time_penalty = 50,
    //         initial_solutions_count = 10,
    //         levelLength_factor = 0.25,
    //         temp_reduction = 0.85,
    //         best_individual=Vector{Int}(),
    //         best_fitness=Inf

    let sa = SimulatedAnnealing::new(n, 50, 10, 0.25, 0.85, best_individual, best_fitness);
    start(sa, 10, &_w, &_d);
    //let test = 15.0 * f64::powi(10.0, -322);
    //println!("test: {}", approx_equal(0.000000001, 0.0, 9));
    //print_type_of(&_w);
}

fn approx_equal(a: f64, b: f64, dp: u8) -> bool {
    let p = 10f64.powi(-(dp as i32));
    (a - b).abs() < p
}

fn fitness(x: &Vec<usize>, w: &Vec<i32>, d: &Vec<i32>) -> i32 {
    let mut f = 0;

    for i in 0..x.len() {
        for j in 0..x.len() {
            f += d[x[i] * x.len() + x[j]] * w[i * x.len() + j];
        }
    }
    f
}

struct SimulatedAnnealing {
    size: usize,
    time_penalty: u16,
    initial_solutions_count: usize,
    level_length_factor: f64,
    temp_reduction: f64,
    best_individual: Vec<usize>,
    best_fitness: f64,
}

struct IteratedLocalSearch {
    size: usize,
    time_penalty: u16,
    best_individual: Vec<usize>,
    best_fitness: f64,
}

impl IteratedLocalSearch {
    fn new(size: usize, time_penalty: u16, best_individual: Vec<usize>, best_fitness: f64) -> Self {
        IteratedLocalSearch {
            size: size,
            time_penalty: time_penalty,
            best_individual: best_individual,
            best_fitness: best_fitness,
        }
    }
}

impl SimulatedAnnealing {
    fn new(
        size: usize,
        time_penalty: u16,
        initial_solutions_count: usize,
        level_length_factor: f64,
        temp_reduction: f64,
        best_individual: Vec<usize>,
        best_fitness: f64,
    ) -> Self {
        SimulatedAnnealing {
            size: size,
            time_penalty: time_penalty,
            initial_solutions_count: initial_solutions_count,
            level_length_factor: level_length_factor,
            temp_reduction: temp_reduction,
            best_individual: best_individual,
            best_fitness: best_fitness,
        }
    }
}

fn swap_two_elements(a: &mut Vec<usize>) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let index1 = rng.gen_range(0, a.len());
    let mut index2 = rng.gen_range(0, a.len());
    while index1 == index2 {
        index2 = rng.gen_range(0, a.len());
    }
    a.swap(index1, index2);
    return a.to_vec();
}

fn random_solution(size: usize) -> Vec<usize> {
    let mut vec: Vec<usize> = (0..size).collect();
    vec.shuffle(&mut thread_rng());
    return vec;
}

fn mean(data: &[f64]) -> Option<f64> {
    let sum = data.iter().sum::<f64>() as f64;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f64),
        _ => None,
    }
}

fn std_deviation(data: &[f64]) -> Option<f64> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data
                .iter()
                .map(|value| {
                    let diff = data_mean - (*value as f64);

                    diff * diff
                })
                .sum::<f64>()
                / count as f64;

            Some(variance.sqrt())
        }
        _ => None,
    }
}

fn start(sa: SimulatedAnnealing, time_limit: u64, w: &Vec<i32>, d: &Vec<i32>) {
    let level_length = (sa.level_length_factor * (sa.size * sa.size - 1) as f64).ceil();
    //print_type_of(&level_length);
    run(sa, level_length, time_limit, w, d);
}

fn run(mut sa: SimulatedAnnealing, level_length: f64, time_limit: u64, w: &Vec<i32>, d: &Vec<i32>) {
    println!("Starting SA");
    let mut solutions = Vec::with_capacity(sa.initial_solutions_count);
    let mut solutions_fitness = Vec::with_capacity(sa.initial_solutions_count);

    let iteration_start_time = 0;
    let iteration_time = 0;
    let max_iteration_time = 0;
    // # Generate initial solutions and select the one with the best fitness.
    // # This is also used to choose an initial temperature value.
    let mut current_solution = Vec::with_capacity(sa.initial_solutions_count as usize);

    let mut current_fitness = 0;
    let mut exec_time = time::SystemTime::now(); //floor(Int, time_ns()/10^9); //
                                                 // if exec_time.elapsed().unwrap().as_secs() > time_limit {
                                                 //     return;
                                                 // }
    while (exec_time.elapsed().unwrap().as_secs() < time_limit) {
        //floor(Int, time_ns()/10^9) - exec_time < time_limit) {//
        while solutions.len() < sa.initial_solutions_count {
            let mut random_solution = random_solution(sa.size);
            solutions.extend([random_solution]);
        }
        //sort!(solutions, by= genome -> sa.FitnessFunction(genome))
        solutions.sort_by(|a, b| fitness(&b, w, d).cmp(&fitness(&a, w, d)));
        //people.sort_by(|a, b| b.age.cmp(&a.age));

        if current_solution.len() == 0 {
            current_solution = solutions[0].clone();
            print_type_of(&current_solution);
            current_fitness = fitness(&current_solution, w, d);
            //current_fitness = fitness(current_solution.clone(), w , d) as f64;
        }

        for i in 0..solutions.len() {
            solutions_fitness.push(fitness(&solutions[i], w, d) as f64);
        }

        let mut temperature = std_deviation(solutions_fitness.as_slice());

        let mut best_solution = current_solution.clone();
        let mut best_fitness = current_fitness;
        // This is equivalent to temperature.unwrap() > 0_f64, but the latter does not work with precision
        while !approx_equal(temperature.unwrap(), 0_f64, u8::MAX) {
            let mut improved_fitness = best_fitness;

            for level in 0..level_length.floor() as usize {
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
                    if u <= (-fitness_diff as f64 / temperature.unwrap()).exp() {
                        if (new_fitness < best_fitness) {
                            best_solution = new_solution.to_vec();
                            best_fitness = new_fitness;
                        }
                        current_solution = new_solution.to_vec();
                        current_fitness = new_fitness;
                    }
                }
            }
            temperature = Some(temperature.unwrap() * sa.temp_reduction);

            if (best_fitness < improved_fitness) {
                println!("BestFitness(SA): {}", best_fitness);
                println!("SOL(SA): {:?}", best_solution);

                sa.best_fitness = best_fitness as f64;
                sa.best_individual = best_solution.clone();
            }
        }
    }

    //   "workbench.colorCustomizations": {
    //     "tab.activeBackground": "#282c34",
    //     "activityBar.background": "#282c34",
    //     "sideBar.background": "#282c34",
}

/*
    timeOut = time.time() + timeLimit
    Returns when a time limit is reached.
*/

// function perturbate_solution(solution::Vector{Int64}, perturbations::Int)
//     for _ in 1:perturbations
//         #solution = allunique(solution) ? SwapTwoElements(solution) : IncrementOrDecrementVar(solution)
//         #solution = IncrementOrDecrementVar(solution)
//         solution = SwapTwoElements(solution)
//     end
//     return solution
// end

fn perturbate_solution(mut solution: Vec<usize>, perturbations: i32) -> Vec<usize> {
    for _ in 0..perturbations {
        solution = swap_two_elements(&mut solution);
    }
    solution
}
