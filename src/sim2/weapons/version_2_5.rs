use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, WeaponAttack};
use crate::sim2::types::{DamageType, CharacterAction, WeaponType, FieldEnergy, Vision};
use crate::sim2::element::{ElementalGauge, ElementalReactionType, ElementalReaction, PHYSICAL_GAUGE};
use crate::sim2::record::{CharacterData, WeaponRecord, Enemy};

use WeaponType::*;
// use Vision::*;

// Gains the Kagura Dance effect when using an Elemental Skill, causing the
// Elemental Skill DMG of the character wielding this weapon to increase by 12%
// for 12s. Max 3 stacks. This character will gain 12% All Elemental DMG Bonus
// when they possess 3 stacks.
#[derive(Debug)]
pub struct KagurasVerity {
    time: f32,
    stack: f32,
}

impl KagurasVerity {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Kagura's Verity").type_(Catalyst).version(2.5)
            .base_atk(608.)
            .cd(66.2)
            .skill_dmg(36.).elemental_dmg(12.)
    }

    pub fn new() -> Self {
        Self {
            time: -99.,
            stack: 0.,
        }
    }
}

impl Timeline for KagurasVerity {}

impl WeaponAttack for KagurasVerity {}
// impl WeaponAttack for KagurasVerity {
//     fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
//         if action_state.did_skill() {
//             self.time = action_state.current_time;
//             self.stack += 1.;
//         }
//         if attack.idx == data.idx {
//             if attack.time - self.time > 12. {
//                 self.stack = 0.;
//             }
//             if self.stack > 3. {
//                 self.stack = 3.;
//             }
//             state.skill_dmg += 12. * self.stack;
//             if self.stack == 3. {
//                 state.elemental_dmg += 12.;
//             }
//         }
//     }
//     fn reset_modify(&mut self) -> () {
//         self.time = -99.;
//         self.stack = 0.;
//     }
// }
