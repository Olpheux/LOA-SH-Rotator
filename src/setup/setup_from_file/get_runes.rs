use crate::character::other_structs::RuneLevels;
use serde_json::{Value};

pub fn get_rune_name(name: &Value) -> String {
    // Is there a better way to do this? Rust doesn't like string enums much...
    // Most of these runes don't actually do anything for DI SH, but I still want them all
    // so it doens't explode if someone's running a weird build for whatever reason.
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
    name != "None" &&
    !name.is_null() {
        println!("Rune {:?} not recognized. Ensure exact spelling and capitalization are correct.", name);
        panic!()
    } else {
        name.to_string().trim_matches('"').to_string()
    }
}

pub fn get_rune_level(level: &Value) -> i64 {
    match level.as_i64() {
        None => panic!("Couldn't read rune level!"),
        Some(level) => {
            if level.is_negative() {
                println!("WARNING: Cannot have negative rune level. Defaulting to 0.");
                0
            } else if level > 4 {
                println!("WARNING: Cannot have relic or above runes yet. Defaulting rune level to 4.");
                4
            } else {
                level
            }
        }
    }
}

//==========

pub fn get(raw_file: &str) -> Vec<RuneLevels> {
    let file = match serde_json::from_str::<Value>(raw_file) {
        Ok(file) => file,
        Err(_) => {
            println!("Unable to open file! If this issue persists, try renaming config.json to something else for input-based setup.");
            panic!()
        }
    };

    let mut runes: Vec<RuneLevels> = vec![RuneLevels { rune: "".to_string(), rune_level: 0 }; 6];

    for i in 0..=5 {
        runes[i].rune = get_rune_name(&file["Skills"][i]["Rune"]);
        runes[i].rune_level = get_rune_level(&file["Skills"][i]["RuneLevel"]);
    }

    runes
}