#[derive(Debug,Clone,Default)]
pub struct Cooldowns {
    pub ruining_rush_cd: f64,
    pub death_claw_cd: f64,
    pub destruction_cd: f64,
    pub gore_bleeding_cd: f64,
    pub leaping_blow_cd: f64,
    pub blood_massacre_cd: f64
}

impl Cooldowns {
    pub fn to_array(&self) -> [f64; 6] {
        [self.ruining_rush_cd,
         self.death_claw_cd,
         self.destruction_cd,
         self.gore_bleeding_cd,
         self.leaping_blow_cd,
         self.blood_massacre_cd]
    }

   pub fn replace_with_array(&mut self, cooldowns: [f64; 6]) {
        self.ruining_rush_cd = cooldowns[0];
        self.death_claw_cd = cooldowns[1];
        self.destruction_cd = cooldowns[2];
        self.gore_bleeding_cd = cooldowns[3];
        self.leaping_blow_cd = cooldowns[4];
        self.blood_massacre_cd = cooldowns[5];
    }

    pub fn update_timers(&mut self, update: f64) {
        let mut cd_array = self.to_array();
    
        for x in 0..6 { 
            cd_array[x] += update;
            if cd_array[x] <= 0.0 { cd_array[x] = 0.0; }
        }
    
        self.replace_with_array(cd_array);
    }
}

//=====

#[derive(Debug,Clone,Default)]
pub struct TimerManager {
    pub cooldowns: Cooldowns,
    pub rune_timers: RuneTimers,
    pub hallucination: HallucinationTimers,
    pub damage_boost: f64,
    pub demon_duration: f64
}

//=====

#[derive(Debug,Clone,Default)]
pub struct RuneTimers {
    pub bleed: f64,
    pub bleed_tick: f64,
    pub rage: f64,
    pub rage_level: i64,
    pub conviction: f64,
    pub judgement: f64,
    pub conviction_icd: f64
}

impl RuneTimers {
    pub fn update_timers(&mut self, update: f64) {
        self.bleed += update;
        self.rage += update;
        self.conviction += update;
        self.judgement += update;
        self.conviction_icd += update;

        self.floor_timers();
    }

    fn floor_timers(&mut self){
        if self.bleed < 0.0 { self.bleed = 0.0; }
        if self.rage < 0.0 { self.rage = 0.0; }
        if self.conviction < 0.0 { self.conviction = 0.0; }
        if self.judgement < 0.0 { self.judgement = 0.0; }
        if self.conviction_icd < 0.0 { self.conviction_icd = 0.0; }
    }
}

#[derive(Debug,Clone,Default)]
pub struct HallucinationTimers {
    pub remaining: f64,
    pub duration: f64,
    pub cooldown: f64,
}

impl HallucinationTimers {
    pub fn update_timers(&mut self, update: f64){
        // After the Hallucination set update in the Brelshaza patch,
        // maintaining Reality is basically free. If we have the 6-piece,
        // there's no point in tracking its duration.
        self.remaining += update;
        self.cooldown += update;
        self.duration -= update;

        self.floor_timers();
    }

    pub fn crit(&mut self){
        if self.remaining == 0.0 { self.remaining = 6.0; }
        else { self.remaining += 1.0; }
    }

    fn floor_timers(&mut self){
        if self.cooldown < 0.0 { self.cooldown = 0.0; }

        if self.remaining < 0.0 { 
            self.remaining = 0.0;
            // Secondary check to avoid repeatedly resetting cooldown
            if self.cooldown == 0.0 { self.cooldown = 3.0; }
        }
        else if self.remaining > 5.0 { self.remaining = 5.0; }
    }
}