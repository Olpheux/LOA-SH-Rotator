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
    let baseline_attack_power = getAP();

    let cd_gem = getCDGem();
    let ap_gem = getAPGem();

    let crit = getCrit();
    let spec = getSpec();
    let swift = getSwift();

    // Define function used in upcoming engraving setup
    fn getEngraving() -> i32 {
        let mut engraving_input = String::new();
    
        io::stdin()
            .read_line(&mut engraving_input)
            .expect("Failed to read input.");
        
        let engraving_level = engraving_input.trim().parse::<i32>()
            .expect("That doesn't look like an integer.");
    
        if engraving_level < 0 || engraving_level > 3{
            println!("Acceptable range is 0 to 3.");
            return getEngraving();
        }
        else {
            return engraving_level;
        }
    }
    
    // This section could arguably be pushed over to GetStats.rs
    println!("What level do you have in the following engravings:");
    println!("Demonic Impulse: ");
    let demonic_impulse = getEngraving();
    println!("Grudge: ");
    let grudge = getEngraving();
    println!("Cursed Doll: ");
    let cursed_doll = getEngraving();
    println!("Raid Captain: ");
    let raid_captain = getEngraving();
    println!("Spirit Absorption: ");
    let spirit_absorption = getEngraving();
    println!("Adrenaline: ");
    let adrenaline = getEngraving();
    println!("Hit Master: ");
    let hit_master = getEngraving();
    println!("Keen Blunt Weapon: ");
    let keen_blunt = getEngraving();
    println!("Attack Power Reduction: ");
    let ap_reduction = getEngraving();
    println!("Attack Speed Reduction: ");
    let as_reduction = getEngraving();
    println!(" "); // Just a line break after getting engravings
    
    let extra_weapon_damage = getWD();

    let demon_duration = calcDemonDuration(spec);
    let crit_chance = calcCritChance(crit, demonic_impulse, adrenaline);
    let attack_speed = calcAtkSpeed(swift, spirit_absorption, as_reduction);
    let attack_power = calcModifiedAtkPower(baseline_attack_power, cursed_doll, adrenaline, ap_reduction);
    let damage_modifiers = calcDamageModifiers(grudge, raid_captain, hit_master, keen_blunt, crit_chance);

    let mut skills = calcModifiedSkills(cd_gem, ap_gem, swift, spec, extra_weapon_damage, attack_speed, attack_power, damage_modifiers);
    
    return (skills, demon_duration);
}


fn main(){
    let (skills, demon_duration) = setup();
    println!("Setup complete!");
    startSimulation(skills, demon_duration);
}
