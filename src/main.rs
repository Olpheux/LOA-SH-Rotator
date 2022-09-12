#![allow(non_snake_case)]
use std::io;
include!("SkillSetup.rs");
include!("GetStats.rs");
include!("Simulation.rs");
include!("Setup.rs");

fn main(){
    let (skills, demon_duration) = setup();
    println!("Setup complete!");
    start_simulation(skills, demon_duration);
}
