#[derive(Debug,Clone,Default)]
pub struct OtherGear{
    pub cooldown_gem: i64,
    pub attack_power_gem: i64,
    pub weapon_damage: f64
}

#[derive(Debug,Clone,Default)]
pub struct Cards {
    pub lostwind_cliff: i64,
    pub light_of_salvation: i64
}

#[derive(Debug,Clone,Default)]
pub struct Gearset {
    pub preordained: i64,
    pub demon_beast: i64,
    pub salvation: i64,
    pub hallucination: i64
}

#[derive(Debug,Clone,Default)]
pub struct Engravings {
    pub demonic_impulse: i64,
    pub grudge: i64,
    pub cursed_doll: i64,
    pub raid_captain: i64,
    pub spirit_absorption: i64,
    pub adrenaline: i64,
    pub hit_master: i64,
    pub keen_blunt: i64,
    pub attack_power_reduction: i64,
    pub attack_speed_reduction: i64
}