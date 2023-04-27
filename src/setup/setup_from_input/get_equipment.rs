use crate::character::{equipment_structs as Equipment, meta_structs as Meta};
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

fn get_f64_input() -> f64 {
    match try_read!() {
        Ok(user_input) => user_input,
        Err(_) => {
            println!("Failed to parse input as a number. Maybe you included the symbols? Try again: ");
            get_f64_input()
        }
    }
}

//=============

fn get_cooldown_gem() -> i64 {
    println!("Demon Skill cooldown gem level: ");
    
    let gem_level: i64 = get_i64_input();

    if gem_level > 10 {
        println!("Gem level cannot be above 10.");
        get_cooldown_gem()
    } else if gem_level < 0 {
        println!("Gem level cannot be negative.");
        get_cooldown_gem()
    } else {
        gem_level
    }
}

fn get_damage_gem() -> i64 {
    println!("Demon Skill damage gem level: ");
    
    let gem_level: i64 = get_i64_input();

    if gem_level > 10 {
        println!("Gem level cannot be above 10.");
        get_damage_gem()
    } else if gem_level == 10 {
        40
    } else if gem_level == 9 {
        30
    } else if gem_level < 0 {
        println!("Gem level cannot be negative.");
        get_cooldown_gem()
    } else {
        gem_level * 3
    }
}

fn get_weapon_damage() -> f64 {
    println!("Weapon Additional Damage stat:");
    println!("(Omit the symbols. For instance, \"+11.50%\" would be entered as 11.50.)");

    get_f64_input()
}

fn get_other_gear() -> Equipment::OtherGear {
    Equipment::OtherGear{
        cooldown_gem: get_cooldown_gem(),
        attack_power_gem: get_damage_gem(),
        weapon_damage: get_weapon_damage()
    }
}

//==========

fn get_cards() -> Equipment::Cards {
    let mut cards = Equipment::Cards{
        lostwind_cliff: 0,
        light_of_salvation: 0
    };

    println!("Awakening level on Lostwind Cliff card deck: ");
    println!("(If unequipped, enter 0, regardless of level unlocked.)");
    cards.lostwind_cliff = get_i64_input();
    println!("Awakening level on Light Of Salvation card deck: ");
    println!("(If unequipped, enter 0, regardless of level unlocked.)");
    cards.light_of_salvation = get_i64_input();    

    cards
}

//==========

fn get_gearset() -> Equipment::Gearset {
    let mut gearset = Equipment::Gearset{
        preordained: 0,
        demon_beast: 0,
        hallucination: 0,
        salvation: 0,
        hallucination2: 0,
        salvation2: 0
    };

    println!("Equipped pieces of Preordained set:");
    gearset.preordained = get_i64_input();
    println!("Equipped pieces of Demon Beast set:");
    gearset.demon_beast = get_i64_input();
    println!("Equipped pieces of Hallucination set (level 1):");
    gearset.hallucination = get_i64_input();
    println!("Equipped pieces of Salvation set (level 1):");
    gearset.salvation = get_i64_input();
    println!("Equipped pieces of Hallucination set (level 2):");
    gearset.hallucination2 = get_i64_input();
    println!("Equipped pieces of Salvation set (level 2):");
    gearset.salvation2 = get_i64_input();

    if gearset.preordained +
       gearset.demon_beast +
       gearset.hallucination +
       gearset.salvation > 6 {
        println!("ERROR: You cannot have more than 6 pieces equipped!");
        println!("Resetting gear...");
        get_gearset()
    } else {
        gearset
    }
}

//==========

fn get_engravings() -> Equipment::Engravings {
    let mut engravings = Equipment::Engravings{
        demonic_impulse: 0,
        grudge: 0,
        cursed_doll: 0,
        raid_captain: 0,
        spirit_absorption: 0,
        adrenaline: 0,
        hit_master: 0,
        keen_blunt: 0,
        attack_power_reduction: 0,
        attack_speed_reduction: 0
    };

    // Helper function to ensure legal engraving levels
    fn get_single_engraving() -> i64 {
        let level = get_i64_input();
        if level > 3 { println!("Engraving level cannot be above 3. Try again: "); get_single_engraving() }
        else if level < 0 { println!("Engraving level cannot be negative. Try again: "); get_single_engraving() }
        else { level }
    }

    println!("Enter level in the following engravings:");

    println!("Demonic Impulse: ");
    engravings.demonic_impulse = get_single_engraving();
    println!("Grudge: ");
    engravings.grudge = get_single_engraving();
    println!("Cursed Doll: ");
    engravings.cursed_doll = get_single_engraving();
    println!("Raid Captain: ");
    engravings.raid_captain = get_single_engraving();
    println!("Spirit Absorption: ");
    engravings.spirit_absorption = get_single_engraving();
    println!("Adrenaline: ");
    engravings.adrenaline = get_single_engraving();
    println!("Hit Master: ");
    engravings.hit_master = get_single_engraving();
    println!("Keen Blunt Weapon: ");
    engravings.keen_blunt = get_single_engraving();
    println!("Attack Power Reduction: ");
    engravings.attack_power_reduction = get_single_engraving();
    println!("Attack Speed Reduction: ");
    engravings.attack_speed_reduction = get_single_engraving();

    engravings
}

//==========

pub fn get() -> Meta::EquipmentBonuses {
    Meta::EquipmentBonuses{
        other_gear: get_other_gear(),
        cards: get_cards(),
        sets: get_gearset(),
        engravings: get_engravings()
    }
}