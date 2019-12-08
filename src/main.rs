use rand::prelude::*;
use std::thread;

fn shifting(switch_prob: f64) -> bool {
    let mut rng = rand::thread_rng();
    let switch: f64 = rng.gen();
    let switching: bool  =  switch <= switch_prob;
    switching
}

fn matchmaker(sims: u64, threads: u64) -> (Vec<u64>, u64) {
    let mut sim_assignments: Vec<u64> = vec![];

    let sim_assignment_divisor: u64 = (sims as f64 / threads as f64).floor() as u64; // This looks less like code and more like my english homework :P
    let mut assignment = 0;
        
    for i in 0..threads {
        assignment = sim_assignment_divisor * (i+1);
        sim_assignments.push(assignment);
    }
    let remainder = sims - assignment;
    (sim_assignments, remainder)
}

fn simulation(range: std::ops::Range<u64>, students: u64, switch_prob: f64) {
    for sim_num in range {
        let mut num_stay = 0; // How many students are staying with their roommate?
        let mut num_shift = 0; // And how many are changing roomates?
        for _ in 0..students { // Finds if a student is keeping their roomate or changeing it
            let switching: bool = shifting(switch_prob);
            if switching {
                num_shift += 1;
            } else {
                num_stay += 1;
            }
        }
        println!("SIM: {}: Staying: {}, Switching: {}", sim_num, num_stay, num_shift); // And gives the results
    }
}

fn main() {
    const SIMS_TO_RUN: u64 = 10000; // How many simulations do you want to run?
    const CONCURRENT_THREADS: u64 = 6; // How many do you want to run at once?
    const NUMBER_OF_STUDENTS: u64 = 1000; // How many students (that matter) are "participating"?
    const SWITCH_CHANCE: f64 = 0.25; // What is the chance that a student will change their roommate?
    
    let (sim_assignments, remainder) = matchmaker(SIMS_TO_RUN, CONCURRENT_THREADS);
    let mut threads = vec![];
    
    for n in 0..CONCURRENT_THREADS {
        if n == 0 { // Deals with accesing (0-1). Is there a function of Vec that allows this to be removed (IE v.at((n-1),0))
            let t_sim_assignments = sim_assignments.clone();
    
            let thread = thread::Builder::new().name(n.to_string()).spawn(move || {
                simulation(0..(t_sim_assignments)[n as usize], NUMBER_OF_STUDENTS, SWITCH_CHANCE);
            }).unwrap();
            threads.push(thread);
        } else { // Deals with everything else
            let t_sim_assignments = sim_assignments.clone();
    
            let thread = thread::Builder::new().name(n.to_string()).spawn(move || {
                simulation((n-1)..(t_sim_assignments)[n as usize], NUMBER_OF_STUDENTS, SWITCH_CHANCE);
            }).unwrap();
            threads.push(thread);
        }
    }
    
    // Now deals with the remainder
    if remainder !=0 {
    
        let thread = thread::Builder::new().name("Remainder".to_string()).spawn(move || {
            simulation((SIMS_TO_RUN-remainder)..SIMS_TO_RUN, NUMBER_OF_STUDENTS, SWITCH_CHANCE);
            
        }).unwrap();
        threads.push(thread);    
    }

    // Puts a bow on it
    for thread in threads {
        thread.join().unwrap();
    }
}
