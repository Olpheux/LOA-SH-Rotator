#![allow(non_snake_case)]
mod character;
mod setup;
mod simulation;
use crate::character::char_structs::meta_structs::Character;

fn main(){
    let character: Character = setup::init::init();
    println!("Setup complete!");
    simulation::start_simulation(&character);
}
