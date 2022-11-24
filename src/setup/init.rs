use crate::character::other_structs::RuneLevels;
use crate::setup::setup_from_input;
use crate::setup::setup_from_file;
use crate::setup::setup_skills;
use crate::character::meta_structs::Character as char_struct;
use std::io::ErrorKind;

pub fn init() -> char_struct{
    let filename = "./config.json";
    let mut character: char_struct;
    let runes: Vec<RuneLevels>;
    
    let file_opened = std::fs::read_to_string(filename);
    
    match file_opened {
        Ok(_) => {
            character = setup_from_file::setup_from_file((file_opened.as_ref()).unwrap()); //I don't know why this needs .as_ref() instead of & either.
            runes = setup_from_file::get_runes::get(&(file_opened).unwrap());
        }
        Err(error) => { 
            match error.kind() {
                ErrorKind::NotFound => println!("Config file not found."),
                ErrorKind::PermissionDenied => println!("Read permissions denied."),
                ErrorKind::InvalidData => println!("Unable to read file properly! May contain non-UTF8 characters?"),
                _ => println!("Unknown error! Error seen: {:?}", error.kind())
            }
            println!("Falling back to input-based setup...");
            character = setup_from_input::setup_from_input();
            runes = setup_from_input::get_runes::get();
        }
    };

    character.skills = setup_skills::skill_setup(&character.stats, runes);

    character
}