use std::rc::Rc;
use std::cell::RefCell;

use crate::sim1::state::State;
use crate::sim1::types::{AttackType, WeaponType, FieldEnergy, VecFieldEnergy, Particle, PHYSICAL_GAUGE};
use crate::sim1::fc::{FieldCharacterIndex, SpecialAbility, CharacterData, WeaponRecord, Enemy};
use crate::sim1::action::{Attack, AttackEvent, ICDTimer, ElementalAbsorption, NTimer, DurationTimer, ICDTimers};
use crate::sim1::testutil;

use AttackType::*;
use WeaponType::*;
// use Vision::*;

// version 1.0

pub struct PrototypeCrescentR5;

impl SpecialAbility for PrototypeCrescentR5 {}

impl PrototypeCrescentR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Prototype Crescent").type_(Bow).version(1.0)
            .base_atk(510.0)
            .atk(41.3 + 72.0)
    }
}

pub struct CompoundBowR5 {
    idx: FieldCharacterIndex,
    timer: DurationTimer,
}

impl CompoundBowR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Compound Bow").type_(Bow).version(1.0)
            .base_atk(454.0)
            .physical_dmg(69.0)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx,
            timer: DurationTimer::new(6.0, &[0.3,0.3,0.3,0.3]),
        }
    }
}

impl SpecialAbility for CompoundBowR5 {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.timer.update(time, event.idx == self.idx && (event.kind == Na || event.kind == Ca));
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if self.timer.n > 0 {
            let state = &mut modifiable_data[self.idx.0].state;
            state.atk += 8.0 * self.timer.n as f32;
            state.atk_spd += 2.4 * self.timer.n as f32;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct TheViridescentHuntR5 {
    idx: FieldCharacterIndex,
    timer: NTimer,
    aa: Attack,
}

impl TheViridescentHuntR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("The Viridescent Hunt").type_(Bow).version(1.0)
            .base_atk(510.0)
            .cr(27.6)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            idx,
            timer: NTimer::new(&[0.5,0.5, 0.5,0.5, 0.5,0.5, 0.5,0.5, 6.0]),
            aa: Attack {
                kind: AdditionalAttack,
                element: &PHYSICAL_GAUGE,
                multiplier: 80.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.noop),
                idx,
            }
        }
    }
}

impl SpecialAbility for TheViridescentHuntR5 {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        let should_update = event.idx == self.idx && (event.kind == Na || event.kind == Ca);
        self.timer.update(time, testutil::chance() < 0.5 && should_update);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        if self.timer.ping && self.timer.n > 0 {
            atk_queue.push(&self.aa);
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

// one stack is always active
pub struct BlackcliffWarbowR5;

impl SpecialAbility for BlackcliffWarbowR5 {}

impl BlackcliffWarbowR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Blackcliff Warbow").type_(Bow).version(1.0)
            .base_atk(510.0)
            .atk(24.0).cd(55.1)
    }
}

pub struct RoyalBowR5;

impl SpecialAbility for RoyalBowR5 {}

impl RoyalBowR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Royal Bow").type_(Bow).version(1.0)
            .base_atk(510.0)
            .atk(41.3)
    }
}

pub struct SlingshotR5;

impl SpecialAbility for SlingshotR5 {}

impl SlingshotR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Slingshot").type_(Bow).version(1.0)
            .base_atk(354.0)
            .cr(31.2)
            .na_dmg(60.0).ca_dmg(60.0)
    }
}

pub struct RustR5;

impl SpecialAbility for RustR5 {}

impl RustR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Rust").type_(Bow).version(1.0)
            .base_atk(510.0)
            .atk(41.3)
            .na_dmg(80.0).ca_dmg(-10.0)
    }
}

pub struct TheStringlessR5;

impl SpecialAbility for TheStringlessR5 {}

impl TheStringlessR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("The Stringless").type_(Bow).version(1.0)
            .base_atk(510.0)
            .em(165.0)
            .skill_dmg(48.0).burst_dmg(48.0)
    }
}
