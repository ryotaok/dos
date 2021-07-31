use crate::state::State;
use crate::types::{AttackType, WeaponType, FieldEnergy, VecFieldEnergy, Particle};
use crate::fc::{SpecialAbility, WeaponAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{ElementalAttack, FullCharacterTimers, TimerGuard, EffectTimer, StackTimer};

use AttackType::*;
use WeaponType::*;
// use Vision::*;

pub struct GoldenMajesty {
    timer: StackTimer,
}

impl GoldenMajesty {
    pub fn new() -> Self {
        Self {
            timer: StackTimer::new(0.3, 8.0, 5),
        }
    }
}

impl WeaponAbility for GoldenMajesty {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .version(1.1)
            .base_atk(608.0)
            .atk(49.6)
    }
}

impl SpecialAbility for GoldenMajesty {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, attack: &[ElementalAttack], _particles: &[FieldEnergy], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
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

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            modifiable_state[data.idx.0].atk += 8.0 * self.timer.n as f32;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct TheUnforged(GoldenMajesty);

impl TheUnforged {
    pub fn new() -> Self {
        Self(GoldenMajesty::new())
    }
}

impl WeaponAbility for TheUnforged {
    fn record(&self) -> WeaponRecord {
        self.0.record().name("The Unforged").type_(Claymore)
    }
}

impl SpecialAbility for TheUnforged {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[FieldEnergy], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.0.update(guard, timers, attack, particles, data, enemy, time);
    }

    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, timers, data, enemy);
    }

    fn reset(&mut self) -> () { self.0.reset() }
}

pub struct SummitShaper(GoldenMajesty);

impl SummitShaper {
    pub fn new() -> Self {
        Self(GoldenMajesty::new())
    }
}

impl WeaponAbility for SummitShaper {
    fn record(&self) -> WeaponRecord {
        self.0.record().name("Summit shaper").type_(Sword)
    }
}

impl SpecialAbility for SummitShaper {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[FieldEnergy], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.0.update(guard, timers, attack, particles, data, enemy, time);
    }

    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, timers, data, enemy);
    }

    fn reset(&mut self) -> () { self.0.reset() }
}

pub struct VortexVanquisher(GoldenMajesty);

impl VortexVanquisher {
    pub fn new() -> Self {
        Self(GoldenMajesty::new())
    }
}

impl WeaponAbility for VortexVanquisher {
    fn record(&self) -> WeaponRecord {
        self.0.record().name("Vortex Vanquisher").type_(Polearm)
    }
}

impl SpecialAbility for VortexVanquisher {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[FieldEnergy], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.0.update(guard, timers, attack, particles, data, enemy, time);
    }

    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, timers, data, enemy);
    }

    fn reset(&mut self) -> () { self.0.reset() }
}

pub struct MemoryOfDust(GoldenMajesty);

impl MemoryOfDust {
    pub fn new() -> Self {
        Self(GoldenMajesty::new())
    }
}

impl WeaponAbility for MemoryOfDust {
    fn record(&self) -> WeaponRecord {
        self.0.record().name("Memory of Dust").type_(Catalyst)
    }
}

impl SpecialAbility for MemoryOfDust {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[FieldEnergy], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.0.update(guard, timers, attack, particles, data, enemy, time);
    }

    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, timers, data, enemy);
    }

    fn reset(&mut self) -> () { self.0.reset() }
}
