use crate::character::char_structs::other_structs::Skill;
use crate::character::char_structs::other_structs::SkillList;

pub fn baseline_skills(attack_power: f64) -> SkillList {
    SkillList{
        ruining_rush: Skill{
            name: "Ruining Rush".to_string(),
            cooldown: 6.0,
            cast_time: 1.7,
            result_damage: (attack_power * 12.4468599) + 1985.0,
            keybind: 'Q'.to_string(),
            rune: "None".to_string(),
            rune_level: 0,
            hits: 6,
            id: 0
        },

        death_claw: Skill{
            name: "Death Claw".to_string(),
            cooldown: 4.0,
            cast_time: 1.7,
            result_damage: (attack_power * 17.19504831) + 2452.0,
            keybind: 'W'.to_string(),
            rune: "None".to_string(),
            rune_level: 0,
            hits: 6,
            id: 1
        },

        destruction: Skill{
            name: "Destruction".to_string(),
            cooldown: 8.0,
            cast_time: 1.15,
            result_damage: (attack_power * 26.87137681) + 4306.0,
            keybind: 'E'.to_string(),
            rune: "None".to_string(),
            rune_level: 0,
            hits: 3,
            id: 2
        },

        gore_bleeding: Skill{
            name: "Gore Bleeding".to_string(),
            cooldown: 6.0,
            cast_time: 2.15,
            result_damage: (attack_power * 33.30495169) + 5305.0,
            keybind: 'R'.to_string(),
            rune: "None".to_string(),
            rune_level: 0,
            hits: 9,
            id: 3
        },

        leaping_blow: Skill{
            name: "Leaping Blow".to_string(),
            cooldown: 18.0,
            cast_time: 2.05,
            result_damage: (attack_power * 41.75603865) + 6701.0,
            keybind: 'A'.to_string(),
            rune: "None".to_string(),
            rune_level: 0,
            hits: 3,
            id: 4
        },

        blood_massacre: Skill{
            name: "Blood Massacre".to_string(),
            cooldown: 20.0,
            cast_time: 1.95,
            result_damage: (attack_power * 55.13888888) + 8838.0,
            keybind: 'S'.to_string(),
            rune: "None".to_string(),
            rune_level: 0,
            hits: 1,
            id: 5
        }
    }
}