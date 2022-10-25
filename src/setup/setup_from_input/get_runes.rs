use crate::character::char_structs::other_structs::RuneLevels;
use text_io::try_read;

fn get_i64_input() -> i64 {
    match try_read!() {
        Ok(user_input) => user_input,
        Err(_) => {
            println!("Failed to parse input as a number. Try again.");
            get_i64_input()
        }
    }
}

fn get_string_input() -> String {
    match try_read!() {
        Ok(user_input) => user_input,
        Err(_) => {
            println!("Failed to parse input. Try again.");
            get_string_input()
        }
    }
}

fn validate_rune_name(name: String) -> String {
    if name != "Wealth" &&
    name != "Rage" &&
    name != "Galewind" &&
    name != "Quick Recharge" &&
    name != "Bleed" &&
    name != "Overwhelm" &&
    name != "Protection" &&
    name != "Purify" &&
    name != "Conviction" &&
    name != "Judgement" &&
    name != "Focus" &&
    name != "Iron Wall" &&
    !name.is_empty() {
        println!("Rune {:?} not recognized. Ensure exact spelling and capitalization are correct. Try entering it again.", name);
        get_rune_name()
    } else {
        name
    }
}

//==========

fn get_rune_name() -> String {
    println!("Enter rune name: ");
    let mut input = get_string_input();

    input = validate_rune_name(input);

    input
}

fn get_rune_level() -> i64 {
    println!("Enter rune level: ");
    let input = get_i64_input();

    match input {
        0..=4 => input,
        5..=core::i64::MAX => {
            println!("WARNING: Rune levels cannot be above 4. Defauling to 4 (legendary/gold).");
            4
        }
        core::i64::MIN..=-1 => {
            println!("WARNING: Rune levels cannot be below 0. Defauling to 0 (none equipped).");
            0
        }
    }
}

//==========

pub fn get() -> Vec<RuneLevels> {
    let skill_names: Vec<String> = vec!["Ruining Rush".to_string(), "Death Claw".to_string(), "Destruction".to_string(), "Gore Bleeding".to_string(), "Leaping Blow".to_string(), "Blood Massacre".to_string()];
    let mut runes: Vec<RuneLevels> = vec![RuneLevels { rune: "".to_string(), rune_level: 0 }; 6];

    println!("Setting up skill runes.\nEnter rune names with exact spelling and capitalization.\nEnter rune levels as numbers (green=1, blue=2, purple=3, gold=4).\nJust press enter on both steps if no rune equipped to that skill.");

    for i in 0..=5 {
        println!("=={:?}==", skill_names[i]);
        runes[i].rune = get_rune_name();
        runes[i].rune_level = get_rune_level();
    }

    runes
}