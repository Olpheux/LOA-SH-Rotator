use crate::character::char_structs::other_structs::Skill;
use crate::character::char_structs::other_structs::SkillList;
use crate::character::char_structs::other_structs::DerivedStats;

pub fn modify_skills(stats: &DerivedStats, base_skill_list: &SkillList) -> SkillList {
    SkillList{
        ruining_rush: modify_skill(stats, &base_skill_list.ruining_rush),
        death_claw: modify_skill(stats, &base_skill_list.death_claw),
        destruction: modify_skill(stats, &base_skill_list.destruction),
        gore_bleeding: modify_skill(stats, &base_skill_list.gore_bleeding),
        leaping_blow: modify_skill(stats, &base_skill_list.leaping_blow),
        blood_massacre: modify_skill(stats, &base_skill_list.blood_massacre),
    }
}

//==========

fn modify_skill(stats: &DerivedStats, baseline_skill: &Skill) -> Skill {
    Skill{
        cast_time: calc_cast_time(stats, baseline_skill),
        cooldown: calc_cooldown(stats, baseline_skill),
        hits: baseline_skill.hits.clone(),
        keybind: baseline_skill.keybind.clone(),
        name: baseline_skill.name.clone(),
        result_damage: calc_damage(stats, baseline_skill),
        rune: baseline_skill.rune.clone(),
        rune_level: baseline_skill.rune_level.clone(),
        id: baseline_skill.id.clone()
    }
}

//==========

fn calc_cooldown(stats: &DerivedStats, skill: &Skill) -> f64 {
    skill.cooldown - (skill.cooldown * stats.cooldown_reduction)
}

fn calc_cast_time(stats: &DerivedStats, skill: &Skill) -> f64{
    skill.cast_time - (skill.cast_time * stats.attack_speed)
}

fn calc_damage(stats: &DerivedStats, skill: &Skill) -> f64{
    skill.result_damage * stats.damage_modifier
}
