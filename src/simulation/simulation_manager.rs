use crate::simulation::run_simulation;
use crate::character::char_structs::other_structs::Skill as Skill;
use crate::character::char_structs::meta_structs::Character as Character;

pub fn start_simulations(iterations: i64, character: &Character) -> (f64, Vec<Skill>) {
    // Premature assigns/underscore avoid errors if the user requests 0 iterations for some reason
    let mut best_found_damage: f64 = 0.0;
    let mut best_found_rotation: Vec<Skill> = [].to_vec();
    let mut _current_damage: f64 = 0.0;
    let mut _current_rotation: Vec<Skill> = [].to_vec();

    for _x in 0..iterations {
        let (current_damage, current_rotation) = run_simulation::run(character);
        
        if current_damage > best_found_damage {
            best_found_damage = current_damage;
            best_found_rotation = current_rotation;
        }

    }

    (best_found_damage, best_found_rotation)
}