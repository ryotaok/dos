use std::rc::Rc;
use std::cell::RefCell;

use crate::state::State;
use crate::types::{AttackType, WeaponType, FieldEnergy, VecFieldEnergy, Particle, Vision, PHYSICAL_GAUGE, DRAGONSBANE};
use crate::fc::{FieldCharacterIndex, SpecialAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{Attack, AttackEvent, ICDTimer, DurationTimer, Time};
// use crate::testutil;

use AttackType::*;
use WeaponType::*;
// use Vision::*;


// version 1.0

pub struct PrototypeStarglitterR5 {
    timer: DurationTimer,
}

impl PrototypeStarglitterR5 {
    pub fn new() -> Self {
        Self {
            timer: DurationTimer::new(12.0, &[0.0,0.0]),
        }
    }
}

impl PrototypeStarglitterR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Prototype Starglitter").type_(Polearm).version(1.0)
            .base_atk(510.0)
            .er(45.9)
    }
}

impl SpecialAbility for PrototypeStarglitterR5 {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        let should_update = event.kind == PressSkill || event.kind == HoldSkill;
        self.timer.update(time, should_update);
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, enemy: &mut Enemy) -> () {
        let state = &mut modifiable_state[data.idx.0];
        match (self.timer.ping, self.timer.n > 0) {
            (true, true) => {
                state.na_dmg += 16.0;
                state.ca_dmg += 16.0;
            },
            (true, false) => {
                state.na_dmg -= 16.0 * self.timer.previous_n as f32;
                state.ca_dmg -= 16.0 * self.timer.previous_n as f32;
            },
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct CrescentPikeR5 {
    timer: DurationTimer,
    aa: Attack,
    did_na: bool,
}

impl CrescentPikeR5 {
    pub fn new(idx: FieldCharacterIndex, icd_timer: &Rc<RefCell<ICDTimer>>) -> Self {
        Self {
            timer: DurationTimer::new(5.0, &[0.0]),
            aa: Attack {
                kind: AdditionalAttack,
                element: &PHYSICAL_GAUGE,
                multiplier: 40.0,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            },
            did_na: false,
        }
    }
}

impl CrescentPikeR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Crescent Pike").type_(Polearm).version(1.0)
            .base_atk(566.0)
            .physical_dmg(34.5)
    }
}

impl SpecialAbility for CrescentPikeR5 {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        let should_update = particles.has_particles();
        self.timer.update(time, should_update);
        self.did_na = event.idx == data.idx && (event.kind == Na || event.kind == Ca);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        match (self.did_na, &self.timer.dr) {
            (true, Time::Waiting(_)) => atk_queue.push(&self.aa),
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct DeathmatchR5;

impl SpecialAbility for DeathmatchR5 {}

impl DeathmatchR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Deathmatch").type_(Polearm).version(1.0)
            .base_atk(454.0)
            .atk(48.0).cr(36.8)
    }
}

// one stack is always active
pub struct BlackcliffPoleR5;

impl SpecialAbility for BlackcliffPoleR5 {}

impl BlackcliffPoleR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Blackcliff Pole").type_(Polearm).version(1.0)
            .base_atk(510.0)
            .atk(24.0).cd(55.1)
    }
}

pub struct RoyalSpearR5;

impl SpecialAbility for RoyalSpearR5 {}

impl RoyalSpearR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Royal Spear").type_(Polearm).version(1.0)
            .base_atk(565.0)
            .atk(27.6)
    }
}

pub struct WhiteTasselR5;

impl SpecialAbility for WhiteTasselR5 {}

impl WhiteTasselR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("White Tassel").type_(Polearm).version(1.0)
            .base_atk(401.0)
            .cr(23.4)
            .na_dmg(48.0)
    }
}

pub struct DragonsBaneR5;

impl DragonsBaneR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Dragon's Bane").type_(Polearm).version(1.0)
            .base_atk(454.0)
            .em(221.0)
    }
}

impl SpecialAbility for DragonsBaneR5 {
    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, enemy: &mut Enemy) -> () {
        let state = &mut modifiable_state[data.idx.0];
        match (&enemy.aura.aura, state.stacked_buff != DRAGONSBANE) {
            (Vision::Hydro, true) |
            (Vision::Pyro, true) => {
                state.all_dmg += 36.0;
                state.stacked_buff.turn_on(&DRAGONSBANE);
            },
            (Vision::Hydro, false) |
            (Vision::Pyro, false) => (),
            (_, false) => {
                state.all_dmg -= 36.0;
                state.stacked_buff.turn_off(&DRAGONSBANE);
            },
            _ => (),
        }
    }
}
