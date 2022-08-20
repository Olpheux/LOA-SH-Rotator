pub fn getAP() -> i32 {
    println!("Attack power: ");

    let mut ap_input = String::new();

    io::stdin()
        .read_line(&mut ap_input)
        .expect("Failed to read input.");
    
    return ap_input.trim().parse::<i32>()
        .expect("That doesn't look like an integer.");
}

pub fn getCDGem() -> i32 {
    println!("Demon Skill cooldown gem level: ");

    let mut cd_gem_input = String::new();

    io::stdin()
        .read_line(&mut cd_gem_input)
        .expect("Failed to read input.");
    
    return cd_gem_input.trim().parse::<i32>()
        .expect("That doesn't look like an integer.");
}

pub fn getAPGem() -> i32 {
    println!("Demon Skill damage gem level: ");

    let mut ap_gem_input = String::new();
    io::stdin()
        .read_line(&mut ap_gem_input)
        .expect("Failed to read input.");
    
    let ap_gem = ap_gem_input.trim().parse::<i32>()
        .expect("That doesn't look like an integer.");

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

pub fn getCrit() -> i32 {
    println!("Crit (stat, not percent): ");

    let mut crit_input = String::new();

    io::stdin()
        .read_line(&mut crit_input)
        .expect("Failed to read input.");
    
    return crit_input.trim().parse::<i32>()
        .expect("That doesn't look like an integer.");
}

pub fn getSpec() -> i32 {
    println!("Specialization: ");

    let mut spec_input = String::new();

    io::stdin()
        .read_line(&mut spec_input)
        .expect("Failed to read input.");
    
    return spec_input.trim().parse::<i32>()
        .expect("That doesn't look like an integer.");
}

pub fn getSwift() -> i32 {
    println!("Swiftness: ");

    let mut swift_input = String::new();

    io::stdin()
        .read_line(&mut swift_input)
        .expect("Failed to read input.");
    
    return swift_input.trim().parse::<i32>()
        .expect("That doesn't look like an integer.");
}

pub fn getWD() -> f64 {
    println!("Weapon Additional Damage stat:");
    println!("(Omit the symbols. For instance, \"+11.50%\" would be entered as 11.50.)");

    let mut wd_input = String::new();

    io::stdin()
        .read_line(&mut wd_input)
        .expect("Failed to read input.");
    
    return wd_input.trim().parse::<f64>()
        .expect("Failed to parse. You may have accidentally included the symbols?");
}


//
// ACTUALLY CALCULATE THE USEFUL VALUES
//

pub fn calcDemonDuration(spec: i32) -> f64 { 20.0 * (1.0 + ((spec as f64) * 0.042916) / 100.0) }

pub fn calcCritChance(crit: i32, demonic_impulse: i32, adrenaline: i32) -> f64 {
    let mut crit_chance = crit as f64 * 0.03578;
    crit_chance = crit_chance + (((demonic_impulse - 1) * 15) as f64);
    crit_chance = crit_chance + ((adrenaline * 5) as f64);
    return crit_chance;
}

pub fn calcAtkSpeed(swift: i32, spirit_absorption: i32, as_reduction: i32) -> f64 {
    let mut attack_speed = swift as f64 * 0.01717;
    
    if spirit_absorption == 1 { attack_speed += 3.0; }
    else if spirit_absorption == 2 { attack_speed += 8.0; }
    else if spirit_absorption == 3 { attack_speed += 15.0; }

    attack_speed -= 2.0 * as_reduction as f64;

    return attack_speed;
}

pub fn calcModifiedAtkPower(attack_power: i32, cursed_doll: i32, adrenaline: i32, ap_reduction: i32) -> f64 {
    let mut cursed_doll_bonus = 0.0;
    let mut adrenaline_bonus = 0.0;
    let mut ap_reduction_penalty = 0.0;

    if cursed_doll == 1 {
        cursed_doll_bonus = (attack_power as f64) * 0.03;
    } else if cursed_doll == 2 {
        cursed_doll_bonus = (attack_power as f64) * 0.08;
    } else if cursed_doll == 3 {
        cursed_doll_bonus = (attack_power as f64) * 0.16;
    }

    if adrenaline == 1 {
        adrenaline_bonus = (attack_power as f64) * 0.018;
    } else if adrenaline == 2 {
        adrenaline_bonus = (attack_power as f64) * 0.036;
    } else if adrenaline == 3 {
        adrenaline_bonus = (attack_power as f64) * 0.06;
    }

    ap_reduction_penalty = (attack_power as f64) * (0.02 * ap_reduction as f64);

    return attack_power as f64 + cursed_doll_bonus + adrenaline_bonus - ap_reduction_penalty;
}

pub fn calcDamageModifiers(grudge: i32, raid_captain: i32, hit_master: i32, keen_blunt: i32, crit_chance: f64) -> f64{
    let mut grudge_bonus = 0.0;
    let mut raid_captain_bonus = 0.0;
    let mut hit_master_bonus = 0.0;
    let mut keen_blunt_bonus = 0.0;

    if grudge == 1 {
        grudge_bonus = 0.04;
    } else if grudge == 2 {
        grudge_bonus = 0.1;
    } else if grudge == 3 {
        grudge_bonus = 0.2;
    }

    if raid_captain > 0 {
        println!("Looks like you're using Raid Captain. What's your move speed bonus in Demonize?");
        println!("(Enter it without the symbols, and omit the base 100%.");
        println!("So if you have a move speed of 132.8%, enter 32.8)");

        let mut ms_input = String::new();
        io::stdin()
            .read_line(&mut ms_input)
            .expect("Failed to read input.");
    
        let ms_bonus = ms_input.trim().parse::<f64>()
            .expect("Failed to parse this. Did you include the % sign?");

        if raid_captain == 1 {
            raid_captain_bonus = ms_bonus * 0.1;
        } else if raid_captain == 2 {
            raid_captain_bonus = ms_bonus * 0.22;
        } else if raid_captain == 3 {
            raid_captain_bonus = ms_bonus * 0.4;
        }
    }

    if hit_master == 1 {
        hit_master_bonus = 0.03;
    } else if hit_master == 2 {
        hit_master_bonus = 0.08;
    } else if hit_master == 3 {
        hit_master_bonus = 0.16;
    }

    // "Else" clause here isn't really the Keen Blunt Weapon bonus, just the bonus
    // damage your crit chance earns you. That's a simple enough condition there's
    // no real benefit to calculating it elsewhere, especially when we're about to use that value.
    if keen_blunt == 1 {
        keen_blunt_bonus = (2.10 * crit_chance) + (1.0 - crit_chance) - 1.02;
    } else if keen_blunt == 2 {
        keen_blunt_bonus = (2.25 * crit_chance) + (1.0 - crit_chance) - 1.02;
    } else if keen_blunt == 3 {
        keen_blunt_bonus = (2.50 * crit_chance) + (1.0 - crit_chance) - 1.02;
    } else {
        keen_blunt_bonus = (2.0 * crit_chance) + (1.0 - crit_chance);
    }

    return grudge_bonus + raid_captain_bonus + hit_master_bonus + keen_blunt_bonus + 1.0;
}