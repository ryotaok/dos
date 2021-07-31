use crate::types::{AttackType, WeaponType, FieldEnergy, VecFieldEnergy, Particle};
use crate::fc::{SpecialAbility, WeaponAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{ElementalAttack, FullCharacterTimers, TimerGuard, EffectTimer, HitsTimer};

use AttackType::*;
use WeaponType::*;
// use Vision::*;

pub struct Composed {
    timer: HitsTimer,
}

impl Composed {
    pub fn new() -> Self {
        Self {
            timer: HitsTimer::new(16.0, 1),
        }
    }
}

impl WeaponAbility for Composed {
    fn accelerate(&self, timers: &mut FullCharacterTimers) -> () {
        if self.timer.is_active() {
            timers.reset_cd();
        }
    }
}

impl SpecialAbility for Composed {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[FieldEnergy], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let should_update = guard.kind == PressSkill || guard.kind == HoldSkill;
        self.timer.update(guard.second(should_update), time);
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct SacrificialSwordR5(Composed);

impl SacrificialSwordR5 {
    pub fn new() -> Self {
        Self(Composed::new())
    }
}

impl WeaponAbility for SacrificialSwordR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Sacrificial Sword").type_(Sword).version(1.0)
            .base_atk(454.0)
            .er(61.3)
    }

    fn accelerate(&self, timers: &mut FullCharacterTimers) -> () {
        self.0.accelerate(timers);
    }
}

impl SpecialAbility for SacrificialSwordR5 {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[FieldEnergy], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.0.update(guard, timers, attack, particles, data, enemy, time);
    }

    fn reset(&mut self) -> () {
        self.0.reset();
    }
}

pub struct SacrificialGreatswordR5(Composed);

impl SacrificialGreatswordR5 {
    pub fn new() -> Self {
        Self(Composed::new())
    }
}

impl WeaponAbility for SacrificialGreatswordR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Sacrificial Greatsword").type_(Claymore).version(1.0)
            .base_atk(565.0)
            .er(30.6).em(0.0).atk_spd(0.0)
    }

    fn accelerate(&self, timers: &mut FullCharacterTimers) -> () {
        self.0.accelerate(timers);
    }
}

impl SpecialAbility for SacrificialGreatswordR5 {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[FieldEnergy], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.0.update(guard, timers, attack, particles, data, enemy, time);
    }

    fn reset(&mut self) -> () {
        self.0.reset();
    }
}

// pub struct SacrificialLanceR5(Composed);

pub struct SacrificialBowR5(Composed);

impl SacrificialBowR5 {
    pub fn new() -> Self {
        Self(Composed::new())
    }
}

impl WeaponAbility for SacrificialBowR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Sacrificial Bow").type_(Bow).version(1.0)
            .base_atk(565.0)
            .er(30.6)
    }

    fn accelerate(&self, timers: &mut FullCharacterTimers) -> () {
        self.0.accelerate(timers);
    }
}

impl SpecialAbility for SacrificialBowR5 {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[FieldEnergy], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.0.update(guard, timers, attack, particles, data, enemy, time);
    }

    fn reset(&mut self) -> () {
        self.0.reset();
    }
}

pub struct SacrificialFragmentsR5(Composed);

impl SacrificialFragmentsR5 {
    pub fn new() -> Self {
        Self(Composed::new())
    }
}

impl WeaponAbility for SacrificialFragmentsR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Sacrificial Fragments").type_(Catalyst).version(1.0)
            .base_atk(454.0)
            .em(221.0)
    }

    fn accelerate(&self, timers: &mut FullCharacterTimers) -> () {
        self.0.accelerate(timers);
    }
}

impl SpecialAbility for SacrificialFragmentsR5 {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[FieldEnergy], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.0.update(guard, timers, attack, particles, data, enemy, time);
    }

    fn reset(&mut self) -> () {
        self.0.reset();
    }
}
