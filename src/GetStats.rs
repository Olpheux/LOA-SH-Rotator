pub fn get_i64_input() -> i64 {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input.");
    
    // Can this expect be revised to, instead of crashing,
    // instead call this function again? Isn't there an expect_or_else()?    
    return user_input.trim().parse::<i64>()
        .expect("That doesn't look like an integer.");
}

pub fn get_attack_power() -> i64 {
    println!("Attack power: ");
    return get_i64_input();
}

pub fn get_cooldown_gem() -> i64 {
    println!("Demon Skill cooldown gem level: ");
    return get_i64_input();
}

pub fn get_damage_gem() -> i64 {
    println!("Demon Skill damage gem level: ");
    
    let ap_gem = get_i64_input();

    // Because damage gems don't scale linearly, giving +3% damage per level until level 8,
    // but +30% at level 9, and +40% at level 10, we instead return just the actual damage value.
    // Most people won't have level 9 or 10 gems, so it's fine to special-case these.
    if ap_gem == 9 {
        return 30;
    } else if ap_gem == 10 {
        return 40;
    } else {
        return ap_gem * 3;
    }
}

pub fn get_crit() -> i64 {
    println!("Crit (stat, not percent): ");
    return get_i64_input();
}

pub fn get_spec() -> i64 {
    println!("Specialization: ");
    return get_i64_input();
}

pub fn get_swift() -> i64 {
    println!("Swiftness: ");
    return get_i64_input();
}

pub fn get_weapon_damage() -> f64 {
    println!("Weapon Additional Damage stat:");
    println!("(Omit the symbols. For instance, \"+11.50%\" would be entered as 11.50.)");

    let mut wd_input = String::new();

    io::stdin()
        .read_line(&mut wd_input)
        .expect("Failed to read input.");
    
    return wd_input.trim().parse::<f64>()
        .expect("Failed to parse. You may have accidentally included the symbols?");
}

pub fn get_cards() -> (i64, i64) {
    println!("Awakening level on Lostwind Cliff card deck: ");
    println!("(If unequipped, enter 0, regardless of level unlocked.)");
    let lostwind_cliff = get_i64_input();
    println!("Awakening level on Light Of Salvation card deck: ");
    println!("(If unequipped, enter 0, regardless of level unlocked.)");
    let light_of_salvation = get_i64_input();    

    return (lostwind_cliff, light_of_salvation);
}

pub fn get_gearset(setname: String) -> i64 {
    println!("Number of equipped pieces of the {} gear set: ", setname);
    return get_i64_input();
}

//
// ACTUALLY CALCULATE THE USEFUL VALUES
//

pub fn calc_demon_duration(spec: i64) -> f64 { 
    return (20.0 * (((spec as f64) * 0.042916) / 100.0)) + 20.0; 
}

pub fn calc_crit_chance(crit: i64, demonic_impulse: i64, adrenaline: i64, lostwind_cliff: i64, diligence_set: i64) -> f64 {
    // This neglects the Reality timer from the Hallucination set.
    // Only +5% crit, and realistically, you shouldn't be using Hallucination if
    // crit is a big part of your damage; Salvation is better on a Keen Blunt Weapon setup.
    let mut crit_chance = crit as f64 * 0.03578;

    if demonic_impulse >= 1 { crit_chance += ((demonic_impulse - 1) * 15) as f64; }
    crit_chance += (adrenaline * 5) as f64;

    if lostwind_cliff >= 30 { crit_chance += 15.0; }
    else if lostwind_cliff >= 18 { crit_chance += 7.0; }

    if diligence_set >= 5 { crit_chance += 25.0; }
    else if diligence_set >= 2 { crit_chance += 15.0; }

    return crit_chance;
}

pub fn calc_attack_speed(swift: i64, spirit_absorption: i64, as_reduction: i64, demon_beast_pieces: i64) -> f64 {
    let mut attack_speed = swift as f64 * 0.01717;
    
    if spirit_absorption == 1 { attack_speed += 3.0; }
    else if spirit_absorption == 2 { attack_speed += 8.0; }
    else if spirit_absorption == 3 { attack_speed += 15.0; }

    if demon_beast_pieces >= 4 { attack_speed += 10.0; }

    attack_speed -= 2.0 * as_reduction as f64;

    return attack_speed;
}

pub fn calc_modified_attack_power(attack_power: i64, cursed_doll: i64, adrenaline: i64, ap_reduction: i64) -> f64 {
    let mut cursed_doll_bonus = 0.0;
    let mut adrenaline_bonus = 0.0;
    let ap_reduction_penalty: f64;

    if cursed_doll == 1 { cursed_doll_bonus = (attack_power as f64) * 0.03; }
    else if cursed_doll == 2 { cursed_doll_bonus = (attack_power as f64) * 0.08; } 
    else if cursed_doll == 3 { cursed_doll_bonus = (attack_power as f64) * 0.16; }

    if adrenaline == 1 { adrenaline_bonus = (attack_power as f64) * 0.018; }
    else if adrenaline == 2 { adrenaline_bonus = (attack_power as f64) * 0.036; } 
    else if adrenaline == 3 { adrenaline_bonus = (attack_power as f64) * 0.06; }

    ap_reduction_penalty = (attack_power as f64) * (0.02 * ap_reduction as f64);

    return attack_power as f64 + cursed_doll_bonus + adrenaline_bonus - ap_reduction_penalty;
}

pub fn calc_crit_damage(keen_blunt: i64, diligence_set: i64) -> f64{
    let mut crit_damage_multi = 2.0;

    if keen_blunt == 3 { crit_damage_multi += 0.5; }
    else if keen_blunt == 2 { crit_damage_multi += 0.25; }
    else if keen_blunt == 1 { crit_damage_multi += 0.1; }

    if diligence_set >= 5 { crit_damage_multi += 0.5; }

    return crit_damage_multi;
}

// Lots of duplicated code here.
// TODO: Refactor this at some point.
pub fn calc_damage_modifier(grudge: i64, raid_captain: i64, hit_master: i64, keen_blunt: i64, crit_chance: f64, light_of_salvation: i64, preordained_set: i64, demon_beast_set: i64, salvation_set: i64, hallucination_set: i64) -> f64{
    let grudge_bonus: f64;
    let raid_captain_bonus: f64;
    let keen_blunt_bonus: f64;
    let mut light_of_salvation_bonus = 0.0;
    let mut hit_master_bonus = 0.0;
    let mut demon_beast_bonus = 0.0;
    let mut salvation_bonus = 0.0;
    let mut hallucination_bonus = 0.0;

    if grudge == 1 { grudge_bonus = 0.04; } 
    else if grudge == 2 { grudge_bonus = 0.1; } 
    else if grudge == 3 { grudge_bonus = 0.2; }
    else { grudge_bonus = 0.0; }

    if raid_captain == 0 { raid_captain_bonus = 0.0; }
    else {
        println!("Looks like you're using Raid Captain. What's your move speed bonus in Demonize?");
        println!("(Enter it without the symbols, and omit the base 100%.");
        println!("So if you have a move speed of 132.8%, enter 32.8)");

        let mut ms_input = String::new();
        io::stdin()
            .read_line(&mut ms_input)
            .expect("Failed to read input.");
    
        let ms_bonus = ms_input.trim().parse::<f64>()
            .expect("Failed to parse this. Did you include the % sign?");

        if raid_captain == 1 { raid_captain_bonus = ms_bonus * 0.1; } 
        else if raid_captain == 2 { raid_captain_bonus = ms_bonus * 0.22; }
        else if raid_captain == 3 { raid_captain_bonus = ms_bonus * 0.4; }
        else { raid_captain_bonus = 0.0; }
    }

    if hit_master == 1 { hit_master_bonus = 0.03; } 
    else if hit_master == 2 { hit_master_bonus = 0.08; } 
    else if hit_master == 3 { hit_master_bonus = 0.16; }

    let crit_damage = calc_crit_damage(keen_blunt, preordained_set);
    // "Else" clause here isn't really the Keen Blunt Weapon bonus, just the bonus
    // damage your crit chance earns you. That's a simple enough condition there's
    // no real benefit to calculating it elsewhere, though the variable name isn't great.
    if keen_blunt != 0 { keen_blunt_bonus = (crit_damage * crit_chance) + (1.0 - crit_chance) - 1.02; } 
    else { keen_blunt_bonus = (crit_damage * crit_chance) + (1.0 - crit_chance); }

    if light_of_salvation >= 30 { light_of_salvation_bonus = 0.15; }
    else if light_of_salvation >= 18 { light_of_salvation_bonus = 0.07; }

    if demon_beast_set >= 6 { demon_beast_bonus = 0.2; }
    else if demon_beast_set >= 2 { demon_beast_bonus = 0.1; }

    if salvation_set >= 6 { salvation_bonus = 0.47; }
    else if salvation_set >= 4 { salvation_bonus = 0.28; }
    else if salvation_set >= 2 { salvation_bonus = 0.14; }

    // Hallucination is weird, only really has a static bonus at 2- and 3-set. 4+ is weird. Gets computed within the simulation.    
    if hallucination_set == 2 || hallucination_set == 3 { hallucination_bonus = 0.866666; }

    return grudge_bonus + raid_captain_bonus + hit_master_bonus + keen_blunt_bonus + light_of_salvation_bonus + demon_beast_bonus + salvation_bonus + hallucination_bonus + 1.0;
}

// This is only separate to avoid additional request of user when using Raid Captain.
pub fn calc_damage_modifier_from_file(grudge: i64, raid_captain: i64, ms_bonus: f64, hit_master: i64, keen_blunt: i64, crit_chance: f64, light_of_salvation: i64, preordained_set: i64, demon_beast_set: i64, salvation_set: i64, hallucination_set: i64) -> f64{
    let grudge_bonus: f64;
    let raid_captain_bonus: f64;
    let keen_blunt_bonus: f64;
    let mut light_of_salvation_bonus = 0.0;
    let mut hit_master_bonus = 0.0;
    let mut demon_beast_bonus = 0.0;
    let mut salvation_bonus = 0.0;
    let mut hallucination_bonus = 0.0;

    if grudge == 1 { grudge_bonus = 0.04; } 
    else if grudge == 2 { grudge_bonus = 0.1; } 
    else if grudge == 3 { grudge_bonus = 0.2; }
    else { grudge_bonus = 0.0; }

    if raid_captain == 1 { raid_captain_bonus = ms_bonus * 0.1; } 
    else if raid_captain == 2 { raid_captain_bonus = ms_bonus * 0.22; }
    else if raid_captain == 3 { raid_captain_bonus = ms_bonus * 0.4; }
    else { raid_captain_bonus = 0.0; }

    if hit_master == 1 { hit_master_bonus = 0.03; } 
    else if hit_master == 2 { hit_master_bonus = 0.08; } 
    else if hit_master == 3 { hit_master_bonus = 0.16; }

    let crit_damage = calc_crit_damage(keen_blunt, preordained_set);
    // "Else" clause here isn't really the Keen Blunt Weapon bonus, just the bonus
    // damage your crit chance earns you. That's a simple enough condition there's
    // no real benefit to calculating it elsewhere, though the varible name isn't great.
    if keen_blunt != 0 { keen_blunt_bonus = (crit_damage * crit_chance) + (1.0 - crit_chance) - 1.02; } 
    else { keen_blunt_bonus = (crit_damage * crit_chance) + (1.0 - crit_chance); }

    if light_of_salvation >= 30 { light_of_salvation_bonus = 0.15; }
    else if light_of_salvation >= 18 { light_of_salvation_bonus = 0.07; }

    if demon_beast_set >= 6 { demon_beast_bonus = 0.2; }
    else if demon_beast_set >= 2 { demon_beast_bonus = 0.1; }

    if salvation_set >= 6 { salvation_bonus = 0.47; }
    else if salvation_set >= 4 { salvation_bonus = 0.28; }
    else if salvation_set >= 2 { salvation_bonus = 0.14; }

    // Hallucination is weird, only really has a static bonus at 2- and 3-set. 4+ is weird. Gets computed within the simulation.
    if hallucination_set == 2 || hallucination_set == 3 { hallucination_bonus = 0.866666; }
    
    return grudge_bonus + raid_captain_bonus + hit_master_bonus + keen_blunt_bonus + light_of_salvation_bonus + demon_beast_bonus + salvation_bonus + hallucination_bonus + 1.0;
}