use std::ptr;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Particle, Vision, GAUGE1A};
use crate::fc::{FieldCharacterIndex, SpecialAbility, WeaponAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{Attack, ElementalAttack, FullCharacterTimers, TimerGuard, EffectTimer, HitsTimer, StackTimer};
use crate::testutil;

use AttackType::*;
use WeaponType::*;
// use Vision::*;

// version 1.0

pub struct PrototypeArchaicR5 {
    timer: HitsTimer,
    aa: Attack,
}

impl PrototypeArchaicR5 {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            timer: HitsTimer::new(15.0, 1),
            aa: Attack {
                kind: AdditionalAttack,
                gauge: &GAUGE1A,
                multiplier: 480.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            }
        }
    }
}

impl WeaponAbility for PrototypeArchaicR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Prototype Archaic R5").type_(Claymore).version(1.0)
            .base_atk(566.0)
            .atk(27.6)
    }
}

impl SpecialAbility for PrototypeArchaicR5 {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let should_update = timers.na_timer().is_active() || timers.ca_timer().is_active();
        self.timer.update(guard.second(testutil::chance() < 0.5 && should_update), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, _particles: &mut Vec<Particle>, _timers: &FullCharacterTimers, _data: &CharacterData, _enemy: &Enemy) -> () {
        if self.timer.is_active() {
            atk_queue.push(ElementalAttack::physical(&self.aa))
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct WhiteblindR5 {
    timer: StackTimer,
}

impl WhiteblindR5 {
    pub fn new() -> Self {
        Self { timer: StackTimer::new(0.5, 6.0, 4) }
    }
}

impl WeaponAbility for WhiteblindR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Whiteblind R5").type_(Claymore).version(1.0)
            .base_atk(510.0)
            .def(51.7)
    }
}

impl SpecialAbility for WhiteblindR5 {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let should_update = timers.na_timer().is_active() || timers.ca_timer().is_active();
        self.timer.update(guard.second(should_update), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            let mut state = &mut modifiable_state[data.idx.0];
            state.atk += 12.0 * self.timer.n as f32;
            state.def += 12.0 * self.timer.n as f32;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct SerpentSpineR5 {
    timer: StackTimer,
}

impl SerpentSpineR5 {
    pub fn new() -> Self {
        Self { timer: StackTimer::new(4.0, 8.0, 5) }
    }
}

impl WeaponAbility for SerpentSpineR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Serpent Spine R5").type_(Claymore).version(1.0)
            .base_atk(510.0)
            .cr(27.6)
    }
}

impl SpecialAbility for SerpentSpineR5 {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        // only the attacker can activate the passive
        self.timer.update(guard.second(data.idx.0 == 0), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            modifiable_state[data.idx.0].all_dmg += 10.0 * self.timer.n as f32;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

// one stack is always active
pub struct BlackcliffSlasherR5;

impl SpecialAbility for BlackcliffSlasherR5 {}

impl WeaponAbility for BlackcliffSlasherR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Blackcliff Slasher R5").type_(Claymore).version(1.0)
            .base_atk(510.0)
            .atk(24.0).cd(55.1)
    }
}

pub struct RoyalGreatswordR5;

impl SpecialAbility for RoyalGreatswordR5 {}

impl WeaponAbility for RoyalGreatswordR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Royal Greatsword R5").type_(Claymore).version(1.0)
            .base_atk(565.0)
            .atk(27.6)
    }
}

pub struct RainslasherR5;

impl WeaponAbility for RainslasherR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Rainslasher R5").type_(Claymore).version(1.0)
            .base_atk(510.0)
            .em(165.0)
    }
}

impl SpecialAbility for RainslasherR5 {
    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        match &enemy.aura.aura {
            Vision::Electro |
            Vision::Hydro => modifiable_state[data.idx.0].all_dmg += 36.0,
            _ => (),
        }
    }
}

// The Bell
