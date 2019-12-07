use rand::prelude::*;
fn shifting(switch_prob: f64) -> bool {
    let mut rng = rand::thread_rng();
    let switch: f64 = rng.gen();
    let switching: bool  =  switch <= switch_prob;
    switching
}
fn main() {
    const SIMS_TO_RUN: u64 = 10000;
    //const CONCURRENT_THREADS: u64 = 6;
    //const NUMBER_OF_STUDENTS: u64 = 250;
    const SWITCH_CHANCE: f64 = 0.25;
    //let mut rng = thread_rng();
    let mut num_stay = 0;
    let mut num_shift = 0;
    for _ in 0..SIMS_TO_RUN {
        let switching: bool  =  shifting(SWITCH_CHANCE);
        if switching {
            num_shift += 1;
        } else {
            num_stay += 1;
        }
    }
    println!("Staying: {}, Switching: {}", num_stay, num_shift);
}
