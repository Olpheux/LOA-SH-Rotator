#![allow(non_snake_case)]
use std::io;
include!("SkillSetup.rs");
include!("GetStats.rs");
include!("Simulation.rs");

fn setup() -> (Vec<Skill>, f64) {
    // TODO: Add an option to pull data from a .txt or something
    // TODO: Add an option to save your build to said .txt
    // This is a lot of work to set up every time. Need a better way to do this.
    println!("We'll need some stats from your character to get started.");
    println!("Please fill out the following:");
    let baseline_attack_power = get_attack_power();

    let cd_gem = get_cooldown_gem();
    let ap_gem = get_damage_gem();

    let crit = get_crit();
    let spec = get_spec();
    let swift = get_swift();

    // Define function used in upcoming engraving setup
    fn get_engraving() -> i32 {
        let mut engraving_input = String::new();
    
        io::stdin()
            .read_line(&mut engraving_input)
            .expect("Failed to read input.");
        
        let engraving_level = engraving_input.trim().parse::<i32>()
            .expect("That doesn't look like an integer.");
    
        if engraving_level < 0 || engraving_level > 3{
            println!("Acceptable range is 0 to 3.");
            return get_engraving();
        }
        else {
            return engraving_level;
        }
    }
    
    // This section could arguably be pushed over to GetStats.rs
    println!("What level do you have in the following engravings:");
    println!("Demonic Impulse: ");
    let demonic_impulse = get_engraving();
    println!("Grudge: ");
    let grudge = get_engraving();
    println!("Cursed Doll: ");
    let cursed_doll = get_engraving();
    println!("Raid Captain: ");
    let raid_captain = get_engraving();
    println!("Spirit Absorption: ");
    let spirit_absorption = get_engraving();
    println!("Adrenaline: ");
    let adrenaline = get_engraving();
    println!("Hit Master: ");
    let hit_master = get_engraving();
    println!("Keen Blunt Weapon: ");
    let keen_blunt = get_engraving();
    println!("Attack Power Reduction: ");
    let ap_reduction = get_engraving();
    println!("Attack Speed Reduction: ");
    let as_reduction = get_engraving();
    println!(" "); // Just a line break after getting engravings
    
    let extra_weapon_damage = get_weapon_damage();
    let (lostwind_cliff, light_of_salvation) = get_cards();

    let demon_duration = calc_demon_duration(spec);
    let crit_chance = calc_crit_chance(crit, demonic_impulse, adrenaline, lostwind_cliff);
    let attack_speed = calc_attack_speed(swift, spirit_absorption, as_reduction);
    let attack_power = calc_modified_attack_power(baseline_attack_power, cursed_doll, adrenaline, ap_reduction);
    let damage_modifiers = calc_damage_modifier(grudge, raid_captain, hit_master, keen_blunt, crit_chance, light_of_salvation);

    let skills = calc_modified_skills(cd_gem, ap_gem, swift, spec, extra_weapon_damage, attack_speed, attack_power, damage_modifiers);
    
    return (skills, demon_duration);
}


fn main(){
    let (skills, demon_duration) = setup();
    println!("Setup complete!");
    start_simulation(skills, demon_duration);
}
