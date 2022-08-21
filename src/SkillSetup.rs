#[derive(Debug,Clone,PartialEq)]
pub struct Skill {
    name: String,
    cooldown: f64,
    cast_time: f64,
    result_damage: f64,
    keybind: String
}

// The 'magic numbers' in result_damage are hardcoded into the game.
// These can't change - skill damage is calculated as ((AP * X)+Y),
// where every skill has its own value for X and Y.
pub fn skill_setup(attack_power: f64) -> Vec<Skill> {
    let ruining_rush = Skill{
        name: "Ruining Rush".to_string(),
        cooldown: 6.0,
        cast_time: 1.7,
        result_damage: (attack_power * 12.4468599) + 1985.0,
        keybind: 'Q'.to_string()
    };

    let death_claw = Skill{
        name: "Death Claw".to_string(),
        cooldown: 4.0,
        cast_time: 1.7,
        result_damage: (attack_power * 17.19504831) + 2452.0,
        keybind: 'W'.to_string()
    };

    let destruction = Skill{
        name: "Destruction".to_string(),
        cooldown: 8.0,
        cast_time: 1.15,
        result_damage: (attack_power * 26.87137681) + 4306.0,
        keybind: 'E'.to_string()
    };

    let gore_bleeding = Skill{
        name: "Gore Bleeding".to_string(),
        cooldown: 6.0,
        cast_time: 2.15,
        result_damage: (attack_power * 33.30495169) + 5305.0,
        keybind: 'R'.to_string()
    };

    let leaping_blow = Skill{
        name: "Leaping Blow".to_string(),
        cooldown: 18.0,
        cast_time: 2.05,
        result_damage: (attack_power * 41.75603865) + 6701.0,
        keybind: 'E'.to_string()
    };

    let blood_massacre = Skill{
        name: "Blood Massacre".to_string(),
        cooldown: 20.0,
        cast_time: 1.95,
        result_damage: (attack_power * 55.13888888) + 8838.0,
        keybind: 'S'.to_string()
    };

    return vec![ruining_rush,death_claw,destruction,gore_bleeding,leaping_blow,blood_massacre];
}

pub fn calc_modified_skills(
        cd_gem: i32,
        ap_gem: i32,
        swift: i32,
        spec: i32,
        extra_weapon_damage: f64,
        attack_speed: f64,
        attack_power: f64,
        damage_modifiers: f64
    ) -> Vec<Skill> {
    let mut skills = skill_setup(attack_power);

    for skill in skills.iter_mut(){
        skill.cooldown -= skill.cooldown * (((swift as f64) * 0.00021472) + ((cd_gem as f64) * 0.02));
        skill.cast_time -= skill.cast_time * (attack_speed / 100.0);
        skill.result_damage *= 1.0 + 
            (((spec as f64) * 0.08583) / 100.0) +
              (damage_modifiers) +
              ((ap_gem as f64) / 100.0) +
              (extra_weapon_damage / 100.0)
              // TODO: Add runes
              // TODO: Add set bonuses
              // TODO: Add card sets
            ;
    }

    return skills;
}