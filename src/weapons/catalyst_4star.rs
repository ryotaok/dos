use std::ptr;

use crate::state::State;
use crate::types::{AttackType, WeaponType, FieldEnergy, VecFieldEnergy, Particle, GAUGE1A};
use crate::fc::{FieldCharacterIndex, SpecialAbility, WeaponAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{Attack, ElementalAttack, FullCharacterTimers, TimerGuard, EffectTimer, HitsTimer, StackTimer, DurationTimer};
use crate::testutil;

use AttackType::*;
use WeaponType::*;
// use Vision::*;

// version 1.0

pub struct PrototypeAmberR5;

impl SpecialAbility for PrototypeAmberR5 {}

impl WeaponAbility for PrototypeAmberR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Prototype Amber").type_(Catalyst).version(1.0)
            .base_atk(510.0)
            .hp(41.3)
    }
}

pub struct MappaMareR5 {
    timer: StackTimer,
}

impl MappaMareR5 {
    pub fn new() -> Self {
        Self {
            timer: StackTimer::new(0.0, 10.0, 2),
        }
    }
}

impl WeaponAbility for MappaMareR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Mappa Mare").type_(Catalyst).version(1.0)
            .base_atk(565.0)
            .em(110.0)
    }
}

impl SpecialAbility for MappaMareR5 {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, attack: &[ElementalAttack], _particles: &[FieldEnergy], _data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        let should_update = attack.iter().any(|a| enemy.trigger_er(&a.element).is_triggered());
        self.timer.update(guard.second(should_update), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            modifiable_state[data.idx.0].elemental_dmg += 16.0 * self.timer.n as f32;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct SolarPearlR5 {
    na_timer: DurationTimer,
    skill_timer: DurationTimer,
}

impl SolarPearlR5 {
    pub fn new() -> Self {
        Self {
            na_timer: DurationTimer::new(0.0, 6.0),
            skill_timer: DurationTimer::new(0.0, 6.0),
        }
    }
}

impl WeaponAbility for SolarPearlR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Solar Pearl").type_(Catalyst).version(1.0)
            .base_atk(510.0)
            .cr(27.6)
    }
}

impl SpecialAbility for SolarPearlR5 {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[FieldEnergy], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let mut should_update = guard.kind == Na;
        self.na_timer.update(guard.second(should_update), time);
        should_update = guard.kind == Burst || guard.kind == PressSkill || guard.kind == HoldSkill;
        self.skill_timer.update(guard.second(should_update), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        let mut state = &mut modifiable_state[data.idx.0];
        if self.na_timer.is_active() {
            state.skill_dmg += 40.0;
            state.burst_dmg += 40.0;
        }
        if self.skill_timer.is_active() {
            state.na_dmg += 40.0;
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

impl WeaponAbility for BlackcliffAgateR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Blackcliff Agate").type_(Catalyst).version(1.0)
            .base_atk(510.0)
            .atk(24.0).cd(55.1)
    }
}

pub struct RoyalGrimoireR5;

impl SpecialAbility for RoyalGrimoireR5 {}

impl WeaponAbility for RoyalGrimoireR5 {
    fn record(&self) -> WeaponRecord {
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
    pub fn new() -> Self {
        Self {
            timer: DurationTimer::new(20.0, 10.0),
        }
    }
}

impl WeaponAbility for ThrillingTalesOfDragonSlayersR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Thrilling Tales of Dragon Slayers").type_(Catalyst).version(1.0)
            .base_atk(401.0)
            .hp(35.2)
    }
}

impl SpecialAbility for ThrillingTalesOfDragonSlayersR5 {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[FieldEnergy], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(guard.second(true), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, _data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            // always buff the first member
            modifiable_state[0].atk += 48.0;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct EyeOfPerceptionR5 {
    timer: HitsTimer,
    aa: Attack,
}

impl EyeOfPerceptionR5 {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            timer: HitsTimer::new(8.0, 1),
            aa: Attack {
                kind: AdditionalAttack,
                gauge: &GAUGE1A,
                multiplier: 360.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            }
        }
    }
}

impl WeaponAbility for EyeOfPerceptionR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Eye of Perception").type_(Catalyst).version(1.0)
            .base_atk(454.0)
            .atk(55.1)
    }
}

impl SpecialAbility for EyeOfPerceptionR5 {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[FieldEnergy], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let should_update = timers.na_timer().is_active() || timers.ca_timer().is_active();
        self.timer.update(guard.second(testutil::chance() < 0.5 && should_update), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, _particles: &mut Vec<FieldEnergy>, _timers: &FullCharacterTimers, _data: &CharacterData, _enemy: &Enemy) -> () {
        if self.timer.is_active() {
            atk_queue.push(ElementalAttack::physical(&self.aa))
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct TheWidsithR5 {
    timer: DurationTimer,
    random_theme_song: usize,
}

impl TheWidsithR5 {
    pub fn new() -> Self {
        Self {
            timer: DurationTimer::new(30.0, 10.0), random_theme_song: 0,
        }
    }
}

impl WeaponAbility for TheWidsithR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("The Widsith").type_(Catalyst).version(1.0)
            .base_atk(510.0)
            .cd(55.1)
    }
}

impl SpecialAbility for TheWidsithR5 {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[FieldEnergy], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let before = self.timer.is_active();
        self.timer.update(guard.second(true), time);
        let after = self.timer.is_active();
        // check if the first time to gain the theme
        if !before && after {
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

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            match self.random_theme_song {
                0 => modifiable_state[data.idx.0].atk += 120.0,
                1 => modifiable_state[data.idx.0].all_dmg += 96.0,
                2 => modifiable_state[data.idx.0].em += 480.0,
                _ => (),
            };
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}
