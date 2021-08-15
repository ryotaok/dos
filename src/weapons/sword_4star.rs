use std::ptr;

use crate::state::State;
use crate::types::{AttackType, WeaponType, FieldEnergy, Vision, PHYSICAL_GAUGE, LIONSROAR};
use crate::fc::{FieldCharacterIndex, SpecialAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{Attack, AttackEvent, DurationTimer};

use AttackType::*;
use WeaponType::*;
// use Vision::*;

// version 1.0

pub struct PrototypeRancourR5 {
    timer: DurationTimer,
}

impl PrototypeRancourR5 {
    pub fn new() -> Self {
        Self {
            timer: DurationTimer::new(6.0, &[0.3,0.3,0.3,0.3]),
        }
    }

    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Prototype Rancour").type_(Sword).version(1.0)
            .base_atk(566.0)
            .physical_dmg(34.5)
    }
}

impl SpecialAbility for PrototypeRancourR5 {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update(time, event.idx == data.idx && (event.kind == Na || event.kind == Ca));
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, _enemy: &mut Enemy) -> () {
        let state = &mut modifiable_state[data.idx.0];
        match (self.timer.ping, self.timer.n > 0) {
            (true, true) => {
                state.atk += 8.0;
                state.def += 8.0;
            },
            (true, false) => {
                state.atk -= 8.0 * self.timer.previous_n as f32;
                state.def -= 8.0 * self.timer.previous_n as f32;
            },
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

// iron sting

pub struct TheBlackSwordR5;

impl SpecialAbility for TheBlackSwordR5 {}
impl TheBlackSwordR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("The Black Sword").type_(Sword).version(1.0)
            .base_atk(510.0)
            .cr(27.6)
            .na_dmg(40.0).ca_dmg(40.0)
    }
}

// one stack is always active
pub struct BlackcliffLongswordR5;

impl SpecialAbility for BlackcliffLongswordR5 {}
impl BlackcliffLongswordR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Blackcliff Longsword").type_(Sword).version(1.0)
            .base_atk(565.0)
            .atk(24.0).cd(36.8)
    }
}

pub struct RoyalLongswordR5;

impl SpecialAbility for RoyalLongswordR5 {}
impl RoyalLongswordR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Royal Longsword").type_(Sword).version(1.0)
            .base_atk(565.0)
            .atk(27.6).cr(0.0)
    }
}

// the passive is always active
pub struct HarbingerOfDawnR5;

impl SpecialAbility for HarbingerOfDawnR5 {}
impl HarbingerOfDawnR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Harbinger of Dawn").type_(Sword).version(1.0)
            .base_atk(401.0)
            .cr(28.0).cd(46.9)
    }
}

pub struct TheFluteR5 {
    timer: DurationTimer,
    aa: Attack,
}

impl TheFluteR5 {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            timer: DurationTimer::new(30.0, &[0.5,0.5,0.5,0.5,0.5]),
            aa: Attack {
                kind: AdditionalAttack,
                element: &PHYSICAL_GAUGE,
                multiplier: 200.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            }
        }
    }

    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("The Flute").type_(Sword).version(1.0)
            .base_atk(510.0)
            .atk(41.3)
    }
}

impl SpecialAbility for TheFluteR5 {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        match (self.timer.ping, self.timer.n) {
            (true, 5) => self.timer.reset(),
            _ => (),
        };
        self.timer.update(time, event.idx == data.idx && (event.kind == Na || event.kind == Ca));
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, _particles: &mut Vec<FieldEnergy>, _data: &CharacterData) -> () {
        match (self.timer.ping, self.timer.n) {
            (true, 5) => atk_queue.push(&self.aa),
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct LionsRoarR5;

impl LionsRoarR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Lion's Roar").type_(Sword).version(1.0)
            .base_atk(510.0)
            .atk(41.3)
    }
}

impl SpecialAbility for LionsRoarR5 {
    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, enemy: &mut Enemy) -> () {
        let state = &mut modifiable_state[data.idx.0];
        match (&enemy.aura.aura, state.stacked_buff != LIONSROAR) {
            (Vision::Electro, true) |
            (Vision::Pyro, true) => {
                state.all_dmg += 36.0;
                state.stacked_buff.turn_on(&LIONSROAR);
            },
            (Vision::Electro, false) |
            (Vision::Pyro, false) => (),
            (_, false) => {
                state.all_dmg -= 36.0;
                state.stacked_buff.turn_off(&LIONSROAR);
            },
            _ => (),
        }
    }
}
