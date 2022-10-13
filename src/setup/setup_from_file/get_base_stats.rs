use crate::character::char_structs::base_stats_struct::BaseStats as stat_struct;
use serde_json::{Value};

fn get_move_speed(file: &Value)-> f64 {
    let move_speed: f64 = match file["Other"][0]["MoveSpeedBonus"].as_f64() {
        None => {
            println!("FAILED: Cannot read move speed from file! Value must be a decimal number.");
            panic!()
        }
        Some(move_speed) => {
            if move_speed.is_sign_negative(){
                println!("WARNING: Move speed was negative. Setting to 0.");
                0.0
            } else {
                move_speed
            }
        }
    };

    move_speed
}

fn get_attack_power(file: &Value)-> i64 {
    let attack_power: i64 = match file["Stats"][0]["AttackPower"].as_i64() {
        None => {
            println!("FAILED: Cannot read attack power from file! Value must be an integer.");
            panic!()
        }
        Some(attack_power) => {
            if attack_power.is_negative() {
                println!("WARNING: Attack power was negative. Setting to 0.");
                0
            } else {
                attack_power
            }
        }
    }; 

    attack_power
}

fn get_crit(file: &Value)-> i64 {
    let crit: i64 = match file["Stats"][0]["Crit"].as_i64() {
        None => {
            println!("FAILED: Cannot read crit stat from file! Value must be an integer.");
            panic!()
        }
        Some(crit) => {
            if crit.is_negative() {
                println!("WARNING: Crit stat was negative. Setting to 0.");
                0
            } else {
                crit
            }
        }
    };

    crit
}

fn get_spec(file: &Value)->i64{
    let spec: i64 = match file["Stats"][0]["Spec"].as_i64() {
        None => {
            println!("FAILED: Cannot read spec stat from file! Value must be an integer.");
            panic!()
        }
        Some(spec) => {
            if spec.is_negative() {
                println!("WARNING: Specialization was negative. Setting to 0.");
                0
            } else {
                spec
            }
        }
    };
    
    spec
}

fn get_swift(file: &Value)->i64{
    let swift: i64 = match file["Stats"][0]["Swift"].as_i64() {
        None => {
            println!("FAILED: Cannot read swiftness stat from file! Value must be an integer.");
            panic!()
        }
        Some(swift) => {
            if swift.is_negative() {
                println!("WARNING: Swiftness was negative! Setting to 0.");
                0
            } else {
                swift
            }
        }
    };
    
    swift
}

//==========

pub fn get(raw_file: &str) -> stat_struct {
    let file = match serde_json::from_str::<Value>(raw_file) {
        Ok(file) => file,
        Err(_) => {
            println!("Unable to open file! Reverting to input-based setup.");
            panic!()
        }
    };

    stat_struct {
        attack_power: get_attack_power(&file),
        crit_stat: get_crit(&file),
        swift_stat: get_swift(&file),
        spec_stat: get_spec(&file),
        move_speed: get_move_speed(&file)
    }
}