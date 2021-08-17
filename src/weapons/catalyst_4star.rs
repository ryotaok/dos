use std::rc::Rc;
use std::cell::RefCell;

use crate::state::State;
use crate::types::{AttackType, WeaponType, FieldEnergy, PHYSICAL_GAUGE};
use crate::fc::{FieldCharacterIndex, SpecialAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{Attack, AttackEvent, ICDTimers, NTimer, DurationTimer};
use crate::testutil;

use AttackType::*;
use WeaponType::*;
// use Vision::*;

// version 1.0

pub struct PrototypeAmberR5;

impl SpecialAbility for PrototypeAmberR5 {}

impl PrototypeAmberR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Prototype Amber").type_(Catalyst).version(1.0)
            .base_atk(510.0)
            .hp(41.3)
    }
}

pub struct MappaMareR5 {
    idx: FieldCharacterIndex,
    timer: DurationTimer,
}

impl MappaMareR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Mappa Mare").type_(Catalyst).version(1.0)
            .base_atk(565.0)
            .em(110.0)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx,
            timer: DurationTimer::new(10.0, &[0.0,0.0,]),
        }
    }
}

impl SpecialAbility for MappaMareR5 {
    fn update(&mut self, time: f32, event: &AttackEvent, _data: &CharacterData, attack: &[*const Attack], _particles: &[FieldEnergy], enemy: &Enemy) -> () {
        let should_update = unsafe {
            attack.iter().any(|&a| (*a).idx == self.idx && enemy.trigger_er(&(*a).element.aura).is_triggered())
        };
        self.timer.update(time, should_update);
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        let state = &mut modifiable_data[self.idx.0].state;
        match (self.timer.ping, self.timer.n > 0) {
            (true, true) => state.elemental_dmg += 16.0,
            (true, false) => state.elemental_dmg -= 16.0 * self.timer.previous_n as f32,
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct SolarPearlR5 {
    idx: FieldCharacterIndex,
    na_timer: DurationTimer,
    skill_timer: DurationTimer,
}

impl SolarPearlR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Solar Pearl").type_(Catalyst).version(1.0)
            .base_atk(510.0)
            .cr(27.6)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx,
            na_timer: DurationTimer::new(6.0, &[0.0]),
            skill_timer: DurationTimer::new(6.0, &[0.0]),
        }
    }
}

impl SpecialAbility for SolarPearlR5 {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        let check_idx = event.idx == self.idx;
        self.na_timer.update(time, check_idx && event.kind == Na);
        self.skill_timer.update(time, check_idx && (event.kind == Burst || event.kind == PressSkill || event.kind == HoldSkill));
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        let state = &mut modifiable_data[self.idx.0].state;
        match (self.skill_timer.ping, self.skill_timer.n > 0) {
            (true, true) => state.na_dmg += 40.0,
            (true, false) => state.na_dmg -= 40.0,
            _ => (),
        }
        match (self.na_timer.ping, self.na_timer.n > 0) {
            (true, true) => {
                state.skill_dmg += 40.0;
                state.burst_dmg += 40.0;
            },
            (true, false) => {
                state.skill_dmg -= 40.0;
                state.burst_dmg -= 40.0;
            },
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.na_timer.reset();
        self.skill_timer.reset();
    }
}

// one stack is always active
pub struct BlackcliffAgateR5;

impl SpecialAbility for BlackcliffAgateR5 {}

impl BlackcliffAgateR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Blackcliff Agate").type_(Catalyst).version(1.0)
            .base_atk(510.0)
            .atk(24.0).cd(55.1)
    }
}

pub struct RoyalGrimoireR5;

impl SpecialAbility for RoyalGrimoireR5 {}

impl RoyalGrimoireR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Royal Grimoire").type_(Catalyst).version(1.0)
            .base_atk(565.0)
            .atk(27.6)
    }
}

pub struct ThrillingTalesOfDragonSlayersR5 {
    timer: DurationTimer,
}

impl ThrillingTalesOfDragonSlayersR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Thrilling Tales of Dragon Slayers").type_(Catalyst).version(1.0)
            .base_atk(401.0)
            .hp(35.2)
    }

    pub fn new() -> Self {
        Self {
            timer: DurationTimer::new(10.0, &[20.0]),
        }
    }
}

impl SpecialAbility for ThrillingTalesOfDragonSlayersR5 {
    fn update(&mut self, time: f32, _event: &AttackEvent, _data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update(time, true);
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        // always buff the first member
        let state = &mut modifiable_data[0].state;
        match (self.timer.ping, self.timer.n > 0) {
            (true, true) => state.atk += 48.0,
            (true, false) => state.atk -= 48.0,
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct EyeOfPerceptionR5 {
    idx: FieldCharacterIndex,
    timer: NTimer,
    aa: Attack,
}

impl EyeOfPerceptionR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Eye of Perception").type_(Catalyst).version(1.0)
            .base_atk(454.0)
            .atk(55.1)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            idx,
            timer: NTimer::new(&[8.0]),
            aa: Attack {
                kind: AdditionalAttack,
                element: &PHYSICAL_GAUGE,
                multiplier: 360.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.noop),
                idx,
            }
        }
    }
}

impl SpecialAbility for EyeOfPerceptionR5 {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        let should_update = event.idx == self.idx && (event.kind == Na || event.kind == Ca);
        self.timer.update(time, testutil::chance() < 0.5 && should_update);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, _particles: &mut Vec<FieldEnergy>, _data: &CharacterData) -> () {
        match (self.timer.ping, self.timer.n) {
            (true, 1) => atk_queue.push(&self.aa),
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct TheWidsithR5 {
    idx: FieldCharacterIndex,
    random_theme_song: usize,
    timer: DurationTimer,
}

impl TheWidsithR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("The Widsith").type_(Catalyst).version(1.0)
            .base_atk(510.0)
            .cd(55.1)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx,
            random_theme_song: 0,
            timer: DurationTimer::new(10.0, &[30.0]),
        }
    }
}

impl SpecialAbility for TheWidsithR5 {
    fn update(&mut self, time: f32, _event: &AttackEvent, _data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update(time, true);
        // check if the first time to gain the theme
        if self.timer.ping && self.timer.n == 1 {
            let p = testutil::chance();
            self.random_theme_song = if p > 0.6666 {
                0
            } else if p > 0.3333 {
                1
            } else {
                2
            };
        }
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        match (self.timer.ping, self.timer.n) {
            (true, 1) => match self.random_theme_song {
                0 => modifiable_data[self.idx.0].state.atk += 120.0,
                1 => modifiable_data[self.idx.0].state.all_dmg += 96.0,
                2 => modifiable_data[self.idx.0].state.em += 480.0,
                _ => (),
            },
            (true, 0) => match self.random_theme_song {
                0 => modifiable_data[self.idx.0].state.atk -= 120.0,
                1 => modifiable_data[self.idx.0].state.all_dmg -= 96.0,
                2 => modifiable_data[self.idx.0].state.em -= 480.0,
                _ => (),
            },
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}
