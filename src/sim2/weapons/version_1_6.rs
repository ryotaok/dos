use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, WeaponAttack};
use crate::sim2::types::{DamageType, CharacterAction, WeaponType, FieldEnergy, Vision, MILLENNIAL_MOVEMENT_SERIES};
use crate::sim2::element::{ElementalGauge, ElementalReactionType, ElementalReaction};
use crate::sim2::record::{CharacterData, WeaponRecord, Enemy};

use WeaponType::*;
// use Vision::*;

pub struct FreedomSworn {
    sigil: u8,
    time: f32,
}

impl FreedomSworn {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Freedom-Sworn").type_(Sword).version(1.6)
            .base_atk(608.0)
            .em(198.0).na_dmg(10.0).ca_dmg(10.0).skill_dmg(10.0).burst_dmg(10.0)
    }

    pub fn new() -> Self {
        Self {
            sigil: 0,
            time: -99.,
        }
    }
}

impl Timeline for FreedomSworn {}

impl WeaponAttack for FreedomSworn {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx &&
           enemy.trigger_er(&attack.element.aura).is_triggered() &&
           attack.time - self.time > 20. {
            self.sigil += 1;
            if self.sigil == 2 {
                self.time = attack.time;
                self.sigil = 0;
            }
        }
        if state.stacked_buff != MILLENNIAL_MOVEMENT_SERIES && attack.time - self.time <= 12. {
            state.atk += 20.0;
            state.na_dmg += 16.0;
            state.ca_dmg += 16.0;
            state.stacked_buff.turn_on(&MILLENNIAL_MOVEMENT_SERIES);
        }
    }

    fn reset_modify(&mut self) -> () {
        self.sigil = 0;
        self.time = -99.;
    }
}

pub struct MitternachtsWaltz {
    na_time: f32,
    skill_time: f32,
}

impl MitternachtsWaltz {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Mitternachts Waltz").type_(Bow).version(1.6)
            .base_atk(510.0)
            .physical_dmg(51.7)
    }

    pub fn new() -> Self {
        Self {
            na_time: -99.,
            skill_time: -99.,
        }
    }
}

impl Timeline for MitternachtsWaltz {}

impl WeaponAttack for MitternachtsWaltz {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        let oneself = attack.idx == data.idx;
        if action_state.did_na() {
            self.na_time = action_state.current_time;
        }
        if action_state.did_skill() {
            self.skill_time = action_state.current_time;
        }
        if oneself && attack.time - self.na_time <= 5. {
            state.skill_dmg += 40.;
        }
        if oneself && attack.time - self.skill_time <= 5. {
            state.na_dmg += 40.;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.na_time = -99.;
        self.skill_time = -99.;
    }
}

pub struct DodocoTales {
    na_time: f32,
    ca_time: f32,
}

impl DodocoTales {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Dodoco Tales").type_(Catalyst).version(1.6)
            .base_atk(454.0)
            .atk(55.1)
    }

    pub fn new() -> Self {
        Self {
            na_time: -99.,
            ca_time: -99.,
        }
    }
}

impl Timeline for DodocoTales {}

impl WeaponAttack for DodocoTales {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        let oneself = attack.idx == data.idx;
        if action_state.did_na() {
            self.na_time = action_state.current_time;
        }
        if action_state.did_ca() {
            self.ca_time = action_state.current_time;
        }
        if oneself && attack.time - self.na_time <= 6. {
            state.ca_dmg += 32.;
        }
        if oneself && attack.time - self.ca_time <= 6. {
            state.atk += 16.;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.na_time = -99.;
        self.ca_time = -99.;
    }
}
