use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, WeaponAttack};
use crate::sim2::types::{DamageType, CharacterAction, WeaponType, FieldEnergy, Vision};
use crate::sim2::element::{ElementalGauge, ElementalReactionType, ElementalReaction, PHYSICAL_GAUGE};
use crate::sim2::record::{CharacterData, WeaponRecord, Enemy};

use WeaponType::*;
// use Vision::*;

// DEF is increased by 20%. Normal and Charged Attack DMG is increased by 28% of
// DEF.
#[derive(Debug)]
pub struct RedhornStonethresher {}

impl RedhornStonethresher {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Redhorn Stonethresher").type_(Claymore).version(2.3)
            .base_atk(542.0)
            .cd(88.2)
            // .base_atk(608.0)
            // .cd(66.2)
            .def(20.)
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Timeline for RedhornStonethresher {}

impl WeaponAttack for RedhornStonethresher {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx {
            if attack.kind == DamageType::Na || attack.kind == DamageType::Ca {
                let bonus = 0.36 * state.DEF();
                state.flat_dmg += bonus;
            }
            // let bonus = 0.28 * state.DEF();
            // state.na_dmg += bonus;
            // state.ca_dmg += bonus;
        }
    }
}

// Elemental Skill DMG is increased by 80% of DEF. The effect will be triggered
// no more than once every 1.5s and will be cleared 0.1s after the Elemental
// Skill deals DMG.
#[derive(Debug)]
pub struct CinnabarSpindle {
    skill_time: f32,
}

impl CinnabarSpindle {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Cinnabar Spindle").type_(Sword).version(2.3)
            .base_atk(454.)
            .def(69.)
    }

    pub fn new() -> Self {
        Self {
            skill_time: -99.,
        }
    }
}

impl Timeline for CinnabarSpindle {}

impl WeaponAttack for CinnabarSpindle {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx && attack.kind == DamageType::Skill && attack.time - self.skill_time >= 1.5 {
            let bonus = 0.8 * state.DEF();
            state.flat_dmg += bonus;
            self.skill_time = attack.time;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.skill_time = -99.;
    }
}
