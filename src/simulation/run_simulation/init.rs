use crate::simulation::run_simulation::structs;
use crate::simulation::Skill;
use crate::character::meta_structs::Character as Character;

pub fn setup_cooldowns() -> structs::Cooldowns {
    structs::Cooldowns{
        ruining_rush_cd: 0.0,
        death_claw_cd: 0.0,
        destruction_cd: 0.0,
        gore_bleeding_cd: 0.0,
        leaping_blow_cd: 0.0,
        blood_massacre_cd: 0.0
    }
}

pub fn setup_rune_timers() -> structs::RuneTimers {
    structs::RuneTimers { 
        bleed: 0.0,
        bleed_tick: 0.0,
        rage: 0.0, 
        rage_level: 0,
        conviction: 0.0,
        judgement: 0.0,
        conviction_icd: 0.0 
    }
}

pub fn setup_hallucination() -> structs::HallucinationTimers {
    structs::HallucinationTimers {
        remaining: 0.0, 
        cooldown: 0.0,
        duration: 0.0
    }
}


//=====


pub fn setup_timers(character: &Character) -> structs::TimerManager {
    structs::TimerManager{
        cooldowns: setup_cooldowns(),
        rune_timers: setup_rune_timers(),
        hallucination: setup_hallucination(),
        damage_boost: 0.0,
        demon_duration: character.stats.demon_duration
    }
}


//=====


pub fn create_skill_list(character: Character) -> [Skill; 6] {
    [character.skills.ruining_rush,
     character.skills.death_claw,
     character.skills.destruction,
     character.skills.gore_bleeding,
     character.skills.leaping_blow,
     character.skills.blood_massacre]
}