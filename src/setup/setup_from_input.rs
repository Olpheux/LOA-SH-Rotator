pub mod get_base_stats;
mod get_equipment;
mod derive_stats;
use crate::setup::setup_skills;
use crate::character::char_structs::meta_structs::Character as Character;

pub fn setup_from_input() -> Character {
    let base_stats = get_base_stats::get();
    let equipment = get_equipment::get();
    let buff_timers = crate::character::char_structs::other_structs::BuffTimers{
                            demon_time_remaining: 0.0,
                            damage_boost: 0.0,
                            hallucination_timer: 0.0,
                            hallucination_duration: 0.0,
                            reality_timer: 0.0
                        };

    let derived_stats = derive_stats::get(&base_stats, &equipment);
    let skills = setup_skills::skill_setup(&derived_stats);

    Character{
        stats: derived_stats,
        equipment,
        buffs: buff_timers,
        skills
    }
}