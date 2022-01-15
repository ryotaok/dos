use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, WeaponAttack};
use crate::sim2::types::{DamageType, CharacterAction, WeaponType, FieldEnergy, Vision};
use crate::sim2::element::{ElementalGauge, ElementalReactionType, ElementalReaction, PHYSICAL_GAUGE};
use crate::sim2::record::{CharacterData, WeaponRecord, Enemy};

use WeaponType::*;
// use Vision::*;

// Gain 12% All Elemental DMG Bonus. Obtain Consummation for 20s after using an
// Elemental Skill, causing ATK to increase by 3.2% per second. This ATK
// increase has a maximum of 6 stacks. When the character equipped with this
// weapon is not on the field, Consummation's ATK increase is doubled.
#[derive(Debug)]
pub struct CalamityQueller {
    time: f32,
    stack: f32,
}

impl CalamityQueller {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Calamity Queller").type_(Polearm).version(2.4)
            .base_atk(741.)
            .atk(16.5)
            .pyro_dmg(12.).cryo_dmg(12.).hydro_dmg(12.).electro_dmg(12.).anemo_dmg(12.).geo_dmg(12.).dendro_dmg(12.)
    }

    pub fn new() -> Self {
        Self {
            time: -99.,
            stack: 0.,
        }
    }
}

impl Timeline for CalamityQueller {}

impl WeaponAttack for CalamityQueller {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_skill() {
            self.time = attack.time;
            self.stack = 0.;
        }
        if attack.idx == data.idx {
            let dr = attack.time - self.time;
            if dr > self.stack {
                self.stack += 1.;
            }
            if dr > 20. {
                self.stack = 0.;
            }
            if self.stack > 6. {
                self.stack = 6.;
            }
            if attack.idx.0 == 0 {
                state.atk += 3.2 * self.stack;
            } else {
                state.atk += 6.4 * self.stack;
            }
        }
    }

    fn reset_modify(&mut self) -> () {
        self.time = -99.;
        self.stack = 0.;
    }
}
