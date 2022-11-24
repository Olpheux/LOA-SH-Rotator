use crate::simulation::run_simulation;
use crate::character::other_structs::Skill as Skill;
use crate::character::meta_structs::Character as Character;

pub fn start_simulations(iterations: i64, character: &Character) -> (f64, Vec<Skill>) {
    // Premature assigns/underscore in name avoids errors if the user requests 0 iterations for some reason
    let mut best_found_damage: f64 = 0.0;
    let mut best_found_rotation: Vec<Skill> = [].to_vec();
    let mut _current_damage: f64 = 0.0;
    let mut _current_rotation: Vec<Skill> = [].to_vec();

    println!("=========");

    let updates_needed: i64 = std::cmp::max(iterations / 10000, 10);
    let percent_per_update: f64 = 100.0 / updates_needed as f64;
    
    for x in 0..iterations {
        let (current_damage, current_rotation) = run_simulation::run(character);
        
        if current_damage > best_found_damage {
            best_found_damage = current_damage;
            best_found_rotation = current_rotation;
        }

        if x % (iterations / updates_needed) == 0 && x != 0 {
            println!("{:?}% done...", (x / (iterations / updates_needed)) as f64 * percent_per_update);
        }
    }

    (best_found_damage, best_found_rotation)
}