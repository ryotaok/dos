use std::rc::Rc;
use crate::state::State;
use crate::types::{AttackType, WeaponType, FieldEnergy, PHYSICAL_GAUGE};
use crate::fc::{FieldCharacterIndex, SpecialAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{Attack, AttackEvent, ICDTimer, ICDTimers, NTimer, DurationTimer};

use AttackType::*;
use WeaponType::*;
// use Vision::*;

pub struct EngulfingLightning {
    idx: FieldCharacterIndex,
    timer: DurationTimer,
}

impl EngulfingLightning {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx,
            timer: DurationTimer::new(12.0, &[0.0]),
        }
    }
}

impl EngulfingLightning {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Engulfing Lightning").type_(Polearm).version(2.1)
            .base_atk(608.0)
            .er(55.1)
    }
}

impl SpecialAbility for EngulfingLightning {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update(time, event.idx == self.idx && event.kind == Burst);
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        let state = &mut modifiable_data[self.idx.0].state;
        if self.timer.n == 1 {
            state.er += 30.0;
        }
        state.atk += 0.28 * state.er;
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct EverlastingMoonglow {
    idx: FieldCharacterIndex,
    did_na: bool,
    timer: DurationTimer,
}

impl EverlastingMoonglow {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Everlasting Moonglow").type_(Catalyst).version(2.1)
            .base_atk(608.0)
            // TODO healing bonus
            .hp(49.6)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx,
            did_na: false,
            timer: DurationTimer::new(12.0, &[0.0]),
        }
    }
}

impl SpecialAbility for EverlastingMoonglow {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.did_na = event.idx == self.idx && event.kind == Na;
        self.timer.update(time, event.idx == self.idx && event.kind == Burst);
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        let state = &mut modifiable_data[self.idx.0].state;
        // TODO incorrect
        state.flat_atk += 0.001 * state.HP();
        if self.timer.n == 1 && self.did_na {
            state.energy += 0.6;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct LuxuriousSeaLord {
    idx: FieldCharacterIndex,
    timer: NTimer,
    aa: Attack,
}

impl LuxuriousSeaLord {
    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            idx,
            timer: NTimer::new(&[15.0]),
            aa: Attack {
                kind: AdditionalAttack,
                element: &PHYSICAL_GAUGE,
                multiplier: 200.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.noop),
                idx,
            }
        }
    }

    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Luxurious Sea-Lord").type_(Claymore).version(2.1)
            .base_atk(454.0)
            .atk(55.1).burst_dmg(24.0)
    }
}

impl SpecialAbility for LuxuriousSeaLord {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.timer.update(time, event.idx == self.idx && (event.kind == Burst || event.kind == BurstDot));
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        if self.timer.ping && self.timer.n == 1 {
            atk_queue.push(&self.aa);
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct TheCatch(pub FieldCharacterIndex);

impl TheCatch {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("The Catch").type_(Polearm).version(2.1)
            .base_atk(510.0)
            .er(45.9).burst_dmg(32.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self(idx)
    }
}

impl SpecialAbility for TheCatch {
    fn intensify(&self, attack: &Attack) -> Option<State> {
        if self.0 == attack.idx {
            match &attack.kind {
                Burst |
                BurstDot => Some(State::new().cr(12.0)),
                _ => None,
            }
        } else {
            None
        }
    }
}
