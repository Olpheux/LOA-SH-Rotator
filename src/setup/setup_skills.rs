mod baseline_skills;
mod modify_skills;
use crate::character::other_structs::{SkillList, DerivedStats, RuneLevels};

pub fn skill_setup(stats: &DerivedStats, runes: Vec<RuneLevels>) -> SkillList {
    let base_skill_list: SkillList = baseline_skills::baseline_skills(stats.attack_power as f64, &runes);
    modify_skills::modify_skills(stats, &base_skill_list)
}