#[derive(Debug,Clone,Default)]
pub struct BuffTimers {
    pub demon_time_remaining: f64,
    pub damage_boost: f64,
    pub hallucination_timer: f64,
    pub hallucination_duration: f64,
    pub reality_timer: f64
}

#[derive(Debug,Clone,Default)]
pub struct DerivedStats {
    pub demon_duration: f64,
    pub crit_chance: f64,
    pub crit_damage_multi: f64,
    pub attack_speed: f64,
    pub cooldown_reduction: f64,
    pub attack_power: i64,
    pub damage_modifier: f64
}

#[derive(Debug,Clone,PartialEq,Default)]
pub struct Skill {
    pub name: String,
    pub cooldown: f64,
    pub cast_time: f64,
    pub result_damage: f64,
    pub keybind: String,
    pub rune: String,
    pub rune_level: i64,
    pub hits: i64,
    pub id: i64
}

#[derive(Debug,Clone,PartialEq,Default)]
pub struct SkillList {
    pub ruining_rush: Skill,
    pub death_claw: Skill,
    pub destruction: Skill,
    pub gore_bleeding: Skill,
    pub leaping_blow: Skill,
    pub blood_massacre: Skill
}