use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, WeaponAttack};
use crate::sim2::types::{DamageType, CharacterAction, WeaponType, FieldEnergy, Vision, MILLENNIAL_MOVEMENT_SERIES};
use crate::sim2::element::{ElementalGauge, ElementalReactionType, ElementalReaction};
use crate::sim2::record::{CharacterData, WeaponRecord, Enemy};

use WeaponType::*;
// use Vision::*;

#[derive(Debug)]
pub struct SongOfBrokenPines {
    sigil: u8,
    time: f32,
    did_activate: Vec<f32>,
}

impl SongOfBrokenPines {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Song of Broken Pines").type_(Claymore).version(1.5)
            .base_atk(741.0)
            .atk(16.0)
            .physical_dmg(20.7)
    }

    pub fn new() -> Self {
        Self {
            sigil: 0,
            time: -99.,
            did_activate: Vec::new(),
        }
    }

    pub fn reset(&mut self) -> () {
        self.sigil = 0;
        self.time = -99.;
    }
}

impl Timeline for SongOfBrokenPines {
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        if state.current_time - self.time > 20. &&
           (event.is_na() || event.is_ca()) {
            self.sigil += 1;
            if self.sigil == 4 {
                self.sigil = 0;
                self.time = state.current_time;
                self.did_activate.push(state.current_time);
            }
        }
        if state.current_time - self.time <= 12. {
            state.atk_spd += 12.;
        }
    }
}

impl WeaponAttack for SongOfBrokenPines {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if self.did_activate.contains(&action_state.current_time) {
            self.time = action_state.current_time;
        }
        if state.stacked_buff != MILLENNIAL_MOVEMENT_SERIES && attack.time - self.time <= 12. {
            state.atk += 20.0;
            state.stacked_buff.turn_on(&MILLENNIAL_MOVEMENT_SERIES);
        }
    }

    fn reset(&mut self) -> () {
        self.sigil = 0;
        self.time = -99.;
    }
}
