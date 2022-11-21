mod get_base_stats;
mod get_equipment;
mod derive_stats;
pub mod get_runes;
use crate::setup::setup_skills;
use crate::character::char_structs::meta_structs::Character as Character;

pub fn setup_from_file(file: &str) -> Character{
    let base_stats = get_base_stats::get(file);
    let runes = get_runes::get(file);
    let equipment = get_equipment::get(file);
    let buffs = crate::character::char_structs::other_structs::BuffTimers{
                            demon_time_remaining: 0.0,
                            damage_boost: 0.0,
                            hallucination_timer: 0.0,
                            hallucination_duration: 0.0,
                            reality_timer: 0.0
                        };

    let stats = derive_stats::get(&base_stats, &equipment);
    let skills = setup_skills::skill_setup(&stats, runes);

    Character {
        stats,
        equipment,
        buffs,
        skills
    }
}