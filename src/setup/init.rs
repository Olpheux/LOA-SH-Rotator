use crate::setup::setup_from_input;
use crate::setup::setup_from_file;
use crate::setup::setup_skills;
use crate::character::char_structs::meta_structs::Character as char_struct;
use std::path::Path;

pub fn init() -> char_struct{
    let filename = "./config.json";
    let character_file_exists = Path::new(filename).is_file();
    let mut character: char_struct;
    
    if character_file_exists {
        let file_opened = std::fs::read_to_string(filename); 
        character = setup_from_file::setup_from_file(&file_opened.unwrap());
    } else {
        character = setup_from_input::setup_from_input();
    };

    character.skills = setup_skills::skill_setup(&character.stats);

    character
}