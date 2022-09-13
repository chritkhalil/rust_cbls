/*
 The code below is a test code.
 It is used to test the implementation of the SA algorithm.
 It is not used in the final implementation.
*/

mod qap;
use qap::*;
mod utils;
use utils::*;

mod logger;
use logger::*;

mod sa;
use sa::solve as sa_solve;
use sa::SimulatedAnnealing;

mod ils;
use ils::*;

mod operators;

mod ts;
//use ts::solve as ts_solve;
//use ts::TabuSearch;

use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{io, thread};

fn main() -> io::Result<()> {
    logger::init();
    let n_available_threads = thread::available_parallelism()?.get();
    assert!(n_available_threads >= 1_usize);
    println!("Available threads: {}", n_available_threads);
    let n_solvers = 2;
    let threads_count = std::cmp::min(n_available_threads - 1, n_solvers);

    let (n, _w, _d) = read_instance();

    let use_threading = true;

    if !use_threading {
        //let mut sol = vec![0; n];
    } else {
        test_threads(threads_count);
    }

    Ok(())
}

fn spawn_threads() {
    // `Vec<usize>` is wrapped inside a `Mutex` and `Arc`.
    // `Mutex` provides synchronization, `Arc` provides lifetime so each
    // thread participates in ownership over the `Mutex<Vec<usize>>`
    // let (n, _w, _d) = read_instance();
    let solutions: Arc<Mutex<Vec<Vec<usize>>>> = Arc::new(Mutex::new(vec![]));
    let mut threads = vec![];
    // let mut sa = SimulatedAnnealing::new(n, _fitness, 5, 0.25, 0.85);
    // let mut ils = IteratedLocalSearch::new(n, _fitness);
    threads.push(thread::spawn({
        let clone = Arc::clone(&solutions);
        move || {
            let (n, _w, _d) = read_instance();
            let mut v = clone.lock().unwrap();
            let mut sa = SimulatedAnnealing::new(n, _fitness, 5, 0.25, 0.85);
            let mut ils = IteratedLocalSearch::new(n, _fitness);
            for i in 0..2 {
                if i == 1 {
                    //sa_solve(&mut sa, 2, &_w, &_d, &mut v);
                } else {
                    // solve(&mut ils, 2, &_w, &_d, &mut v);
                }
            }
            //sa_solve(&mut sa, 5, &_w, &_d, &mut v);
            //v.push(vec![x, x + 1]);
            //print_type_of(&solutions);
        }
    }));
    for t in threads {
        let _ = t.join();
    }
    println!("{:?}", solutions);
}

fn test_threads(threads_count: usize) {

    let solutions: Arc<Mutex<Vec<Vec<usize>>>> = Arc::new(Mutex::new(vec![]));

    //let (sender, receiver) = std::sync::mpsc::channel();

    let sa_thread = std::thread::spawn({
        let clone = Arc::clone(&solutions);
        let (n, _w, _d) = read_instance();

        move || {
            let mut sa = SimulatedAnnealing::new(n, _fitness, 5, 0.25, 0.85);
            sa_solve(&mut sa, 2, &_w, &_d, &clone);
        }
    });

    let ils_thread = std::thread::spawn({
        let clone = Arc::clone(&solutions);
        let (n, _w, _d) = read_instance();

        move || {
            let mut ils = IteratedLocalSearch::new(n, _fitness);
            solve(&mut ils, 2, &_w, &_d, &clone);
        }
    });

    let _ = sa_thread.join();

    let _ = ils_thread.join();
    //let _ = ts_thread.join();
    println!("Done");
    println!("{:?}", solutions);
}
