use crate::types::{AttackType, WeaponType, Particle};
use crate::fc::{SpecialAbility, WeaponAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{ElementalAttack, FullCharacterTimers, TimerGuard, EffectTimer, HitsTimer};

use AttackType::*;
use WeaponType::*;
// use Vision::*;

pub struct Windfall {
    timer: HitsTimer,
}

impl Windfall {
    pub fn new() -> Self {
        Self { timer: HitsTimer::new(6.0, 1) }
    }
}

impl SpecialAbility for Windfall {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let should_update = unsafe {
            attack.iter().any(|&a|
                match (*a.atk).kind {
                    Na | Ca | PressSkill | HoldSkill | SkillDot | Burst | BurstDot => true,
                    _ => false,
                }
            )
        };
        self.timer.update(guard.second(should_update), time);
    }

    fn additional_attack(&self, _atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &Enemy) -> () {
        if self.timer.is_active() {
            particles.push(Particle::neutral(3.0 * data.state.cr / 100.0));
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}


pub struct FavoniusGreatswordR5(Windfall);

impl FavoniusGreatswordR5 {
    pub fn new() -> Self {
        Self(Windfall::new())
    }
}

impl WeaponAbility for FavoniusGreatswordR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Favonius Greatsword R5").type_(Claymore).version(1.0)
            .base_atk(454.0)
            .er(61.3)
    }
}

impl SpecialAbility for FavoniusGreatswordR5 {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.0.update(guard, timers, attack, particles, data, enemy, time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        self.0.additional_attack(atk_queue, particles, timers, data, enemy);
    }

    fn reset(&mut self) -> () {
        self.0.reset();
    }
}

pub struct FavoniusSwordR5(Windfall);

impl FavoniusSwordR5 {
    pub fn new() -> Self {
        Self(Windfall::new())
    }
}

impl WeaponAbility for FavoniusSwordR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Favonius Sword R5").type_(Sword).version(1.0)
            .base_atk(454.0)
            .er(61.3)
    }
}

impl SpecialAbility for FavoniusSwordR5 {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.0.update(guard, timers, attack, particles, data, enemy, time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        self.0.additional_attack(atk_queue, particles, timers, data, enemy);
    }

    fn reset(&mut self) -> () {
        self.0.reset();
    }
}

pub struct FavoniusLanceR5(Windfall);

impl FavoniusLanceR5 {
    pub fn new() -> Self {
        Self(Windfall::new())
    }
}

impl WeaponAbility for FavoniusLanceR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Favonius Lance R5").type_(Polearm).version(1.0)
            .base_atk(565.0)
            .er(30.6)
    }
}

impl SpecialAbility for FavoniusLanceR5 {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.0.update(guard, timers, attack, particles, data, enemy, time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        self.0.additional_attack(atk_queue, particles, timers, data, enemy);
    }

    fn reset(&mut self) -> () {
        self.0.reset();
    }
}

pub struct FavoniusWarbowR5(Windfall);

impl FavoniusWarbowR5 {
    pub fn new() -> Self {
        Self(Windfall::new())
    }
}

impl WeaponAbility for FavoniusWarbowR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Favonius Warbow R5").type_(Bow).version(1.0)
            .base_atk(454.0)
            .er(61.3)
    }
}

impl SpecialAbility for FavoniusWarbowR5 {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.0.update(guard, timers, attack, particles, data, enemy, time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        self.0.additional_attack(atk_queue, particles, timers, data, enemy);
    }

    fn reset(&mut self) -> () {
        self.0.reset();
    }
}

pub struct FavoniusCodexR5(Windfall);

impl FavoniusCodexR5 {
    pub fn new() -> Self {
        Self(Windfall::new())
    }
}

impl WeaponAbility for FavoniusCodexR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Favonius Codex R5").type_(Catalyst).version(1.0)
            .base_atk(510.0)
            .er(45.9)
    }
}

impl SpecialAbility for FavoniusCodexR5 {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.0.update(guard, timers, attack, particles, data, enemy, time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        self.0.additional_attack(atk_queue, particles, timers, data, enemy);
    }

    fn reset(&mut self) -> () {
        self.0.reset();
    }
}
