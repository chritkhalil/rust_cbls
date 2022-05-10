/*
 The code below is a test code.
 It is used to test the implementation of the SA algorithm.
 It is not used in the final implementation.
*/

mod qap;
use qap::*;
mod utils;

mod sa;
use sa::*;


fn main() {
    let (n, _w, _d) = read_instance();

    let mut sa = SimulatedAnnealing::new(n, _fitness, 5, 0.25, 0.85);
    solve(&mut sa, 60, &_w, &_d);
    
}

