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

// Charged Attack hits on weak points increase Movement SPD by 10% and ATK by
// 36~72% for 10s.
pub struct PrototypeCrescentR5;

impl Timeline for PrototypeCrescentR5 {}

impl WeaponAttack for PrototypeCrescentR5 {}

impl PrototypeCrescentR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Prototype Crescent").type_(Bow).version(1.0)
            .base_atk(510.0)
            .atk(41.3 + 72.0)
    }
}

// Normal Attack and Charged Attack hits increase ATK by 4~8% and Normal ATK SPD
// by 1.2~2.4% for 6s. Max 4 stacks. Can only occur once every 0.3s.
pub struct CompoundBowR5 {
    stack: f32,
    time: f32,
}

impl CompoundBowR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Compound Bow").type_(Bow).version(1.0)
            .base_atk(454.0)
            .physical_dmg(69.0)
    }

    pub fn new() -> Self {
        Self {
            stack: 0.,
            time: -99.,
        }
    }
}

impl Timeline for CompoundBowR5 {
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        if event.is_na() || event.is_ca() {
            self.time = state.current_time;
            self.stack += 1.;
            if self.stack > 4. {
                self.stack = 4.;
            }
        }
        if state.current_time - self.time <= 6. {
            state.atk_spd += 2.4 * self.stack;
        }
    }

    fn reset_timeline(&mut self) -> () {
        self.stack = 0.;
        self.time = -99.;
    }
}

impl WeaponAttack for CompoundBowR5 {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_na() || action_state.did_ca() {
            self.time = action_state.current_time;
            self.stack += 1.;
            if self.stack > 4. {
                self.stack = 4.;
            }
        }
        if attack.idx == data.idx && attack.time - self.time <= 6. {
            state.atk += 8. * self.stack;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.stack = 0.;
        self.time = -99.;
    }
}

// Upon hit, Normal and Aimed Shot Attacks have a 50% chance to generate a
// Cyclone, which will continuously attract surrounding opponents, dealing
// 40~80% of ATK as DMG to these opponents every 0.5s for 4s. This effect can
// only occur once every 14~10s.
pub struct TheViridescentHuntR5 {
    time: f32,
    last_attack: f32,
    stack: u8,
}

impl TheViridescentHuntR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("The Viridescent Hunt").type_(Bow).version(1.0)
            .base_atk(510.0)
            .cr(27.6)
    }

    pub fn new() -> Self {
        Self {
            time: -99.,
            last_attack: -99.,
            stack: 0,
        }
    }
}

impl Timeline for TheViridescentHuntR5 {}

impl WeaponAttack for TheViridescentHuntR5 {
    fn attack(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        if time - self.time > 10. &&
           testutil::chance() < 0.5 &&
           (event.is_na() || event.is_ca()) {
            self.time = time;
            self.stack = 0;
        }
        if self.stack <= 8 &&
           time - self.last_attack >= 0.5 {
            self.stack += 1;
            self.last_attack = time;
            atk_queue.push(Attack {
                kind: DamageType::AdditionalAttack,
                multiplier: 80.,
                element: &PHYSICAL_GAUGE,
                aura_application: false,
                time,
                idx: data.idx,
            });
        }
    }

    fn reset_attack(&mut self) -> () {
        self.time = -99.;
        self.last_attack = -99.;
        self.stack = 0;
    }
}

// one stack is always active
// After defeating an opponent, ATK is increased by 12~24% for 30s. This effect
// has a maximum of 3 stacks, and the duration of each stack is independent of
// the others.
pub struct BlackcliffWarbowR5;

impl Timeline for BlackcliffWarbowR5 {}

impl WeaponAttack for BlackcliffWarbowR5 {}

impl BlackcliffWarbowR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Blackcliff Warbow").type_(Bow).version(1.0)
            .base_atk(510.0)
            .atk(24.0).cd(55.1)
    }
}

pub struct RoyalBowR5;

impl Timeline for RoyalBowR5 {}

impl WeaponAttack for RoyalBowR5 {}

impl RoyalBowR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Royal Bow").type_(Bow).version(1.0)
            .base_atk(510.0)
            .atk(41.3)
    }
}

// If a Normal or Charged Attack hits a target within 0.3s of being fired,
// increases DMG by 36~60%. Otherwise, decreases DMG by 10%.
pub struct SlingshotR5;

impl Timeline for SlingshotR5 {}

impl WeaponAttack for SlingshotR5 {}

impl SlingshotR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Slingshot").type_(Bow).version(1.0)
            .base_atk(354.0)
            .cr(31.2)
            .na_dmg(60.0).ca_dmg(60.0)
    }
}

// Increases Normal Attack DMG by 80% but decreases Charged Attack DMG by
// 10%.
pub struct RustR5;

impl Timeline for RustR5 {}

impl WeaponAttack for RustR5 {}

impl RustR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Rust").type_(Bow).version(1.0)
            .base_atk(510.0)
            .atk(41.3)
            .na_dmg(80.0).ca_dmg(-10.0)
    }
}

// Increases Elemental Skill and Elemental Burst DMG by 48%.
pub struct TheStringlessR5;

impl Timeline for TheStringlessR5 {}

impl WeaponAttack for TheStringlessR5 {}

impl TheStringlessR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("The Stringless").type_(Bow).version(1.0)
            .base_atk(510.0)
            .em(165.0)
            .skill_dmg(48.0).burst_dmg(48.0)
    }
}
