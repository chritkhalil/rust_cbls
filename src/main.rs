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

mod ts;
//use ts::solve as ts_solve;
//use ts::TabuSearch;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
fn main() {
    // let (n, _w, _d) = read_instance();

    // let mut sa = SimulatedAnnealing::new(n, _fitness, 5, 0.25, 0.85);
    // let mut ils = IteratedLocalSearch::new(n, _fitness);
    //sa_solve(&mut sa, 60, &_w, &_d);
    //solve(&mut ils, 60, &_w, &_d);
    //spawn_threads();
    //test_threads();
    let (n, _w, _d) = read_instance();
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

fn test_threads() {
    let solutions: Arc<Mutex<Vec<Vec<usize>>>> = Arc::new(Mutex::new(vec![]));

    //let (sender, receiver) = std::sync::mpsc::channel();

    let sa_thread = std::thread::spawn({
        let clone = Arc::clone(&solutions);
        let (n, _w, _d) = read_instance();

        move || {
            let mut sa = SimulatedAnnealing::new(n, _fitness, 5, 0.25, 0.85);
            sa_solve(&mut sa, 2, &_w, &_d, &clone);
            // for i in 0..100 {
            //     println!("[{:?}] Sending: {}", std::thread::current().id(), i);
            //     //sender.send(i).unwrap();
            //     //std::thread::sleep(Duration::from_secs(1));
            // }
        }
    });

    // let ts_thread = std::thread::spawn({
    //     let clone = Arc::clone(&solutions);
    //     let (n, _w, _d) = read_instance();

    //     move || {
    //         let mut sa = SimulatedAnnealing::new(n, _fitness, 5, 0.25, 0.85);
    //         sa_solve(&mut sa, 2, &_w, &_d, &clone);
    //         // for i in 0..100 {
    //         //     println!("[{:?}] Sending: {}", std::thread::current().id(), i);
    //         //     //sender.send(i).unwrap();
    //         //     //std::thread::sleep(Duration::from_secs(1));
    //         // }
    //     }
    // });

    let ils_thread = std::thread::spawn({
        let clone = Arc::clone(&solutions);
        let (n, _w, _d) = read_instance();

        move || {
            //let mut v = clone.lock().unwrap();
            let mut ils = IteratedLocalSearch::new(n, _fitness);
            //solve(&mut ils, 10, &_w, &_d, &mut v);
            solve(&mut ils, 2, &_w, &_d, &clone);
            // for i in 0..100 {
            //     println!("[{:?}] Received: {}", std::thread::current().id(), i);
            //     //sender.send(i).unwrap();
            //     //std::thread::sleep(Duration::from_secs(1));
            // }
            // for i in receiver {
            //     println!("[{:?}] Received: {}", std::thread::current().id(), i);
            // }
        }
    });

    let _ = sa_thread.join();
    let _ = ils_thread.join();
    //let _ = ts_thread.join();
    println!("Done");
    println!("{:?}", solutions);
}
