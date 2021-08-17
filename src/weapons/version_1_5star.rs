use std::rc::Rc;
use std::cell::RefCell;

use crate::state::State;
use crate::types::{AttackType, WeaponType, FieldEnergy, PHYSICAL_GAUGE};
use crate::fc::{FieldCharacterIndex, SpecialAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{Attack, AttackEvent, ICDTimers, NTimer, DurationTimer, Time};
use crate::testutil;

use AttackType::*;
use WeaponType::*;
// use Vision::*;

// version 1.0

pub struct SkywardBlade {
    idx: FieldCharacterIndex,
    timer: DurationTimer,
    did_na: bool,
    aa: Attack,
}

impl SkywardBlade {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Skyward Blade").type_(Sword).version(1.0)
            .base_atk(608.0)
            .cr(4.0).er(55.1)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            idx,
            timer: DurationTimer::new(12.0, &[0.0]),
            did_na: false,
            aa: Attack {
                kind: AdditionalAttack,
                element: &PHYSICAL_GAUGE,
                multiplier: 20.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.noop),
                idx,
            },
        }
    }
}

impl SpecialAbility for SkywardBlade {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        let check_idx = event.idx == self.idx;
        self.did_na = check_idx && (event.kind == Na || event.kind == Ca);
        self.timer.update(time, check_idx && event.kind == Burst);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, _particles: &mut Vec<FieldEnergy>, _data: &CharacterData) -> () {
        match (&self.timer.dr, self.did_na) {
            (Time::Waiting(_), true) => atk_queue.push(&self.aa),
            _ => (),
        };
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        let state = &mut modifiable_data[self.idx.0].state;
        match (self.timer.ping, self.timer.n) {
            (true, 1) => state.atk_spd += 18.0,
            (true, 0) => state.atk_spd -= 18.0,
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct AquilaFavonia {
    idx: FieldCharacterIndex,
    timer: NTimer,
    aa: Attack,
}

impl AquilaFavonia {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Aquila Favonia").type_(Sword).version(1.0)
            .base_atk(674.0)
            .atk(20.0)
            .physical_dmg(41.3)
    }

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
            },
        }
    }

 }
impl SpecialAbility for AquilaFavonia {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        let should_update = event.idx == self.idx && (event.kind == Na || event.kind == Ca);
        self.timer.update(time, should_update);
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

pub struct SkywardPride {
    idx: FieldCharacterIndex,
    timer: NTimer,
    aa: Attack,
}

impl SkywardPride {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Skyward Pride").type_(Claymore).version(1.0)
            .base_atk(674.0)
            .er(36.8)
            .na_dmg(8.0).ca_dmg(8.0).skill_dmg(8.0).burst_dmg(8.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            idx,
            timer: NTimer::new(&[0.5,0.5,0.5,0.5,0.5,0.5,0.5,0.5,]),
            aa: Attack {
                kind: AdditionalAttack,
                element: &PHYSICAL_GAUGE,
                multiplier: 80.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.noop),
                idx,
            },
        }
    }
 }

impl SpecialAbility for SkywardPride {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update(time, event.idx == self.idx && event.kind == Burst);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, _particles: &mut Vec<FieldEnergy>, _data: &CharacterData) -> () {
        match (self.timer.ping, self.timer.n > 0) {
            (true, true) => atk_queue.push(&self.aa),
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct WolfsGravestone;

impl WolfsGravestone {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Wolf's Gravestone").type_(Claymore).version(1.0)
            .base_atk(608.0)
            .atk(49.6 + 20.0)
    }
    // TODO Box::new(FixedStack::new(MovementActivator, 30.0, 12.0, Atk(40.0)))
}

impl SpecialAbility for WolfsGravestone {}

pub struct SkywardSpine {
    idx: FieldCharacterIndex,
    timer: NTimer,
    aa: Attack,
}

impl SkywardSpine {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Skyward Spine").type_(Polearm).version(1.0)
            .base_atk(674.0)
            .cr(8.0).er(36.8).atk_spd(12.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            idx,
            timer: NTimer::new(&[2.0]),
            aa: Attack {
                kind: AdditionalAttack,
                element: &PHYSICAL_GAUGE,
                multiplier: 40.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.noop),
                idx,
            }
        }
    }
 }

impl SpecialAbility for SkywardSpine {
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

pub struct PrimordialJadeWingedSpear {
    idx: FieldCharacterIndex,
    timer: DurationTimer,
}

impl PrimordialJadeWingedSpear {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Primordial Jade Winged-Spear").type_(Polearm).version(1.0)
            .base_atk(674.0)
            .cr(22.1)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx,
            timer: DurationTimer::new(6.0, &[0.3,0.3,0.3,0.3,0.3,0.3,0.3])
        }
    }
 }

impl SpecialAbility for PrimordialJadeWingedSpear {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update(time, event.idx == self.idx && event.kind != StandStill);
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        let state = &mut modifiable_data[self.idx.0].state;
        match (self.timer.ping, self.timer.n > 0) {
            (true, true) => {
                state.atk += 3.2;
                if self.timer.n == 7 {
                    state.all_dmg += 12.0;
                }
            },
            (true, false) => {
                state.atk -= 3.2 * self.timer.previous_n as f32;
                if self.timer.previous_n == 7 {
                    state.all_dmg -= 12.0;
                }
            },
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct SkywardHarp {
    idx: FieldCharacterIndex,
    timer: NTimer,
    aa: Attack,
}

impl SkywardHarp {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Skyward Harp").type_(Bow).version(1.0)
            .base_atk(674.0)
            .cr(22.1).cd(20.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            idx,
            timer: NTimer::new(&[4.0]),
            aa: Attack {
                kind: AdditionalAttack,
                element: &PHYSICAL_GAUGE,
                multiplier: 125.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.noop),
                idx,
            }
        }
    }
 }

impl SpecialAbility for SkywardHarp {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update(time, testutil::chance() < 0.6 && event.idx == self.idx && event.kind != StandStill);
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

pub struct AmosBow;

impl AmosBow {
     pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Amos' Bow").type_(Bow).version(1.0)
            .base_atk(608.0)
            .atk(49.6)
            .na_dmg(12.0 + 40.0).ca_dmg(12.0 + 40.0)
    }
}

impl SpecialAbility for AmosBow {}

pub struct SkywardAtlas {
    idx: FieldCharacterIndex,
    timer: NTimer,
    aa: Attack,
}

impl SkywardAtlas {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Skyward Atlas").type_(Catalyst).version(1.0)
            .base_atk(674.0)
            .atk(33.1)
            .pyro_dmg(12.0).cryo_dmg(12.0).hydro_dmg(12.0).electro_dmg(12.0).anemo_dmg(12.0).geo_dmg(12.0).dendro_dmg(12.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            idx,
            timer: NTimer::new(&[2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0, 14.0]),
            aa: Attack {
                kind: AdditionalAttack,
                element: &PHYSICAL_GAUGE,
                multiplier: 160.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.noop),
                idx,
            }
        }
    }
 }

impl SpecialAbility for SkywardAtlas {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        let should_update = event.idx == self.idx && (event.kind == Na || event.kind == Ca);
        self.timer.update(time, testutil::chance() < 0.5 && should_update);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, _particles: &mut Vec<FieldEnergy>, _data: &CharacterData) -> () {
        if self.timer.ping && 0 < self.timer.n && self.timer.n <= 8 {
            atk_queue.push(&self.aa);
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct LostPrayerToTheSacredWinds {
    idx: FieldCharacterIndex,
    timer: DurationTimer,
}

impl LostPrayerToTheSacredWinds {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Lost Prayer to the Sacred Winds").type_(Catalyst).version(1.0)
            .base_atk(608.0)
            .cr(33.1)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx,
            timer: DurationTimer::new(8.0, &[4.0,4.0,4.0,4.0])
        }
    }
 }

impl SpecialAbility for LostPrayerToTheSacredWinds {
    fn update(&mut self, time: f32, _event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        // only the attacker can activate the passive
        self.timer.update(time, data.idx.0 == 0);
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        let state = &mut modifiable_data[self.idx.0].state;
        match (self.timer.ping, self.timer.n > 0) {
            (true, true) => state.elemental_dmg += 8.0,
            (true, false) => state.elemental_dmg -= 8.0 * self.timer.previous_n as f32,
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}
