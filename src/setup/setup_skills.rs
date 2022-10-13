mod baseline_skills;
mod modify_skills;
use crate::character::char_structs::other_structs::SkillList;
use crate::character::char_structs::other_structs::DerivedStats;

pub fn skill_setup(stats: &DerivedStats) -> SkillList {
    let base_skill_list: SkillList = baseline_skills::baseline_skills(stats.attack_power as f64);
    
    modify_skills::modify_skills(stats, &base_skill_list)
}