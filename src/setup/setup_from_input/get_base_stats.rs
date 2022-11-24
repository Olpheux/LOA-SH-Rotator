use crate::character::base_stats_struct::BaseStats as stat_struct;
use text_io::try_read;

fn get_i64_input() -> i64 {
    match try_read!() {
        Ok(user_input) => user_input,
        Err(_) => {
            println!("Failed to parse input as a number. Maybe you included the symbols? Try again: ");
            get_i64_input()
        }
    }
}

fn get_crit() -> i64 {
    println!("Crit (stat, not percent): ");
    get_i64_input()
}

fn get_spec() -> i64 {
    println!("Specialization: ");
    get_i64_input()
}

fn get_swift() -> i64 {
    println!("Swiftness: ");
    get_i64_input()
}

fn get_attack_power() -> i64 {
    println!("Attack Power: ");
    get_i64_input()
}

pub fn get() -> stat_struct{
    stat_struct{
        crit_stat: get_crit(),
        spec_stat: get_spec(),
        swift_stat: get_swift(),
        attack_power: get_attack_power(),
        move_speed: 0.0 // This gets handled elsewhere when building from inputs.
    }
}