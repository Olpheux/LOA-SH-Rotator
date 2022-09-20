#![allow(non_snake_case)]
use std::io;
// This should probably be improved at some point
// include!() isn't really the right way to handle this, it's just easy. Do this correctly at some point.
include!("SkillSetup.rs");
include!("GetStats.rs");
include!("Simulation.rs");
include!("Setup.rs");

fn main(){
    let (skills, demon_duration, crit_chance, hallucination_set) = setup();
    println!("Setup complete!");
    start_simulation(skills, demon_duration, crit_chance, hallucination_set);
}
