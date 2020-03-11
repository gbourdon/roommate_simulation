# Roommate Simulation
This program simulates the case of two students being roommates with eachother.
Configuration is handled through constants in the main function:
```
const SIMS_TO_RUN: u64 = 100_000; // How many simulations do you want to run?
    const CONCURRENT_THREADS: u64 = 10; // How many do you want to run at once?
    const NUMBER_OF_STUDENTS: u64 = 250; // How many students (that matter) are "participating"?
    const SWITCH_CHANCE: f64 = 0.25; // What is the chance that a student will change their roommate?
```
The program supports concurrency through the `CONCURRENT_THREADS` variable, which splits the simulations.