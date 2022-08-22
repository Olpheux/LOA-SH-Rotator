pub fn get_i32_input() -> i32 {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input.");
    
    return user_input.trim().parse::<i32>()
        .expect("That doesn't look like an integer.");
}

pub fn get_attack_power() -> i32 {
    println!("Attack power: ");
    return get_i32_input();
}

pub fn get_cooldown_gem() -> i32 {
    println!("Demon Skill cooldown gem level: ");
    return get_i32_input();
}

pub fn get_damage_gem() -> i32 {
    println!("Demon Skill damage gem level: ");
    
    let ap_gem = get_i32_input();

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

pub fn get_crit() -> i32 {
    println!("Crit (stat, not percent): ");
    return get_i32_input();
}

pub fn get_spec() -> i32 {
    println!("Specialization: ");
    return get_i32_input();
}

pub fn get_swift() -> i32 {
    println!("Swiftness: ");
    return get_i32_input();
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

pub fn get_cards() -> (i32, i32) {
    println!("Awakening level on Lostwind Cliff card deck: ");
    println!("(If unequipped, enter 0, regardless of level unlocked.)");
    let lostwind_cliff = get_i32_input();
    println!("Awakening level on Light Of Salvation card deck: ");
    println!("(If unequipped, enter 0, regardless of level unlocked.)");
    let light_of_salvation = get_i32_input();    

    return (lostwind_cliff, light_of_salvation);
}
//
// ACTUALLY CALCULATE THE USEFUL VALUES
//

pub fn calc_demon_duration(spec: i32) -> f64 { 
    return (20.0 * (((spec as f64) * 0.042916) / 100.0)) + 20.0; 
}

pub fn calc_crit_chance(crit: i32, demonic_impulse: i32, adrenaline: i32, lostwind_cliff: i32) -> f64 {
    let mut crit_chance = crit as f64 * 0.03578;
    if demonic_impulse >= 1 { crit_chance += ((demonic_impulse - 1) * 15) as f64; }
    crit_chance += (adrenaline * 5) as f64;
    if lostwind_cliff >= 18 { crit_chance += 7.0; }
    return crit_chance;
}

pub fn calc_attack_speed(swift: i32, spirit_absorption: i32, as_reduction: i32) -> f64 {
    let mut attack_speed = swift as f64 * 0.01717;
    
    if spirit_absorption == 1 { attack_speed += 3.0; }
    else if spirit_absorption == 2 { attack_speed += 8.0; }
    else if spirit_absorption == 3 { attack_speed += 15.0; }

    attack_speed -= 2.0 * as_reduction as f64;

    return attack_speed;
}

pub fn calc_modified_attack_power(attack_power: i32, cursed_doll: i32, adrenaline: i32, ap_reduction: i32) -> f64 {
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

pub fn calc_damage_modifier(grudge: i32, raid_captain: i32, hit_master: i32, keen_blunt: i32, crit_chance: f64, light_of_salvation: i32) -> f64{
    let grudge_bonus: f64;
    let raid_captain_bonus: f64;
    let keen_blunt_bonus: f64;
    let mut light_of_salvation_bonus = 0.0;
    let mut hit_master_bonus = 0.0;

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

    // "Else" clause here isn't really the Keen Blunt Weapon bonus, just the bonus
    // damage your crit chance earns you. That's a simple enough condition there's
    // no real benefit to calculating it elsewhere, especially when we're about to use that value.
    if keen_blunt == 1 { keen_blunt_bonus = (2.10 * crit_chance) + (1.0 - crit_chance) - 1.02; } 
    else if keen_blunt == 2 { keen_blunt_bonus = (2.25 * crit_chance) + (1.0 - crit_chance) - 1.02; } 
    else if keen_blunt == 3 { keen_blunt_bonus = (2.50 * crit_chance) + (1.0 - crit_chance) - 1.02; } 
    else { keen_blunt_bonus = (2.0 * crit_chance) + (1.0 - crit_chance); }

    if light_of_salvation >= 30 { light_of_salvation_bonus = 0.15; }
    else if light_of_salvation >= 18 { light_of_salvation_bonus = 0.07; }

    return grudge_bonus + raid_captain_bonus + hit_master_bonus + keen_blunt_bonus + light_of_salvation_bonus + 1.0;
}