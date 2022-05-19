use rand::prelude::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::fs; // 0.6.5

/// reads the QAP instance from the file
pub fn read_instance() -> (usize, Vec<i32>, Vec<i32>) {
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
    return (n, _w, _d);
}

/// generates a random solution
pub fn random_solution(size: usize) -> Vec<usize> {
    let mut vec: Vec<usize> = (0..size).collect();
    vec.shuffle(&mut thread_rng());
    return vec;
}

/// calculates the fitness of a solution
pub fn _fitness(x: &Vec<usize>, w: &Vec<i32>, d: &Vec<i32>) -> i32 {
    let mut f = 0;

    for i in 0..x.len() {
        for j in 0..x.len() {
            f += d[x[i] * x.len() + x[j]] * w[i * x.len() + j];
        }
    }
    f
}

/// modifies a solution by swapping two elements
pub fn swap_two_elements(a: &mut Vec<usize>) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let index1 = rng.gen_range(0, a.len());
    let mut index2 = rng.gen_range(0, a.len());
    while index1 == index2 {
        index2 = rng.gen_range(0, a.len());
    }
    a.swap(index1, index2);
    return a.to_vec();
}

/// computes a new solution by swapping two elements n(perturbations) times
pub fn perturbate_solution(mut solution: Vec<usize>, perturbations: u8) -> Vec<usize> {
    for _ in 0..perturbations {
        solution = swap_two_elements(&mut solution);
    }
    solution
}
