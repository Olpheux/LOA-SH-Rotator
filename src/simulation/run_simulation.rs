use crate::character::meta_structs::Character as Character;
use crate::character::other_structs::Skill as Skill;
use rand::Rng;

mod structs;
mod init;
mod timers;


//==========
// Crits

fn roll_crit(character: &Character, skill: &Skill) -> i64 {
    let mut crits: i64 = 0;
    
    for _x in 0..skill.hits{
        let mut rng = rand::thread_rng();
        let mut roll = rng.gen_range(0..10000);

        if character.equipment.sets.hallucination >= 4 { roll += 1500; } 
        if character.equipment.sets.hallucination >= 6 { roll += 500; } 
        if (roll as f64) < (character.stats.crit_chance * 100.0) { crits += 1; }
    }

    crits
}

// Crits
//==========
// Deal damage

fn deal_damage(skill: &Skill, character: &Character, timers: &mut structs::TimerManager, crits: i64) -> f64 {
    let mut damage_dealt: f64 = skill.result_damage;

    if timers.damage_boost > 0.0 { damage_dealt *= 1.06; }
    if character.equipment.sets.hallucination >= 6 { damage_dealt *= 1.12; } 

    if timers.rune_timers.rage > 0.0 &&
        character.equipment.engravings.raid_captain > 0 { 
            damage_dealt *= rage_raid_captain(character, timers); 
    }

    if crits > 0 { 
        let damage_per_hit: f64 = damage_dealt / skill.hits as f64;
        let damage_per_crit: f64 = (damage_dealt / skill.hits as f64) * character.stats.crit_damage_multi;
    
        damage_dealt = (damage_per_crit * crits as f64) +
                       (damage_per_hit * (skill.hits - crits) as f64);
    }

    if timers.rune_timers.bleed > 0.0 { damage_dealt += bleed_damage(&mut timers.rune_timers, skill, character.stats.attack_power); }

    damage_dealt
}

// Deal damage
//=========
// Damage helpers

fn bleed_damage(timers: &mut structs::RuneTimers, skill: &Skill, attack_power: i64) -> f64 {
    timers.bleed_tick += skill.cast_time + 0.4;
    timers.bleed -= skill.cast_time + 0.4;
    
    let bleed_damage: f64 = (attack_power as f64 * 1.5) * (timers.bleed_tick / 1.0).floor(); // Unconfirmed, but testing suggests it's damage=1.5x AP.
    timers.bleed_tick %= 1.0;

    bleed_damage
}

fn rage_raid_captain(character: &Character, timers: &structs::TimerManager) -> f64 {
    // Determine remaining overhead before hitting move speed cap,
    // then add the bonus damage Rage's bonus move speed + Raid Captain would give you
    let move_speed_remaining: f64 = 40.0 - character.stats.move_speed;
    let raid_captain_bonus: f64 = move_speed_remaining.min(timers.rune_timers.rage_level as f64 * 4.0);
    match character.equipment.engravings.raid_captain {
        1 => (0.1 * raid_captain_bonus) + 1.0,
        2 => (0.22 * raid_captain_bonus) + 1.0,
        3 => (0.45 * raid_captain_bonus) + 1.0,
        _ => 1.0
    } 
}

// Damage helpers
//=========
// Cooldown managers

fn check_if_off_cooldown (timers: &mut structs::TimerManager, available_skills: &mut Vec<Skill>, skills: &[Skill; 6]) {
    let cd_array = timers.cooldowns.to_array();
    for x in 0..6 { if cd_array[x] == 0.0 { available_skills.push(skills[x].clone()); }}
    timers.cooldowns.replace_with_array(cd_array);
}

fn put_on_cooldown (timers: &mut structs::TimerManager, skill: &mut Skill) {
    if timers.rune_timers.judgement > 0.0 {
        skill.cooldown -= skill.cooldown * 0.15;
    }

    match skill.name.as_str() {
        "Ruining Rush" => timers.cooldowns.ruining_rush_cd = skill.cooldown,
        "Death Claw" => timers.cooldowns.death_claw_cd = skill.cooldown,
        "Destruction" => timers.cooldowns.destruction_cd = skill.cooldown,
        "Gore Bleeding" => timers.cooldowns.gore_bleeding_cd = skill.cooldown,
        "Leaping Blow" => timers.cooldowns.leaping_blow_cd = skill.cooldown,
        "Blood Massacre" => timers.cooldowns.blood_massacre_cd = skill.cooldown,
        _ => panic!("Tried to put a non-existant skill on cooldown!")
    }
}

// Cooldown managers
//==========
// Rune managers

fn rage_manager(timers: &mut structs::RuneTimers, skill: &Skill){
    let mut rng = rand::thread_rng();
    if skill.rune_level >= timers.rage_level &&
       rng.gen_range(0..100) < 15 { // Unconfirmed, but testing suggests 15% trigger rate.
            timers.rage = 6.0;
            timers.rage_level = skill.rune_level;
    }
}

fn quick_recharge_manager(cooldowns: &mut structs::Cooldowns, skills: &[Skill; 6], rune_level: i64) {
    let mut rng = rand::thread_rng();
    
    if rng.gen_range(0..100) < 20 { // Unconfirmed, but testing suggests 20% trigger rate.
        let mut cd_array = cooldowns.to_array();
        for i in 0..6 { cd_array[i] -= skills[i].cooldown * (1.0 - ((4 * rune_level) / 100) as f64) }
        cooldowns.replace_with_array(cd_array);
    }
}

fn conviction_manager(timers: &mut structs::RuneTimers, level: i64) {
    let mut rng = rand::thread_rng();
    
    if rng.gen_range(0..100) < (10 * level) { 
        timers.conviction = 3.0;
        timers.conviction_icd = 30.0;
    }
}

fn judgement_manager(timers: &mut structs::RuneTimers, level: i64) {
    let mut rng = rand::thread_rng();
    
    if rng.gen_range(0..100) < (10 * level) && timers.conviction > 0.0 { 
        timers.judgement = 6.0;
    }
}

fn bleed_manager(timers: &mut structs::RuneTimers, level: i64) {
    timers.bleed = match level {
        1 => 3.0,
        2 => 4.0,
        3 => 5.0,
        4 => 6.0,
        _ => timers.bleed
    };
}

fn apply_runes(timers: &mut structs::TimerManager, skill: &Skill, skill_list: &[Skill; 6]){
    match skill.rune.as_str() {
        "Bleed" => bleed_manager(&mut timers.rune_timers, skill.rune_level),
        "Rage" => rage_manager(&mut timers.rune_timers, skill),
        "Quick Recharge" => quick_recharge_manager(&mut timers.cooldowns, skill_list, skill.rune_level),
        "Conviction" => conviction_manager(&mut timers.rune_timers, skill.rune_level),
        "Judgement" => judgement_manager(&mut timers.rune_timers, skill.rune_level),
        "Galewind" | "Overwhelm" | "Protection" |
        "Purify" | "Focus" | "Iron Wall" | "None" | "" => (),
        _ => panic!("ERROR! Trying to activate a non-existent rune! This error shouldn't be reachable!")
    }
}

// Rune managers
//==========
// Main

pub fn run(character: &Character) -> (f64, Vec<Skill>){
    // Setup
    let mut timers = init::setup_timers(character);
    let skills = init::create_skill_list(character.clone());
    let mut available_skills = init::create_skill_list(character.clone()).to_vec();
    // These premature assignments avoid "possibly uninitialized" errors
    let mut total_damage: f64 = 0.0;
    let mut rotation: Vec<Skill> = [].to_vec();


    // Run simulation
    while timers.demon_duration > 0.0 {
        // Do nothing for 0.1 sec if no skills available
        if available_skills.is_empty() {
            timers::time_passed(&mut timers, -0.1);
            
            // Add back to available skill list if it's off cooldown now
            check_if_off_cooldown(&mut timers, &mut available_skills, &skills);
            available_skills.sort_by_key(|x| x.name.clone());
            available_skills.dedup();

        } else {
            // Pick a skill
            let mut chosen_skill = available_skills.remove((rand::random::<f32>() * available_skills.len() as f32).floor() as usize);
            let crits = roll_crit(character, &chosen_skill);

            // Apply its damage
            total_damage += deal_damage(&chosen_skill, character, &mut timers, crits);
            rotation.push(chosen_skill.clone());

            // Cleanup
            if crits >= 1 { timers.hallucination.crit(); }
            if chosen_skill.name == "Ruining Rush" || chosen_skill.name == "Death Claw" { timers.damage_boost = 6.0; }
            
            // Update timers
            if timers.rune_timers.rage > 0.0 && chosen_skill.rune == "Galewind" {
                let galewind: f64 = match chosen_skill.rune_level {
                    1 => 0.05,
                    2 => 0.08,
                    3 => 0.12,
                    4 => 0.14,
                    _ => 0.0
                };

                timers::time_passed(&mut timers.clone(), ((chosen_skill.cast_time * ( 1.0 - galewind - (timers.rune_timers.rage_level * 4 / 100) as f64)) + 0.4) * -1.0);
            } else if timers.rune_timers.rage > 0.0 {
                timers::time_passed(&mut timers.clone(), ((chosen_skill.cast_time * ( 1.0 - (timers.rune_timers.rage_level * 4 / 100) as f64)) + 0.4) * -1.0);
            } else if chosen_skill.rune == "Galewind" {
                let galewind: f64 = match chosen_skill.rune_level {
                    1 => 0.05,
                    2 => 0.08,
                    3 => 0.12,
                    4 => 0.14,
                    _ => 0.0
                };

                timers::time_passed(&mut timers.clone(), (chosen_skill.cast_time * (1.0 - galewind) + 0.4) * -1.0);
            } else {
                timers::time_passed(&mut timers, (chosen_skill.cast_time + 0.4) * -1.0);
            }

            put_on_cooldown(&mut timers, &mut chosen_skill);
            check_if_off_cooldown(&mut timers, &mut available_skills, &skills);
            apply_runes(&mut timers, &chosen_skill, &skills);

            // Make sure no duplicates exist in available_skills
            available_skills.sort_by_key(|x| x.name.clone());
            available_skills.dedup();
        }
    }

    (total_damage, rotation)
}