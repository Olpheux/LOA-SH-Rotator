use crate::character::base_stats_struct::BaseStats as base_stats_struct;
use crate::character::meta_structs::EquipmentBonuses as equipment_struct;
use crate::character::other_structs::DerivedStats as derived_stats_struct;

//==========

fn calc_demon_duration(spec: i64) -> f64 { (20.0 * (((spec as f64) * 0.042916) / 100.0)) + 20.0 }

fn calc_crit_chance(equipment: &equipment_struct, crit: i64) -> f64 {
    let crit_stat_bonus = crit as f64 * 0.03578;

    let demonic_impulse_bonus: f64 = match equipment.engravings.demonic_impulse {
        0 | 1 => 0.0,
        2 => 15.0,
        3 => 30.0,
        _ => panic!() // Should be impossible to hit this, engravings get restricted to 0..=3 before this.
    };

    let lostwind_cliff_bonus: f64;
    if equipment.cards.lostwind_cliff >= 30 { lostwind_cliff_bonus = 15.0; }
    else if equipment.cards.lostwind_cliff >= 18 { lostwind_cliff_bonus = 7.0; }
    else { lostwind_cliff_bonus = 0.0; }

    let preordained_bonus: f64 = match equipment.sets.preordained {
        5 | 6 => 25.0,
        2 | 3 | 4 => 15.0,
        _ => 0.0
    };
    
    let hallucination_bonus: f64 = match equipment.sets.hallucination {
        4 | 5 | 6 => 15.0,
        _ => 0.0
    };

    crit_stat_bonus +
    demonic_impulse_bonus +
    lostwind_cliff_bonus +
    hallucination_bonus +
    preordained_bonus
}

fn calc_crit_damage(equipment: &equipment_struct) -> f64 {
    let keen_blunt_bonus: f64 = match equipment.engravings.keen_blunt{
        3 => 0.5,
        2 => 0.25,
        1 => 0.1,
        _ => 0.0
    };
    
    let preordained_bonus: f64 = match equipment.sets.preordained {
        5 | 6 => 0.5,
        _ => 0.0
    };

    2.0 +
    keen_blunt_bonus +
    preordained_bonus
}

//==========

fn calc_attack_speed(equipment: &equipment_struct, swift: i64) -> f64 {
    let swift_bonus: f64 = swift as f64 * 0.0001717;

    let spirit_absorption_bonus: f64 = match equipment.engravings.spirit_absorption {
        3 => 0.15,
        2 => 0.08,
        1 => 0.03,
        _ => 0.0
    };

    let attack_speed_reduction = (equipment.engravings.attack_speed_reduction as f64) * -2.0;

    let salvation_bonus: f64 = match equipment.sets.salvation {
        4 | 5 | 6 => 0.1,
        _ => 0.0
    };

    swift_bonus +
    spirit_absorption_bonus +
    salvation_bonus + 
    attack_speed_reduction
}

fn calc_cooldown_reduction(equipment: &equipment_struct, swift: i64) -> f64 {
    ((swift as f64) * 0.00021472) + ((equipment.other_gear.cooldown_gem as f64) * 0.02)
}

//=========

fn calc_modified_attack_power(equipment: &equipment_struct, ap: i64) -> i64 {
    let cursed_doll_bonus: f64 = match equipment.engravings.cursed_doll {
        3 => (ap as f64) * 0.16,
        2 => (ap as f64) * 0.08,
        1 => (ap as f64) * 0.03,
        _ => 0.0
    };

    let adrenaline_bonus: f64 = match equipment.engravings.adrenaline {
        3 => (ap as f64) * 0.06,
        2 => (ap as f64) * 0.036,
        1 => (ap as f64) * 0.018,
        _ => 0.0
    };

    let ap_reduction_penalty: f64 = ((equipment.engravings.attack_power_reduction * 2) / 100) as f64 * ap as f64;

    ap +
    cursed_doll_bonus.round() as i64 +
    adrenaline_bonus.round() as i64 -
    ap_reduction_penalty.round() as i64
}

//==========

fn calc_grudge_bonus(grudge_level: i64) -> f64 {
    if grudge_level == 3 { 0.2 }
    else if grudge_level == 2 { 0.1 }
    else if grudge_level == 1 { 0.04 }
    else { 0.0 }
}

fn calc_raid_captain_bonus(raid_captain_level: i64, move_speed: f64) -> f64 {
    if raid_captain_level == 3 { move_speed * 0.004 }
    else if raid_captain_level == 2 { move_speed * 0.0022 }
    else if raid_captain_level == 1 { move_speed * 0.001 } 
    else { 0.0 }
}

fn calc_hit_master_bonus(hit_master_level: i64) -> f64 {
    if hit_master_level == 3 { 0.2 }
    else if hit_master_level == 2 { 0.1 }
    else if hit_master_level == 1 { 0.04 }
    else { 0.0 }
}

//==========

fn calc_demon_beast_set_effect(pieces: i64) -> f64 {
    if pieces >= 6 { 0.2 }
    else if pieces >= 1 { 0.1 }
    else { 0.0 }
}

fn calc_salvation_set_effect(pieces: i64) -> f64 {
    if pieces >= 6 { 0.47 }
    else if pieces >= 4 { 0.28 }
    else if pieces >= 2 { 0.14 }
    else { 0.0 }
}

//==========

fn calc_damage_modifier(equipment: &equipment_struct, base_stats: &base_stats_struct) -> f64 {
    let grudge_bonus = calc_grudge_bonus(equipment.engravings.grudge);
    let raid_captain_bonus = calc_raid_captain_bonus(equipment.engravings.raid_captain, base_stats.move_speed);
    let hit_master_bonus = calc_hit_master_bonus(equipment.engravings.hit_master);

    let light_of_salvation_bonus: f64;
    if equipment.cards.light_of_salvation >= 30 { light_of_salvation_bonus = 0.15; }
    else if equipment.cards.light_of_salvation >= 18 { light_of_salvation_bonus = 0.07; }
    else { light_of_salvation_bonus = 0.0; }

    let demon_beast_bonus = calc_demon_beast_set_effect(equipment.sets.demon_beast);
    let salvation_bonus = calc_salvation_set_effect(equipment.sets.salvation);

    let spec_bonus = base_stats.spec_stat as f64 * 0.0008583;

    1.0 +
    grudge_bonus +
    raid_captain_bonus +
    hit_master_bonus +
    light_of_salvation_bonus +
    demon_beast_bonus +
    salvation_bonus +
    spec_bonus
}

//==========

pub fn get(base_stats: &base_stats_struct, equipment: &equipment_struct) -> derived_stats_struct{
    derived_stats_struct{
        demon_duration: calc_demon_duration(base_stats.spec_stat),
        crit_chance: calc_crit_chance(equipment, base_stats.crit_stat),
        crit_damage_multi: calc_crit_damage(equipment),
        attack_speed: calc_attack_speed(equipment, base_stats.swift_stat),
        cooldown_reduction: calc_cooldown_reduction(equipment, base_stats.swift_stat),
        attack_power: calc_modified_attack_power(equipment, base_stats.attack_power),
        damage_modifier: calc_damage_modifier(equipment, base_stats),
        move_speed: base_stats.move_speed
    }
}