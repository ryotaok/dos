use std::rc::Rc;
use std::cell::RefCell;

use crate::sim1::state::State;
use crate::sim1::types::{AttackType, WeaponType, FieldEnergy, Vision, PHYSICAL_GAUGE};
use crate::sim1::fc::{FieldCharacterIndex, SpecialAbility, CharacterData, WeaponRecord, Enemy};
use crate::sim1::action::{Attack, AttackEvent, ICDTimers, NTimer};
use crate::sim1::testutil;

use AttackType::*;
use WeaponType::*;
use Vision::*;

pub struct FesteringDesire(pub FieldCharacterIndex);

impl FesteringDesire {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Festering Desire").type_(Sword).version(1.2)
            .base_atk(510.0)
            .er(45.9).skill_dmg(32.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self(idx)
    }
}

impl SpecialAbility for FesteringDesire {
    fn intensify(&self, attack: &Attack) -> Option<State> {
        if self.0 == attack.idx {
            match &attack.kind {
                PressSkill |
                HoldSkill |
                SkillDot => Some(State::new().cr(12.0)),
                _ => None,
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct FrostBurial {
    idx: FieldCharacterIndex,
    cryo: bool,
    timer: NTimer,
    aa: Attack,
    aa_cryo: Attack,
}

impl FrostBurial {
    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            idx,
            cryo: false,
            timer: NTimer::new(&[10.0]),
            aa: Attack {
                kind: AdditionalAttack,
                element: &PHYSICAL_GAUGE,
                multiplier: 140.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.noop),
                idx,
            },
            aa_cryo: Attack {
                kind: AdditionalAttack,
                element: &PHYSICAL_GAUGE,
                multiplier: 360.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.noop),
                idx,
            },
        }
    }
}

impl SpecialAbility for FrostBurial {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], enemy: &Enemy) -> () {
        let should_update = event.idx == self.idx && (event.kind == Na || event.kind == Ca);
        self.timer.update(time, testutil::chance() < 0.5 && should_update);
        self.cryo = enemy.aura.aura == Cryo;
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, _particles: &mut Vec<FieldEnergy>, _data: &CharacterData) -> () {
        match (self.timer.ping, self.timer.n) {
            (true, 1) => atk_queue.push(if self.cryo {
                    &self.aa_cryo
                } else {
                    &self.aa
                }),
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct SnowTombedStarsilver(FrostBurial);

impl SnowTombedStarsilver {
    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self(FrostBurial::new(idx, icd_timer))
    }
}

impl SnowTombedStarsilver {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Snow-Tombed Starsilver").type_(Claymore).version(1.2)
            .base_atk(565.0)
            .physical_dmg(34.5)
    }
}

impl SpecialAbility for SnowTombedStarsilver {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.0.update(time, event, data, attack, particles, enemy);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        self.0.additional_attack(atk_queue, particles, data);
    }

    fn reset(&mut self) -> () { self.0.reset() }
}

pub struct DragonspineSpear(FrostBurial);

impl DragonspineSpear {
    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self(FrostBurial::new(idx, icd_timer))
    }
}

impl DragonspineSpear {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Dragonspine Spear").type_(Polearm).version(1.2)
            .base_atk(454.0)
            .physical_dmg(69.0)
    }
}

impl SpecialAbility for DragonspineSpear {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.0.update(time, event, data, attack, particles, enemy);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        self.0.additional_attack(atk_queue, particles, data);
    }

    fn reset(&mut self) -> () { self.0.reset() }
}

pub struct Frostbearer(FrostBurial);

impl Frostbearer {
    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self(FrostBurial::new(idx, icd_timer))
    }
}

impl Frostbearer {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Frostbearer").type_(Catalyst).version(1.2)
            .base_atk(510.0)
            .atk(41.3)
    }
}

impl SpecialAbility for Frostbearer {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.0.update(time, event, data, attack, particles, enemy);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        self.0.additional_attack(atk_queue, particles, data);
    }

    fn reset(&mut self) -> () { self.0.reset() }
}
