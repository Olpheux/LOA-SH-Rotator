use serde_json::{Value};

pub fn setup_from_input() -> (Vec<Skill>, f64) {
    println!("No config file found. Falling back to command line setup.");
    println!("WARNING: Command line setup doesn't implement runes.");
    println!("We'll need some stats from your character to get started.");
    println!("Please fill out the following:");
    let baseline_attack_power = get_attack_power();

    let cd_gem = get_cooldown_gem();
    let ap_gem = get_damage_gem();

    let crit = get_crit();
    let spec = get_spec();
    let swift = get_swift();

    // Define function used in upcoming engraving setup
    fn get_engraving() -> i64 {
        let mut engraving_input = String::new();
    
        io::stdin()
            .read_line(&mut engraving_input)
            .expect("Failed to read input.");
        
        let engraving_level = engraving_input.trim().parse::<i64>()
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

fn setup_from_file(file: String) -> (Vec<Skill>, f64){
    // This could be cleaned up a lot if we can add a "bound_to(0..3)" method to i64?
    // Don't think that's possible, though.
    // This is a big mess of imports and there's probably a way to clean it up, but this works.
    let raw_character = serde_json::from_str::<Value>(&file).expect(".json formatting seems wrong.");
    
    // AP
    let mut baseline_attack_power = raw_character["Stats"][0]["AttackPower"].as_i64().expect("Attack power not found.");
    if !baseline_attack_power.is_positive(){
        baseline_attack_power = 0; 
        println!("WARNING: Attack power was negative. Setting to 0.");
    }

    // Gems
    let mut cd_gem = raw_character["Gems"][0]["CooldownGemLevel"].as_i64().expect("Cooldown gem not found.");
    let mut ap_gem = raw_character["Gems"][0]["DamageGemLevel"].as_i64().expect("Damage gem not found.");
    
    if !cd_gem.is_positive(){
        cd_gem = 0; 
        println!("WARNING: Cooldown gem level was negative. Setting to 0.");
    } else if cd_gem > 10 {
        cd_gem = 10; 
        println!("WARNING: Cooldown gem level was above 10. Setting to 10.");
    }

    if !ap_gem.is_positive(){
        ap_gem = 0; 
        println!("WARNING: Damage gem level was negative. Setting to 0.");
    } else if ap_gem > 10 {
        ap_gem = 10; 
        println!("WARNING: Damage gem level was above 10. Setting to 10.");
    }

    // Stats
    let mut crit = raw_character["Stats"][0]["Crit"].as_i64().expect("Crit stat not found.");
    let mut spec = raw_character["Stats"][0]["Spec"].as_i64().expect("Specialization stat not found.");
    let mut swift = raw_character["Stats"][0]["Swift"].as_i64().expect("Swiftness stat not found.");

    if !crit.is_positive(){
        crit = 0; 
        println!("WARNING: Crit stat was negative. Setting to 0.");
    }
    if !spec.is_positive(){
        spec = 0; 
        println!("WARNING: Specialization stat was negative. Setting to 0.");
    }
    if !swift.is_positive(){
        swift = 0; 
        println!("WARNING: Swiftness stat was negative. Setting to 0.");
    }

    // Engravings
    let mut demonic_impulse = raw_character["Engravings"][0]["DemonicImpulse"].as_i64().expect("Demonic Impulse engraving not found.");
    let mut grudge = raw_character["Engravings"][0]["Grudge"].as_i64().expect("Grudge engraving not found.");
    let mut cursed_doll = raw_character["Engravings"][0]["CursedDoll"].as_i64().expect("Cursed Doll engraving not found.");
    let mut raid_captain = raw_character["Engravings"][0]["RaidCaptain"].as_i64().expect("Raid Captain engraving not found.");
    let mut spirit_absorption = raw_character["Engravings"][0]["SpiritAbsorption"].as_i64().expect("Spirit Absorption engraving not found.");
    let mut adrenaline = raw_character["Engravings"][0]["Adrenaline"].as_i64().expect("Adrenaline engraving not found.");
    let mut hit_master = raw_character["Engravings"][0]["HitMaster"].as_i64().expect("Hit Master engraving not found.");
    let mut keen_blunt = raw_character["Engravings"][0]["KeenBluntWeapon"].as_i64().expect("Keen Blunt Weapon engraving not found.");
    let mut ap_reduction = raw_character["Engravings"][0]["AttackPowerReduction"].as_i64().expect("Attack Power Reduction engraving not found.");
    let mut as_reduction = raw_character["Engravings"][0]["AttackSpeedReduction"].as_i64().expect("Attack Speed Reduction engraving not found.");

    let mut engravings: [&i64; 10] = [&mut demonic_impulse, &mut grudge, &mut cursed_doll, &mut raid_captain, &mut spirit_absorption, &mut adrenaline, &mut hit_master, &mut keen_blunt, &mut ap_reduction, &mut as_reduction];
    for x in 0..=9 {
        if *engravings[x] > 3 { 
            engravings[x] = &3; 
            println!("WARNING: Engraving #{} was above 3. Setting to 3.", x+1);
        }
        else if *engravings[x] < 0 {
            engravings[x] = &0;
            println!("WARNING: Engraving #{} was negative. Setting to 0.", x+1);
        }
    }
    
    // Other stats
    let mut extra_weapon_damage = raw_character["Other"][0]["WeaponBonusDamage"].as_f64().expect("Weapon bonus damage not found.");
    let (lostwind_cliff, light_of_salvation) = (raw_character["Cards"][0]["LostwindCliff"].as_i64().expect("Lostwind Cliff not found."), 
                                                raw_character["Cards"][0]["LightOfSalvation"].as_i64().expect("Light of Salvation not found."));
    let mut move_speed = raw_character["Other"][0]["MoveSpeedBonus"].as_f64().expect("Move speed not found.");

    if !extra_weapon_damage.is_sign_positive(){
        extra_weapon_damage = 0.0; 
        println!("WARNING: Weapon bonus damage was negative. Setting to 0.");
    }
    // If card awakening levels are negative or comically large, we don't actually care, so don't bother checking.
    // Move speed can indeed be negative, due to move speed reduction engraving, so we don't check that.
    // (TODO: the bonus move speed from Demonize probably counteracts this. What's the actual floor?)
    if move_speed > 40.0{
        move_speed = 40.0; 
        println!("WARNING: Move speed is hard-capped to 140%. Setting move speed bonus to 40.0");
    }

    let demon_duration = calc_demon_duration(spec);
    let crit_chance = calc_crit_chance(crit, demonic_impulse, adrenaline, lostwind_cliff);
    let attack_speed = calc_attack_speed(swift, spirit_absorption, as_reduction);
    let attack_power = calc_modified_attack_power(baseline_attack_power, cursed_doll, adrenaline, ap_reduction);
    let damage_modifiers = calc_damage_modifier_from_file(grudge, raid_captain, move_speed, hit_master, keen_blunt, crit_chance, light_of_salvation);

    let skills = calc_modified_skills(cd_gem, ap_gem, swift, spec, extra_weapon_damage, attack_speed, attack_power, damage_modifiers);
    
    return (skills, demon_duration);
}

fn setup() -> (Vec<Skill>, f64){
    let character_file_name = "config.json";
    let file_opened = match std::fs::read_to_string(character_file_name) {
        Ok(file_opened) => file_opened,
        Err(_err) => return setup_from_input()
    };

    return setup_from_file(file_opened);
}