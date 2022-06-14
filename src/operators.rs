mut struct Operator{
    // Implement this
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
pub fn perturbate_solution(
    mut solution: Vec<usize>,
    operator: fn(&mut Vec<usize>) -> Vec<usize>,
    perturbations: u8,
) -> Vec<usize> {
    for _ in 0..perturbations {
        solution = operator(&mut solution);
    }
    solution
}

/// modifies a solution by inserting an element i at position j
pub fn insert_element(a: &mut Vec<usize>) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let index1 = rng.gen_range(0, a.len());
    let temp_element = a[index1];
    a.remove(index1);
    let mut index2 = rng.gen_range(0, a.len());
    while index1 == index2 {
        index2 = rng.gen_range(0, a.len());
    }
    a.insert(temp_element, index2);
    return a.to_vec();
}
