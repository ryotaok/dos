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
    idx: FieldCharacterIndex,
    timer: NTimer,
    aa: Attack,
}

impl PrototypeArchaicR5 {
    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            idx,
            timer: NTimer::new(&[15.0]),
            aa: Attack {
                kind: AdditionalAttack,
                element: &PHYSICAL_GAUGE,
                multiplier: 480.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.noop),
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
        let should_update = event.idx == self.idx && (event.kind == Na || event.kind == Ca);
        self.timer.update(time, testutil::chance() < 0.5 && should_update);
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

pub struct WhiteblindR5 {
    idx: FieldCharacterIndex,
    timer: DurationTimer,
}

impl WhiteblindR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Whiteblind").type_(Claymore).version(1.0)
            .base_atk(510.0)
            .def(51.7)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx,
            timer: DurationTimer::new(6.0, &[0.5,0.5,0.5,0.5])
        }
    }
}

impl SpecialAbility for WhiteblindR5 {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.timer.update(time, event.idx == self.idx && (event.kind == Na || event.kind == Ca));
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if self.timer.n > 0 {
            let state = &mut modifiable_data[self.idx.0].state;
            state.atk += 12.0 * self.timer.n as f32;
            state.def += 12.0 * self.timer.n as f32;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct SerpentSpineR5 {
    idx: FieldCharacterIndex,
    timer: DurationTimer,
}

impl SerpentSpineR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Serpent Spine").type_(Claymore).version(1.0)
            .base_atk(510.0)
            .cr(27.6)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx,
            timer: DurationTimer::new(8.0, &[4.0,4.0,4.0,4.0,4.0])
        }
    }
}

impl SpecialAbility for SerpentSpineR5 {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        // only the attacker can activate the passive
        self.timer.update(time, data.idx.0 == 0);
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if self.timer.n > 0 {
            let state = &mut modifiable_data[self.idx.0].state;
            state.all_dmg += 10.0 * self.timer.n as f32;
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

pub struct RainslasherR5 {
    idx: FieldCharacterIndex,
}

impl RainslasherR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Rainslasher").type_(Claymore).version(1.0)
            .base_atk(510.0)
            .em(165.0)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx
        }
    }
}

impl SpecialAbility for RainslasherR5 {
    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if enemy.aura.aura == Vision::Electro || enemy.aura.aura == Vision::Hydro {
            let state = &mut modifiable_data[self.idx.0].state;
            state.all_dmg += 36.0;
        }
    }
}

// The Bell
