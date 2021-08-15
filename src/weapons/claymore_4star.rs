use std::rc::Rc;
use std::cell::RefCell;

use crate::state::State;
use crate::types::{AttackType, WeaponType, FieldEnergy, VecFieldEnergy, Particle, Vision, PHYSICAL_GAUGE, LIONSROAR};
use crate::fc::{FieldCharacterIndex, SpecialAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{Attack, AttackEvent, ICDTimer, ElementalAbsorption, NTimer, DurationTimer, ICDTimers};
use crate::testutil;

use AttackType::*;
use WeaponType::*;
// use Vision::*;

// version 1.0

pub struct PrototypeArchaicR5 {
    timer: NTimer,
    aa: Attack,
}

impl PrototypeArchaicR5 {
    pub fn new(idx: FieldCharacterIndex, icd_timer: &Rc<RefCell<ICDTimer>>) -> Self {
        Self {
            timer: NTimer::new(&[15.0]),
            aa: Attack {
                kind: AdditionalAttack,
                element: &PHYSICAL_GAUGE,
                multiplier: 480.0,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }
        }
    }

    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Prototype Archaic").type_(Claymore).version(1.0)
            .base_atk(566.0)
            .atk(27.6)
    }
}

impl SpecialAbility for PrototypeArchaicR5 {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        let should_update = event.idx == data.idx && (event.kind == Na || event.kind == Ca);
        self.timer.update(time, testutil::chance() < 0.5 && should_update);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        match (self.timer.ping, self.timer.n) {
            (true, 1) => atk_queue.push(&self.aa),
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct WhiteblindR5 {
    timer: DurationTimer,
}

impl WhiteblindR5 {
    pub fn new() -> Self {
        Self { timer: DurationTimer::new(6.0, &[0.5,0.5,0.5,0.5]) }
    }

    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Whiteblind").type_(Claymore).version(1.0)
            .base_atk(510.0)
            .def(51.7)
    }
}

impl SpecialAbility for WhiteblindR5 {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.timer.update(time, event.idx == data.idx && (event.kind == Na || event.kind == Ca));
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, enemy: &mut Enemy) -> () {
        let state = &mut modifiable_state[data.idx.0];
        match (self.timer.ping, self.timer.n > 0) {
            (true, true) => {
                state.atk += 12.0;
                state.def += 12.0;
            },
            (true, false) => {
                state.atk -= 12.0 * self.timer.previous_n as f32;
                state.def -= 12.0 * self.timer.previous_n as f32;
            },
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct SerpentSpineR5 {
    timer: DurationTimer,
}

impl SerpentSpineR5 {
    pub fn new() -> Self {
        Self { timer: DurationTimer::new(8.0, &[4.0,4.0,4.0,4.0,4.0]) }
    }

    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Serpent Spine").type_(Claymore).version(1.0)
            .base_atk(510.0)
            .cr(27.6)
    }
}

impl SpecialAbility for SerpentSpineR5 {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        // only the attacker can activate the passive
        self.timer.update(time, data.idx.0 == 0);
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, enemy: &mut Enemy) -> () {
        let state = &mut modifiable_state[data.idx.0];
        match (self.timer.ping, self.timer.n > 0) {
            (true, true) => {
                state.all_dmg += 10.0;
            },
            (true, false) => {
                state.all_dmg -= 10.0 * self.timer.previous_n as f32;
            },
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

// one stack is always active
pub struct BlackcliffSlasherR5;

impl SpecialAbility for BlackcliffSlasherR5 {}

impl BlackcliffSlasherR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Blackcliff Slasher").type_(Claymore).version(1.0)
            .base_atk(510.0)
            .atk(24.0).cd(55.1)
    }
}

pub struct RoyalGreatswordR5;

impl SpecialAbility for RoyalGreatswordR5 {}

impl RoyalGreatswordR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Royal Greatsword").type_(Claymore).version(1.0)
            .base_atk(565.0)
            .atk(27.6)
    }
}

pub struct RainslasherR5;

impl RainslasherR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Rainslasher").type_(Claymore).version(1.0)
            .base_atk(510.0)
            .em(165.0)
    }
}

impl SpecialAbility for RainslasherR5 {
    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, enemy: &mut Enemy) -> () {
        let state = &mut modifiable_state[data.idx.0];
        match (&enemy.aura.aura, state.stacked_buff != LIONSROAR) {
            (Vision::Electro, true) |
            (Vision::Hydro, true) => {
                state.all_dmg += 36.0;
                state.stacked_buff.turn_on(&LIONSROAR);
            },
            (Vision::Electro, false) |
            (Vision::Hydro, false) => (),
            (_, false) => {
                state.all_dmg -= 36.0;
                state.stacked_buff.turn_off(&LIONSROAR);
            },
            _ => (),
        }
    }
}

// The Bell
