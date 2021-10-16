use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, WeaponAttack};
use crate::sim2::types::{DamageType, CharacterAction, WeaponType, FieldEnergy, Vision};
use crate::sim2::element::{ElementalGauge, ElementalReactionType, ElementalReaction, PHYSICAL_GAUGE};
use crate::sim2::record::{CharacterData, WeaponRecord, Enemy};

use WeaponType::*;
// use Vision::*;

pub struct EngulfingLightning {
    time: f32,
}

impl EngulfingLightning {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Engulfing Lightning").type_(Polearm).version(2.1)
            .base_atk(608.0)
            .er(55.1)
    }

    pub fn new() -> Self {
        Self {
            time: -99.
        }
    }
}

impl Timeline for EngulfingLightning {
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        if *event == CharacterAction::Burst {
            self.time = state.current_time;
        }
        if state.current_time - self.time <= 12. {
            state.er += 30.;
        }
    }
}

impl WeaponAttack for EngulfingLightning {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx {
            state.atk += 0.28 * action_state.er;
        }
    }
}

pub struct EverlastingMoonglow {
    time: f32,
}

impl EverlastingMoonglow {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Everlasting Moonglow").type_(Catalyst).version(2.1)
            .base_atk(608.0)
            // TODO healing bonus
            .hp(49.6)
    }

    pub fn new() -> Self {
        Self {
            time: -99.,
        }
    }
}

impl Timeline for EverlastingMoonglow {
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        if *event == CharacterAction::Burst {
            self.time = state.current_time;
        }
        if state.current_time - self.time <= 12. && state.did_na() {
            state.energy += 0.6;
        }
    }

    fn reset(&mut self) -> () {
        self.time = -99.;
    }
}

impl WeaponAttack for EverlastingMoonglow {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx && attack.kind == DamageType::Na {
            state.flat_atk += 0.01 * state.HP();
        }
    }
}

pub struct LuxuriousSeaLord {
}

impl LuxuriousSeaLord {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Luxurious Sea-Lord").type_(Claymore).version(2.1)
            .base_atk(454.0)
            .atk(55.1).burst_dmg(24.0)
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl WeaponAttack for LuxuriousSeaLord {
    fn attack(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        if event.is_burst() {
            atk_queue.push(Attack {
                kind: DamageType::AdditionalAttack,
                multiplier: 200.,
                element: &PHYSICAL_GAUGE,
                aura_application: false,
                time,
                idx: data.idx,
            });
        }
    }
}

pub struct TheCatch {
}

impl TheCatch {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("The Catch").type_(Polearm).version(2.1)
            .base_atk(510.0)
            .er(45.9).burst_dmg(32.0)
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl WeaponAttack for TheCatch {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx && attack.kind == DamageType::Burst {
            state.cr += 12.;
        }
    }
}
