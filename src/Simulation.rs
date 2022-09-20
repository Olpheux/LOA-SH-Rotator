use rand::seq::SliceRandom;
use rand::Rng;
use thousands::Separable;

pub fn start_simulation(skills: Vec<Skill>, demon_duration: f64, crit_chance: f64, hallucination_set: i64){
    // current_damage and current_rotation could be unused if 0 iterations requested for some reason
    // This makes annoying warnings, so need to put underscore before their name to tell
    // the compiler that this is intended behavior.
    let mut best_found_damage = 0.0;
    let mut best_found_rotation: Vec<Skill> = Vec::new();
    let _current_damage: f64;
    let _current_rotation: Vec<Skill> = Vec::new();

    println!("How many rotations do we generate?");
    println!("A larger number will likely give a better result, but will take longer.");
    let mut iterations_input = String::new();    
    io::stdin()
        .read_line(&mut iterations_input)
        .expect("Failed to read input.");
    let iterations = iterations_input.trim().parse::<i32>()
        .expect("That doesn't look like an integer.");

    for _x in 1..=iterations {
        let (current_damage, current_rotation) = new_rotation(skills.clone(), demon_duration, crit_chance, hallucination_set);
        if current_damage > best_found_damage {
            best_found_damage = current_damage;
            best_found_rotation = current_rotation;
        }
    }

    println!("Completed {} attempts. Best result found:", iterations);
    println!(""); // Just a line break
    for x in best_found_rotation.clone() { print!("{} -> ", x.name); }
    println!("Demonize expires.");
    println!("");
    for x in best_found_rotation.clone() { print!("{} -> ", x.keybind); }
    println!("Demonize expires.");
    println!("Total damage dealt: {}", best_found_damage.round().separate_with_commas());
}

// TODO: Clean this up.
pub fn new_rotation(skills: Vec<Skill>, demon_duration: f64, crit_chance: f64, hallucination_set: i64) -> (f64, Vec<Skill>) {
    let mut cooldowns: [f64; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let mut available_skills: Vec<Skill> = skills.clone().to_owned();
    let mut ruining_rush_bonus = 0.0;
    let mut death_claw_bonus = 0.0;

    let mut total_damage: f64 = 0.0;
    let mut rotation: Vec<Skill> = [].to_vec();

    let mut demon_time_remaining = demon_duration.clone();

    let mut hallucination_timer = 0.0;
    let mut hallucination_duration = 0.0;
    let mut reality_timer = 0.0;
    let mut crit: bool;
 
    // Pick a random skill from the list of available ones
    // Remove it from list of available skills
    // Decrement all cooldowns and demon duration by its cast time + 0.4 sec GCD
    // Check if any cooldowns are negative or exactly 0
    // If so, set cooldown to 0, add it to available_skills if not there
    // Add chosen skill's damage to the total damage dealt
    // Add chosen skill to the rotation
    // Put chosen skill on cooldown
    while demon_time_remaining > 0.0 {
        let chosen_skill = available_skills.choose(&mut rand::thread_rng()).clone();

        if chosen_skill.is_none(){
            // If no skills are available, wait for 1/10th of a second.
            for x in 0..cooldowns.len() {
                cooldowns[x] -= 0.1;
                if cooldowns[x] <= 0.0 { 
                    cooldowns[x] = 0.0; 
                    // If this wait means the skill is now available, make it usable
                    if !available_skills.contains(&skills[x].clone()){
                        available_skills.push(skills[x].clone());
                    }
                };
            }
            demon_time_remaining -= 0.1;
            ruining_rush_bonus -= 0.1;
            death_claw_bonus -= 0.1;
            hallucination_timer -= 0.1;
            reality_timer -= 0.1;
        } else {
            let unwrapped_skill = chosen_skill.unwrap().clone();

            for x in 0..cooldowns.len() {
                cooldowns[x] -= unwrapped_skill.cast_time + 0.4;
                if cooldowns[x] <= 0.0 { 
                    cooldowns[x] = 0.0; 
                    // If this wait means the skill is now available, make it usable
                    if !available_skills.contains(&skills[x].clone()){
                        available_skills.push(skills[x].clone());
                    }
                };
            }

            // Check for damage amplifiers && Hallucination set
            // Hallucination is a huge pain due to having 3 different timers
            // to track that randomly trigger. 
            if hallucination_set >= 6 {
                for _y in 1..=unwrapped_skill.hits{
                    crit = roll_crit(crit_chance);
                    if crit && hallucination_timer <= 0.0 {
                        hallucination_timer = 6.0;
                    } else if crit {
                        hallucination_timer += 1.0;
                    }
                }

                if (ruining_rush_bonus > 0.0 || death_claw_bonus > 0.0) && (reality_timer > 0.0){
                    total_damage += (unwrapped_skill.result_damage) * 1.23;
                } else if (ruining_rush_bonus > 0.0 || death_claw_bonus > 0.0) && !(reality_timer > 0.0) {
                    total_damage += (unwrapped_skill.result_damage) * 1.06;
                } else if !(ruining_rush_bonus > 0.0 || death_claw_bonus > 0.0) && (reality_timer > 0.0) { 
                    total_damage += (unwrapped_skill.result_damage) * 1.17;
                }else {
                    total_damage += unwrapped_skill.result_damage;
                }
            } else {
                if ruining_rush_bonus > 0.0 || death_claw_bonus > 0.0 {
                    total_damage += (unwrapped_skill.result_damage) * 1.06;
                } else {
                    total_damage += unwrapped_skill.result_damage;
                }
            }

            // Reset timer on damage amplifiers if it was used
            if unwrapped_skill.name == "Ruining Rush" { ruining_rush_bonus = 6.0; }
            else if unwrapped_skill.name == "Death Claw" { death_claw_bonus = 6.0; }

            // Update timers
            demon_time_remaining -= unwrapped_skill.cast_time + 0.4;
            ruining_rush_bonus -= unwrapped_skill.cast_time + 0.4;
            death_claw_bonus -= unwrapped_skill.cast_time + 0.4;

            if hallucination_set >= 6 {
                hallucination_timer -= unwrapped_skill.cast_time + 0.4;
                hallucination_duration += unwrapped_skill.cast_time + 0.4;

                if hallucination_duration > 9.0 && reality_timer <= 0.0 {
                    reality_timer = 90.0;
                    hallucination_duration = 0.0;
                } else if reality_timer > 0.0 {
                    reality_timer -= unwrapped_skill.cast_time + 0.4;
                }

                if hallucination_timer <= 0.0 {
                    hallucination_duration = 0.0;
                }
            }

            // Put skill used on cooldown and add it to the rotation
            available_skills.retain(|x| *x != unwrapped_skill);
            let skill_index = skills.iter().position(|x| *x == unwrapped_skill).unwrap();
            cooldowns[skill_index] = skills[skill_index].cooldown;

            rotation.push(unwrapped_skill.clone());
        }
    }
    return (total_damage, rotation);
}

pub fn roll_crit(crit_chance: f64) -> bool{
    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(0..10000);

    return (roll as f64) < (crit_chance * 100.0);
}