use crate::sim1::types::{AttackType, WeaponType, FieldEnergy};
use crate::sim1::fc::{FieldCharacterIndex, SpecialAbility, CharacterData, WeaponRecord, Enemy};
use crate::sim1::action::{Attack, AttackEvent, ICDTimer, NTimer};

use AttackType::*;
use WeaponType::*;
// use Vision::*;

fn reset_cd(timer: &mut NTimer) -> () {
    timer.reset();
}

pub struct Composed {
    timer: NTimer,
}

impl Composed {
    pub fn new() -> Self {
        Self {
            timer: NTimer::new(&[16.0]),
        }
    }
}

impl SpecialAbility for Composed {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update(time, event.idx == data.idx && (event.kind == PressSkill || event.kind == HoldSkill));
    }

    fn accelerator(&self) -> Option<fn(&mut NTimer)> {
        if self.timer.n == 0 {
            Some(reset_cd)
        } else {
            None
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct SacrificialSwordR5(Composed);

impl SacrificialSwordR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Sacrificial Sword").type_(Sword).version(1.0)
            .base_atk(454.0)
            .er(61.3)
    }

    pub fn new() -> Self {
        Self(Composed::new())
    }
}

impl SpecialAbility for SacrificialSwordR5 {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.0.update(time, event, data, attack, particles, enemy);
    }

    fn accelerator(&self) -> Option<fn(&mut NTimer)> {
        self.0.accelerator()
    }

    fn reset(&mut self) -> () {
        self.0.reset();
    }
}

pub struct SacrificialGreatswordR5(Composed);

impl SacrificialGreatswordR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Sacrificial Greatsword").type_(Claymore).version(1.0)
            .base_atk(565.0)
            .er(30.6)
    }

    pub fn new() -> Self {
        Self(Composed::new())
    }
}

impl SpecialAbility for SacrificialGreatswordR5 {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.0.update(time, event, data, attack, particles, enemy);
    }

    fn accelerator(&self) -> Option<fn(&mut NTimer)> {
        self.0.accelerator()
    }

    fn reset(&mut self) -> () {
        self.0.reset();
    }
}

// pub struct SacrificialLanceR5(Composed);

pub struct SacrificialBowR5(Composed);

impl SacrificialBowR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Sacrificial Bow").type_(Bow).version(1.0)
            .base_atk(565.0)
            .er(30.6)
    }

    pub fn new() -> Self {
        Self(Composed::new())
    }
}

impl SpecialAbility for SacrificialBowR5 {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.0.update(time, event, data, attack, particles, enemy);
    }

    fn accelerator(&self) -> Option<fn(&mut NTimer)> {
        self.0.accelerator()
    }

    fn reset(&mut self) -> () {
        self.0.reset();
    }
}

pub struct SacrificialFragmentsR5(Composed);

impl SacrificialFragmentsR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Sacrificial Fragments").type_(Catalyst).version(1.0)
            .base_atk(454.0)
            .em(221.0)
    }

    pub fn new() -> Self {
        Self(Composed::new())
    }
}

impl SpecialAbility for SacrificialFragmentsR5 {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.0.update(time, event, data, attack, particles, enemy);
    }

    fn accelerator(&self) -> Option<fn(&mut NTimer)> {
        self.0.accelerator()
    }

    fn reset(&mut self) -> () {
        self.0.reset();
    }
}
