use crate::character::char_structs::meta_structs::Character as Character;
use crate::character::char_structs::other_structs::Skill as Skill;
use rand::Rng;

//==========

#[derive(Debug,Clone,Default)]
pub struct Cooldowns {
    pub ruining_rush_cd: f64,
    pub death_claw_cd: f64,
    pub destruction_cd: f64,
    pub gore_bleeding_cd: f64,
    pub leaping_blow_cd: f64,
    pub blood_massacre_cd: f64
}

#[derive(Debug,Clone,Default)]
pub struct TimerManager {
    pub cooldowns: Cooldowns,
    pub damage_boost: f64,
    pub demon_duration: f64,
    pub hallucination_remaining: f64,
    pub hallucination_duration: f64,
    pub hallucination_cooldown: f64,
    pub reality: f64
}

//==========

fn create_skill_list(character: Character) -> [Skill; 6] {
    [character.skills.ruining_rush,
     character.skills.death_claw,
     character.skills.destruction,
     character.skills.gore_bleeding,
     character.skills.leaping_blow,
     character.skills.blood_massacre]
}

fn set_cooldowns() -> Cooldowns {
    Cooldowns{
        ruining_rush_cd: 0.0,
        death_claw_cd: 0.0,
        destruction_cd: 0.0,
        gore_bleeding_cd: 0.0,
        leaping_blow_cd: 0.0,
        blood_massacre_cd: 0.0
    }
}

//==========

pub fn time_passed(timers: &mut TimerManager, update: f64) -> &mut TimerManager {
    timers.cooldowns.ruining_rush_cd += update;
    timers.cooldowns.death_claw_cd += update;
    timers.cooldowns.destruction_cd += update;
    timers.cooldowns.gore_bleeding_cd += update;
    timers.cooldowns.leaping_blow_cd += update;
    timers.cooldowns.blood_massacre_cd += update;

    if timers.cooldowns.ruining_rush_cd <= 0.0 { timers.cooldowns.ruining_rush_cd = 0.0; }
    if timers.cooldowns.death_claw_cd <= 0.0 { timers.cooldowns.death_claw_cd = 0.0; }
    if timers.cooldowns.destruction_cd <= 0.0 { timers.cooldowns.destruction_cd = 0.0; }
    if timers.cooldowns.gore_bleeding_cd <= 0.0 { timers.cooldowns.gore_bleeding_cd = 0.0; }
    if timers.cooldowns.leaping_blow_cd <= 0.0 { timers.cooldowns.leaping_blow_cd = 0.0; }
    if timers.cooldowns.blood_massacre_cd <= 0.0 { timers.cooldowns.blood_massacre_cd = 0.0; }

    timers.demon_duration += update;
    timers.damage_boost += update;
    timers.hallucination_duration -= update; // This counts up, not down, so invert its behavior.
    timers.hallucination_cooldown += update;
    timers.hallucination_remaining += update;
    timers.reality += update;

    if timers.demon_duration < 0.0 { timers.demon_duration = 0.0 };
    if timers.damage_boost < 0.0 { timers.damage_boost = 0.0 };
    if timers.hallucination_cooldown < 0.0 { timers.hallucination_cooldown = 0.0 };
    if timers.reality < 0.0 { timers.reality = 0.0 };
    
    if timers.hallucination_remaining < 0.0 { 
        timers.hallucination_remaining = 0.0; 
        timers.hallucination_duration = 0.0; 
        // don't repeatedly reset hallucation cooldown
        if timers.hallucination_cooldown == 0.0 { timers.hallucination_cooldown = 3.0; }
    }

    if timers.hallucination_duration < 0.0 { timers.hallucination_duration = 0.0 };
    if timers.hallucination_duration > 9.0 { timers.reality = 40.0 };
    
    timers
}

fn time_hallucination_crit(timers: &mut TimerManager) -> &mut TimerManager{
    if timers.hallucination_remaining == 0.0 { timers.hallucination_remaining = 9.0; }
    else { timers.hallucination_remaining += 1.0; }

    timers
}

//==========

fn roll_crit(character: &Character, timers: &TimerManager, skill: &Skill) -> i64 {
    let mut crits: i64 = 0;
    
    for _x in 0..skill.hits{
        let mut rng = rand::thread_rng();
        let mut roll = rng.gen_range(0..10000);

        if (character.equipment.sets.hallucination >= 6) && (timers.reality > 0.0) {
            roll += 500;
        } 
        
        if (roll as f64) < (character.stats.crit_chance * 100.0) {
                crits += 1;
        }
    }

    crits
}

//==========

fn deal_damage(skill: &Skill, character: &Character, timers: &TimerManager, crits: i64) -> f64 {
    let mut damage_dealt: f64 = skill.result_damage;

    if timers.damage_boost > 0.0 { damage_dealt *= 1.06; }
    if timers.reality > 0.0 { damage_dealt *= 1.17; }
    
    if crits > 0 { 
        let damage_per_hit = damage_dealt / skill.hits as f64;
        let damage_per_crit = (damage_dealt / skill.hits as f64) * character.stats.crit_damage_multi;
    
        damage_dealt = (damage_per_crit * crits as f64) +
                       (damage_per_hit * (skill.hits - crits) as f64);
    }

    damage_dealt
}

//==========

pub fn run(character: &Character) -> (f64, Vec<Skill>){
    // Setup
    let mut timers = TimerManager{
        cooldowns: set_cooldowns(),
        damage_boost: 0.0,
        demon_duration: character.stats.demon_duration,
        hallucination_duration: 0.0,
        hallucination_remaining: 0.0,
        hallucination_cooldown: 0.0,
        reality: 0.0
    };
    let skills = create_skill_list(character.clone());
    let mut available_skills = create_skill_list(character.clone()).to_vec();
     // These premature assignments avoid "possibly uninitialized" errors
    let mut total_damage: f64 = 0.0;
    let mut rotation: Vec<Skill> = [].to_vec();


    // Run simulation
    while timers.demon_duration > 0.0 {
        // Do nothing for 0.1 sec if no skills available
        if available_skills.is_empty() {
            time_passed(&mut timers, -0.1);
            
            // Add back to available skill list if it's off cooldown now
            // Can probably be done with more elegantly with an iterator?
            if timers.cooldowns.ruining_rush_cd == 0.0 { available_skills.push(skills[0].clone()); }
            if timers.cooldowns.death_claw_cd == 0.0 { available_skills.push(skills[1].clone()); }
            if timers.cooldowns.destruction_cd == 0.0 { available_skills.push(skills[2].clone()); }
            if timers.cooldowns.gore_bleeding_cd == 0.0 { available_skills.push(skills[3].clone()); }
            if timers.cooldowns.leaping_blow_cd == 0.0 { available_skills.push(skills[4].clone()); }
            if timers.cooldowns.blood_massacre_cd == 0.0 { available_skills.push(skills[5].clone()); }
            available_skills.sort_by_key(|x| x.name.clone());
            available_skills.dedup();

        } else {
            // Pick a skill
            let chosen_skill = available_skills.remove((rand::random::<f32>() * available_skills.len() as f32).floor() as usize);
            let crits = roll_crit(character, &timers, &chosen_skill);

            // Apply its damage
            total_damage += deal_damage(&chosen_skill, character, &timers, crits);
            rotation.push(chosen_skill.clone());

            // Cleanup
            if crits >= 1 { time_hallucination_crit(&mut timers); }
            if chosen_skill.name == "Ruining Rush" || chosen_skill.name == "Death Claw" { timers.damage_boost = 6.0; }
            
            time_passed(&mut timers, (chosen_skill.cast_time + 0.4) * -1.0);

            // Put skill used on cooldown and add it to the rotation
                        // Can probably be done with more elegantly with an iterator?
            if chosen_skill.name == "Ruining Rush" { timers.cooldowns.ruining_rush_cd = chosen_skill.cooldown; }
            if chosen_skill.name == "Death Claw" { timers.cooldowns.death_claw_cd = chosen_skill.cooldown; }
            if chosen_skill.name == "Destruction" { timers.cooldowns.destruction_cd = chosen_skill.cooldown; }
            if chosen_skill.name == "Gore Bleeding" { timers.cooldowns.gore_bleeding_cd = chosen_skill.cooldown; }
            if chosen_skill.name == "Leaping Blow" { timers.cooldowns.leaping_blow_cd = chosen_skill.cooldown; }
            if chosen_skill.name == "Blood Massacre" { timers.cooldowns.blood_massacre_cd = chosen_skill.cooldown; }

            // Add everything back to available skill list if it's off cooldown now
            if timers.cooldowns.ruining_rush_cd == 0.0 { available_skills.push(skills[0].clone()); }
            if timers.cooldowns.death_claw_cd == 0.0 { available_skills.push(skills[1].clone()); }
            if timers.cooldowns.destruction_cd == 0.0 { available_skills.push(skills[2].clone()); }
            if timers.cooldowns.gore_bleeding_cd == 0.0 { available_skills.push(skills[3].clone()); }
            if timers.cooldowns.leaping_blow_cd == 0.0 { available_skills.push(skills[4].clone()); }
            if timers.cooldowns.blood_massacre_cd == 0.0 { available_skills.push(skills[5].clone()); }
            available_skills.sort_by_key(|x| x.name.clone());
            available_skills.dedup();
        }
    }

    (total_damage, rotation)
}