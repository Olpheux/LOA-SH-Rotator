use crate::character::{equipment_structs as Equipment, meta_structs as Meta};
use serde_json::{Value};

//==========
// Engravings
//==========

fn parse_engraving(wrapped_level: &Value, name: String) -> i64 {
    if !wrapped_level.is_i64(){ panic!("Failed to parse engraving level for {} as an integer.", name); }

    let level: i64 = wrapped_level.as_i64().unwrap();

    if level.is_negative() { 
        println!("Engraving levels can't be negative! Setting level of {} to 0.", name);
        0
    } else if level > 3 {
        println!("Engraving levels can't be higher than 3! Setting level of {} to 3.", name);
        3
    } else {
        level
    }
}

fn get_engravings(file: &Value) -> Equipment::Engravings {
    Equipment::Engravings{
        demonic_impulse: parse_engraving(&file["Engravings"][0]["DemonicImpulse"], String::from("Demonic Impulse")),
        grudge: parse_engraving(&file["Engravings"][0]["Grudge"], String::from("Grudge")),
        cursed_doll: parse_engraving(&file["Engravings"][0]["CursedDoll"], String::from("Cursed Doll")),
        raid_captain: parse_engraving(&file["Engravings"][0]["RaidCaptain"], String::from("Raid Captain")),
        spirit_absorption: parse_engraving(&file["Engravings"][0]["SpiritAbsorption"], String::from("Spirit Absorption")),
        adrenaline: parse_engraving(&file["Engravings"][0]["Adrenaline"], String::from("Adrenaline")),
        hit_master: parse_engraving(&file["Engravings"][0]["HitMaster"], String::from("Hit Master")),
        keen_blunt: parse_engraving(&file["Engravings"][0]["KeenBluntWeapon"], String::from("Keen Blunt Weapon")),
        attack_power_reduction: parse_engraving(&file["Engravings"][0]["AttackPowerReduction"], String::from("Attack Power Reduction")),
        attack_speed_reduction: parse_engraving(&file["Engravings"][0]["AttackSpeedReduction"], String::from("Attack Speed Reduction")),
    }
}

//==========
// Other gear
//==========

fn get_cooldown_gem(gem: &Value) -> i64 {
    let level: i64 = match gem.as_i64() {
        None => panic!("FAILED: Could not parse cooldown gem level! Value must be an integer."),
        Some(0..=10) => gem.as_i64().unwrap(),
        Some(std::i64::MIN..=-1) => {
            println!("WARNING: Cooldown gem level cannot be negative. Setting to 0 instead.");
            0
        },
        Some(11..=std::i64::MAX) => {
            println!("WARNING: Cooldown gem level cannot be greater than 10. Setting to 10 instead.");
            10
        }
    };

    level
}

fn get_damage_gem(gem: &Value) -> i64 {
    let level: i64 = match gem.as_i64() {
        None => panic!("FAILED: Could not parse damage gem level! Value must be an integer."),
        Some(0..=8) => gem.as_i64().unwrap(),
        Some(9) => 30,
        Some(10) => 40,
        Some(std::i64::MIN..=-1) => {
            println!("WARNING: Damage gem level cannot be negative. Setting to 0 instead.");
            0
        },
        Some(11..=std::i64::MAX) => {
            println!("WARNING: Damage gem level cannot be greater than 10. Setting to 10 instead.");
            10
        }
    };

    level
}

fn get_weapon_damage(damage: &Value) -> f64 {
    match damage.as_f64() {
        None => panic!("FAILED: Could not parse weapon damage! Value must be a decimal number."),
        Some(x) => if x.is_sign_negative() {
            println!("WARNING: Cannot have negative bonus weapon damage. Setting to 0 instead.");
            0.0
        } else { x }
    }
}

//==========

fn get_other_gear(file: &Value) -> Equipment::OtherGear {
    Equipment::OtherGear{
        cooldown_gem: get_cooldown_gem(&file["Gems"][0]["CooldownGemLevel"]),
        attack_power_gem: get_damage_gem(&file["Gems"][0]["DamageGemLevel"]),
        weapon_damage: get_weapon_damage(&file["Other"][0]["WeaponBonusDamage"]),
    }
}

//==========
// Cards
//==========

fn parse_cards(wrapped_cards: &Value) -> i64 {
    match wrapped_cards.as_i64() {
        None => panic!("FAILED: Could not parse card awakening levels! Value must be an integer."),
        Some(std::i64::MIN..=-1) => {
            println!("WARNING: Cannot have negative card awakening levels. Setting to 0 instead.");
            0
        },
        Some(x) => x 
    }
}

//==========

fn get_cards(file: &Value) -> Equipment::Cards {
    Equipment::Cards{
        lostwind_cliff: parse_cards(&file["Cards"][0]["LostwindCliff"]),
        light_of_salvation: parse_cards(&file["Cards"][0]["LightOfSalvation"])
    }
}

//==========
// Gearsets
//==========

fn parse_gearset(wrapped_gear: &Value) -> i64 {
    if !wrapped_gear.is_i64(){ panic!("Failed to parse gearsets."); }

    let gear = wrapped_gear.as_i64().unwrap();

    if gear > 6 {
        println!("Cannot have more than 6 pieces of a set equipped. Reverting to 6.");
        6
    } else if gear.is_negative() {
        println!("Cannot have negative pieces of a set equipped. Reverting to 0.");
        0
    } else {
        gear
    }
}

fn validate_gear(gear: Equipment::Gearset) -> Equipment::Gearset {
    if (gear.preordained +
        gear.demon_beast +
        gear.salvation +
        gear.hallucination) > 6 {
            println!("Cannot have more than 6 pieces of gear equipped. Reverting all to 0.");
            
            Equipment::Gearset{
                preordained: 0,
                demon_beast: 0,
                salvation: 0,
                hallucination: 0
            }
    } else {
         gear
    }
}

//==========

fn get_gearset(file: &Value) -> Equipment::Gearset {
    let gear = Equipment::Gearset{
                    preordained: parse_gearset(&file["GearSets"][0]["Preordained"]),
                    demon_beast: parse_gearset(&file["GearSets"][0]["DemonBeast"]),
                    salvation: parse_gearset(&file["GearSets"][0]["Salvation"]),
                    hallucination: parse_gearset(&file["GearSets"][0]["Hallucination"])
                };

    validate_gear(gear)
}

//==========
// main
//==========

pub fn get(raw_file: &str) -> Meta::EquipmentBonuses {
    let file = serde_json::from_str::<Value>(raw_file).expect(".json formatting seems wrong.");
    
     Meta::EquipmentBonuses{
        other_gear: get_other_gear(&file),
        cards: get_cards(&file),
        sets: get_gearset(&file),
        engravings: get_engravings(&file)
    }
}