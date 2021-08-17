use crate::types::{AttackType, WeaponType, FieldEnergy, VecFieldEnergy, Particle};
use crate::fc::{FieldCharacterIndex, SpecialAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{Attack, AttackEvent, ICDTimer, ElementalAbsorption, NTimer, DurationTimer, ICDTimers};

use AttackType::*;
use WeaponType::*;
// use Vision::*;

pub struct Windfall {
    timer: NTimer,
}

impl Windfall {
    pub fn new() -> Self {
        Self { timer: NTimer::new(&[6.0]) }
    }
}

impl SpecialAbility for Windfall {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.timer.update(time, event.idx == data.idx && event.kind != StandStill);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        match (self.timer.ping, self.timer.n) {
            (true, 1) => particles.push_p(Particle::neutral(3.0 * data.state.cr / 100.0)),
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}


pub struct FavoniusGreatswordR5(Windfall);

impl FavoniusGreatswordR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Favonius Greatsword").type_(Claymore).version(1.0)
            .base_atk(454.0)
            .er(61.3)
    }

    pub fn new() -> Self {
        Self(Windfall::new())
    }
}

impl FavoniusGreatswordR5 {
}

impl SpecialAbility for FavoniusGreatswordR5 {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.0.update(time, event, data, attack, particles, enemy);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        self.0.additional_attack(atk_queue, particles, data);
    }

    fn reset(&mut self) -> () {
        self.0.reset();
    }
}

pub struct FavoniusSwordR5(Windfall);

impl FavoniusSwordR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Favonius Sword").type_(Sword).version(1.0)
            .base_atk(454.0)
            .er(61.3)
    }

    pub fn new() -> Self {
        Self(Windfall::new())
    }
}

impl FavoniusSwordR5 {
}

impl SpecialAbility for FavoniusSwordR5 {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.0.update(time, event, data, attack, particles, enemy);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        self.0.additional_attack(atk_queue, particles, data);
    }

    fn reset(&mut self) -> () {
        self.0.reset();
    }
}

pub struct FavoniusLanceR5(Windfall);

impl FavoniusLanceR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Favonius Lance").type_(Polearm).version(1.0)
            .base_atk(565.0)
            .er(30.6)
    }

    pub fn new() -> Self {
        Self(Windfall::new())
    }
}

impl FavoniusLanceR5 {
}

impl SpecialAbility for FavoniusLanceR5 {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.0.update(time, event, data, attack, particles, enemy);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        self.0.additional_attack(atk_queue, particles, data);
    }

    fn reset(&mut self) -> () {
        self.0.reset();
    }
}

pub struct FavoniusWarbowR5(Windfall);

impl FavoniusWarbowR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Favonius Warbow").type_(Bow).version(1.0)
            .base_atk(454.0)
            .er(61.3)
    }

    pub fn new() -> Self {
        Self(Windfall::new())
    }
}

impl FavoniusWarbowR5 {
}

impl SpecialAbility for FavoniusWarbowR5 {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.0.update(time, event, data, attack, particles, enemy);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        self.0.additional_attack(atk_queue, particles, data);
    }

    fn reset(&mut self) -> () {
        self.0.reset();
    }
}

pub struct FavoniusCodexR5(Windfall);

impl FavoniusCodexR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Favonius Codex").type_(Catalyst).version(1.0)
            .base_atk(510.0)
            .er(45.9)
    }

    pub fn new() -> Self {
        Self(Windfall::new())
    }
}

impl FavoniusCodexR5 {
}

impl SpecialAbility for FavoniusCodexR5 {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.0.update(time, event, data, attack, particles, enemy);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        self.0.additional_attack(atk_queue, particles, data);
    }

    fn reset(&mut self) -> () {
        self.0.reset();
    }
}
