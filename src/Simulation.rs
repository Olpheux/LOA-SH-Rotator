mod simulation_manager;
mod run_simulation;
use crate::character::char_structs::meta_structs::Character as Character;
use crate::character::char_structs::other_structs::Skill as Skill;
use text_io::try_read;
use thousands::Separable;

fn get_i64_input() -> i64 {
    match try_read!(){
        Ok(user_input) => user_input,
        Err(_) => {
            println!("Failed to parse input as an integer. Try again: ");
            get_i64_input()
        }
    }
}

//=========

fn print_output(damage: f64, rotation: Vec<Skill>){
    println!("\n");
    println!("Simulations complete. Best result found:\n");
    
    for item in &rotation {
        print!("{} -> ", item.name);
    }
    println!("Exit demon form.");

    for item in &rotation {
        print!("{} -> ", item.keybind);
    }
    println!("Exit demon form.");

    println!("Total damage dealt: {}", (damage as i64).separate_with_commas());
}

//=========

pub fn start_simulation(character: &Character) {
    println!("How many rotations do we generate?");
    println!("A larger number will likely give a better result, but will take longer.");
    
    let iterations = get_i64_input();

    let (best_found_damage, best_found_rotation) = simulation_manager::start_simulations(iterations, character);

    print_output(best_found_damage, best_found_rotation);
}