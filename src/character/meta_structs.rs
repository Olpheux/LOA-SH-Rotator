use crate::character::equipment_structs as equipment;
use crate::character::other_structs as other;

#[derive(Debug,Clone,Default)]
pub struct EquipmentBonuses {
    pub other_gear: equipment::OtherGear,
    pub cards: equipment::Cards,
    pub sets: equipment::Gearset,
    pub engravings: equipment::Engravings
}

#[derive(Debug,Clone,Default)]
pub struct Character {
    pub stats: other::DerivedStats,
    pub equipment: self::EquipmentBonuses,
    pub buffs: other::BuffTimers,
    pub skills: other::SkillList
}