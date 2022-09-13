use rand::Rng;
use rand::prelude::SliceRandom;
use rand::thread_rng;

pub struct ThreadedQueue{
    // Implement this
    // Arc<Mutex<something>>
}

impl ThreadedQueue {
    pub fn put(

    ){}}


impl ThreadedQueue {
    pub fn get(
        
    ){}}

// we should only use one call to the random number generator
// across in whole project, here we do not, needs fixing
pub fn random(from: usize, to: usize) -> usize {
    rand::thread_rng().gen_range(from, to)
}
