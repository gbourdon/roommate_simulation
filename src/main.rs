use rand::prelude::*;
use rand::seq::SliceRandom;

use std::collections::HashMap;

use std::time::SystemTime;
use std::thread;
use std::sync::mpsc;


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

fn match_students (students: Vec<u64>) -> HashMap<usize,u64> {
    let mut pairs = shuffle(students);
    
    let mut result= HashMap::new();
    
    let half_two = pairs.split_off(pairs.len()/2);

    for p in 0..pairs.len() {
        result.insert(pairs[p] as usize, half_two[p]);
        result.insert(half_two[p] as usize, pairs[p]);
    }

    result
}

fn shuffle(list: Vec<u64>) -> Vec<u64> {
    let mut pairs = list;
    let mut rng = rand::thread_rng();

    pairs.shuffle(&mut rng);
    for n in 0..pairs.len() {
        if pairs[n] == n as u64 {
            pairs = shuffle(pairs);
        }
    }
    pairs
}

fn to_array(hash: HashMap<usize, u64>) -> Vec<u64> {
    let mut out: Vec<u64> = Vec::new();
    let mut vec: Vec<usize> = Vec::new();
    for key in hash.keys() {
        vec.push(*key);
    }
    vec.sort();
    for key in vec {
        out.push(hash[&key]);
    }
    out
}

fn simulation(range: std::ops::Range<u64>, students: u64, switch_prob: f64, tx: std::sync::mpsc::Sender<u32>) {
    for sim_num in range {
        let mut students_init: Vec<u64> = Vec::new();
        for s in 0..students { // Bodge, but a nessesary one, but feel free to correct this.
            students_init.push(s);
        }

        let roommates: HashMap<usize, u64> = match_students(students_init);
        let mut switching: HashMap<usize, bool> = HashMap::new();
        for (roommate, _) in roommates.clone() {
            if shifting(switch_prob) {
                switching.insert(roommate, true);
            } else {
                switching.insert(roommate, false);
            }

        }

        // Adds one of the sim's conditions
        switching.insert(0,true);
        switching.insert(1,true);
        
        let mut new_roommates = HashMap::new();
        for (roommate, _) in roommates.clone() {
            if switching[&roommate] {
                new_roommates.insert(roommate, roommates[&roommate]);
                new_roommates.insert(roommates[&roommate] as usize, roommate as u64);
            }
        }

        let new_roommate = match_students(to_array(new_roommates));
        let mut sucessful = false;

        if new_roommate[&0] == 1 {
            sucessful = true;
        }
        if sucessful {
            tx.send(1).unwrap();
        } else {
            tx.send(0).unwrap();
        }
        println!("SIM: {}: Roomates matching: {}", sim_num, sucessful);
    }
}

fn main() {
    let now = SystemTime::now();
    const SIMS_TO_RUN: u64 = 100_000; // How many simulations do you want to run?
    const CONCURRENT_THREADS: u64 = 10; // How many do you want to run at once?
    const NUMBER_OF_STUDENTS: u64 = 250; // How many students (that matter) are "participating"?
    const SWITCH_CHANCE: f64 = 0.25; // What is the chance that a student will change their roommate?
    
    let (sim_assignments, remainder) = matchmaker(SIMS_TO_RUN, CONCURRENT_THREADS);
    let mut threads = vec![];
    let (tx, rx) = mpsc::channel();
    
    for n in 0..CONCURRENT_THREADS {
        let tx = tx.clone();
        if n == 0 { // Deals with accesing (0-1). Is there a function of Vec that allows this to be removed (IE v.at((n-1),0))
            let t_sim_assignments = sim_assignments.clone();
    
            let thread = thread::Builder::new().name(n.to_string()).spawn(move || {
                simulation(0..(t_sim_assignments)[n as usize], NUMBER_OF_STUDENTS, SWITCH_CHANCE, tx);
            }).unwrap();
            threads.push(thread);
        } else { // Deals with everything else
            let t_sim_assignments = sim_assignments.clone();
            let thread = thread::Builder::new().name(n.to_string()).spawn(move || {
                simulation(t_sim_assignments[(n-1) as usize]..(t_sim_assignments)[n as usize], NUMBER_OF_STUDENTS, SWITCH_CHANCE, tx);
            }).unwrap();
            threads.push(thread);
        }
    }
    
    // Now deals with the remainder
    if remainder !=0 {
    
        let thread = thread::Builder::new().name("Remainder".to_string()).spawn(move || {
            simulation((SIMS_TO_RUN-remainder)..SIMS_TO_RUN, NUMBER_OF_STUDENTS, SWITCH_CHANCE, tx);
            
        }).unwrap();
        threads.push(thread);    
    }

    // Puts a bow on it
    /*for thread in threads {
        thread.join().unwrap();
    }*/
    
    let mut sims_finished = 0;
    let mut perfect = 0;
    for recieved in rx {
        if recieved == 1 {
            perfect += 1;
        }
        sims_finished += 1;
        if sims_finished == (SIMS_TO_RUN) {
            break;
        }
    }

    println!("{} simulations completed", sims_finished);
    println!("{} combinations with Person 1 having Person 2 as a roommate.", perfect);

    println!("Time taken: {} ms", now.elapsed().unwrap().as_millis());
}
