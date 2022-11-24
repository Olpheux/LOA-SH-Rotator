#![allow(non_snake_case)]
#![allow(clippy::needless_range_loop)] // It's wrong in this case; we need the actual position in the range.
mod character;
mod setup;
mod simulation;
use crate::character::meta_structs::Character;

fn main(){
    let character: Character = setup::init::init();
    println!("Setup complete!");
    simulation::start_simulation(&character);
}
