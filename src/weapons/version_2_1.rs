use crate::state::State;
use crate::types::{AttackType, WeaponType, FieldEnergy};
use crate::fc::{SpecialAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{Attack, AttackEvent, ICDTimer, DurationTimer};

use AttackType::*;
use WeaponType::*;
// use Vision::*;

pub struct GrasscuttersLight {
    once: bool,
    timer: DurationTimer,
}

impl GrasscuttersLight {
    pub fn new() -> Self {
        Self {
            once: true,
            timer: DurationTimer::new(12.0, &[0.0]),
        }
    }
}

impl GrasscuttersLight {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Grasscutter's Light").type_(Polearm).version(2.1)
            .base_atk(608.0)
            .er(55.1)
    }
}

impl SpecialAbility for GrasscuttersLight {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        if self.once {
            self.once = false;
        }
        self.timer.update(time, event.idx == data.idx && event.kind == Burst);
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, _enemy: &mut Enemy) -> () {
        let state = &mut modifiable_state[data.idx.0];
        if self.once {
            state.atk += 0.28 * data.state().er;
        }
        match (self.timer.ping, self.timer.n) {
            (true, 1) => state.er += 30.0,
            (true, 0) => state.er -= 30.0,
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.once = true;
        self.timer.reset();
    }
}

pub struct FumetsuGekka {
    did_na: bool,
    once: bool,
    timer: DurationTimer,
}

impl FumetsuGekka {
    pub fn new() -> Self {
        Self {
            did_na: false,
            once: true,
            timer: DurationTimer::new(12.0, &[0.0]),
        }
    }
}

impl FumetsuGekka {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Fumetsu Gekka").type_(Catalyst).version(2.1)
            .base_atk(608.0)
            // TODO healing bonus
            .hp(49.6)
    }
}

impl SpecialAbility for FumetsuGekka {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        if self.once {
            self.once = false;
        }
        self.did_na = event.idx == data.idx && event.kind == Burst;
        self.timer.update(time, event.idx == data.idx && event.kind == Burst);
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.once {
            let state = &mut modifiable_state[data.idx.0];
            // TODO incorrect
            state.na_dmg += 0.0001 * data.state().HP();
        }
        if self.timer.n == 1 && self.did_na {
            let state = &mut modifiable_state[data.idx.0];
            state.energy += state.ER() * 0.6;
        }
    }

    fn reset(&mut self) -> () {
        self.once = true;
        self.timer.reset();
    }
}
