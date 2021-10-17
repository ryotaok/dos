use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, WeaponAttack};
use crate::sim2::types::{DamageType, CharacterAction, WeaponType, FieldEnergy, Vision};
use crate::sim2::element::{ElementalGauge, ElementalReactionType, ElementalReaction, PHYSICAL_GAUGE};
use crate::sim2::record::{CharacterData, WeaponRecord, Enemy};
use crate::sim2::testutil;

use WeaponType::*;
// use Vision::*;

// version 1.0

pub struct PrototypeAmberR5;

impl Timeline for PrototypeAmberR5 {}

impl WeaponAttack for PrototypeAmberR5 {}

impl PrototypeAmberR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Prototype Amber").type_(Catalyst).version(1.0)
            .base_atk(510.0)
            .hp(41.3)
    }
}

pub struct MappaMareR5 {
    stack: f32,
    time: f32,
}

impl MappaMareR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Mappa Mare").type_(Catalyst).version(1.0)
            .base_atk(565.0)
            .em(110.0)
    }

    pub fn new() -> Self {
        Self {
            stack: 0.,
            time: -99.,
        }
    }
}

impl Timeline for MappaMareR5 {}

impl WeaponAttack for MappaMareR5 {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx &&
           enemy.trigger_er(&attack.element.aura).is_triggered() {
            self.stack += 1.;
            if self.stack > 2. {
                self.stack = 2.;
            }
        }
        if attack.idx == data.idx && attack.time - self.time <= 10. {
            state.elemental_dmg += 16. * self.stack;
        }
    }

    fn reset(&mut self) -> () {
        self.stack = 0.;
        self.time = -99.;
    }
}

pub struct SolarPearlR5 {
    na_time: f32,
    skill_time: f32,
}

impl SolarPearlR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Solar Pearl").type_(Catalyst).version(1.0)
            .base_atk(510.0)
            .cr(27.6)
    }

    pub fn new() -> Self {
        Self {
            na_time: -99.,
            skill_time: -99.,
        }
    }
}

impl Timeline for SolarPearlR5 {}

impl WeaponAttack for SolarPearlR5 {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        let oneself = attack.idx == data.idx;
        if action_state.did_na() {
            self.na_time = action_state.current_time;
        }
        if action_state.did_skill() || action_state.did_burst() {
            self.skill_time = action_state.current_time;
        }
        if oneself && attack.time - self.na_time <= 6. {
            state.skill_dmg += 40.;
            state.burst_dmg += 40.;
        }
        if oneself && attack.time - self.skill_time <= 6. {
            state.na_dmg += 40.;
        }
    }

    fn reset(&mut self) -> () {
        self.na_time = -99.;
        self.skill_time = -99.;
    }
}

// one stack is always active
pub struct BlackcliffAgateR5;

impl Timeline for BlackcliffAgateR5 {}

impl WeaponAttack for BlackcliffAgateR5 {}

impl BlackcliffAgateR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Blackcliff Agate").type_(Catalyst).version(1.0)
            .base_atk(510.0)
            .atk(24.0).cd(55.1)
    }
}

pub struct RoyalGrimoireR5;

impl Timeline for RoyalGrimoireR5 {}

impl WeaponAttack for RoyalGrimoireR5 {}

impl RoyalGrimoireR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Royal Grimoire").type_(Catalyst).version(1.0)
            .base_atk(565.0)
            .atk(27.6)
    }
}

pub struct ThrillingTalesOfDragonSlayersR5 {
    time: f32,
}

impl ThrillingTalesOfDragonSlayersR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Thrilling Tales of Dragon Slayers").type_(Catalyst).version(1.0)
            .base_atk(401.0)
            .hp(35.2)
    }

    pub fn new() -> Self {
        Self {
            time: -99.,
        }
    }
}

impl Timeline for ThrillingTalesOfDragonSlayersR5 {}

impl WeaponAttack for ThrillingTalesOfDragonSlayersR5 {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.current_time - self.time >= 20. {
            self.time = action_state.current_time;
        }
        // always buff the first member
        if attack.idx.0 == 0 && attack.time - self.time <= 10. {
            state.atk += 48.0;
        }
    }

    fn reset(&mut self) -> () {
        self.time = -99.;
    }
}

pub struct EyeOfPerceptionR5 {
    time: f32,
}

impl EyeOfPerceptionR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Eye of Perception").type_(Catalyst).version(1.0)
            .base_atk(454.0)
            .atk(55.1)
    }

    pub fn new() -> Self {
        Self {
            time: -99.
        }
    }
}

impl Timeline for EyeOfPerceptionR5 {}

impl WeaponAttack for EyeOfPerceptionR5 {
    fn attack(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        if time - self.time > 8. &&
           testutil::chance() < 0.5 &&
           (event.is_na() || event.is_ca()) {
            self.time = time;
            atk_queue.push(Attack {
                kind: DamageType::AdditionalAttack,
                multiplier: 360.,
                element: &PHYSICAL_GAUGE,
                aura_application: false,
                time,
                idx: data.idx,
            });
        }
    }

    fn reset(&mut self) -> () {
        self.time = -99.;
    }
}

pub struct TheWidsithR5 {
    random_theme_song: u8,
    time: f32
}

impl TheWidsithR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("The Widsith").type_(Catalyst).version(1.0)
            .base_atk(510.0)
            .cd(55.1)
    }

    pub fn new() -> Self {
        Self {
            random_theme_song: 0,
            time: -99.
        }
    }
}

impl Timeline for TheWidsithR5 {}

impl WeaponAttack for TheWidsithR5 {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.current_time - self.time >= 30. {
            self.time = action_state.current_time;
            let p = testutil::chance();
            self.random_theme_song = if p > 0.6666 {
                0
            } else if p > 0.3333 {
                1
            } else {
                2
            };
        }
        if attack.idx == data.idx && attack.time - self.time <= 10. {
            match self.random_theme_song {
                0 => state.atk += 120.0,
                1 => state.all_dmg += 96.0,
                2 => state.em += 480.0,
                _ => (),
            }
        }
    }

    fn reset(&mut self) -> () {
        self.time = -99.;
    }
}
