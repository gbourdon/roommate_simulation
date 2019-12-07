use rand::prelude::*;

fn shifting(switch_prob: f64) -> bool {
    let mut rng = rand::thread_rng();
    let switch: f64 = rng.gen();
    let switching: bool  =  switch <= switch_prob;
    switching
}

fn main() {
    const SIMS_TO_RUN: u64 = 100; // How many simulations do you want to run?
    //const CONCURRENT_THREADS: u64 = 6; // How many do you want to run at once?
    const NUMBER_OF_STUDENTS: u64 = 1000; // How many students (that matter) are "participating"?
    const SWITCH_CHANCE: f64 = 0.25; // What is the chance that a student will change their roommate?

    for sim_num in 0..SIMS_TO_RUN { // For each simulation
        let mut num_stay = 0; // How many students are staying with their roommate?
        let mut num_shift = 0; // And how many are changing roomates?
        for _ in 0..NUMBER_OF_STUDENTS { // Finds if a student is keeping their roomate or changeing it
            let switching: bool  =  shifting(SWITCH_CHANCE);
            if switching {
                num_shift += 1;
            } else {
                num_stay += 1;
            }
        }
        println!("SIM: {}: Staying: {}, Switching: {}", sim_num, num_stay, num_shift); // And gives the results
    }
}
