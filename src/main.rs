/*
 The code below is a test code.
 It is used to test the implementation of the SA algorithm.
 It is not used in the final implementation.
*/

mod qap;
use qap::*;
mod utils;
use utils::*;

mod sa;
use sa::solve as sa_solve;
use sa::SimulatedAnnealing;

mod ils;
use ils::*;

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // let (n, _w, _d) = read_instance();

    // let mut sa = SimulatedAnnealing::new(n, _fitness, 5, 0.25, 0.85);
    // let mut ils = IteratedLocalSearch::new(n, _fitness);
    //sa_solve(&mut sa, 60, &_w, &_d);
    //solve(&mut ils, 60, &_w, &_d);
    spawn_threads();
}

fn spawn_threads() {
    // `Vec<usize>` is wrapped inside a `Mutex` and `Arc`.
    // `Mutex` provides synchronization, `Arc` provides lifetime so each
    // thread participates in ownership over the `Mutex<Vec<usize>>`
    let solutions: Arc<Mutex<Vec<Vec<usize>>>> = Arc::new(Mutex::new(vec![]));
    let mut threads = vec![];
    for x in 0..1 {
        threads.push(thread::spawn({
            let clone = Arc::clone(&solutions);
            move || {
                let mut v = clone.lock().unwrap();
                let (n, _w, _d) = read_instance();
                let mut sa = SimulatedAnnealing::new(n, _fitness, 5, 0.25, 0.85);
                sa_solve(&mut sa, 5, &_w, &_d, &mut v);
                //v.push(vec![x, x + 1]);
                //print_type_of(&solutions);
            }
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
    println!("{:?}", solutions);
}
