use crate::simulation::run_simulation::structs;

pub fn time_passed(timers: &mut structs::TimerManager, update: f64) -> &mut structs::TimerManager {
    timers.cooldowns.update_timers(update);
    timers.rune_timers.update_timers(update);
    timers.hallucination.update_timers(update);

    timers.demon_duration += update;
    timers.damage_boost += update;
    if timers.demon_duration < 0.0 { timers.demon_duration = 0.0 };
    if timers.damage_boost < 0.0 { timers.damage_boost = 0.0 };
    
    timers
}
