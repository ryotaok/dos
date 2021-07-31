use std::ptr;

use crate::state::State;
use crate::types::{AttackType, WeaponType, FieldEnergy, VecFieldEnergy, Particle, GAUGE1A};
use crate::fc::{FieldCharacterIndex, SpecialAbility, WeaponAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{Attack, ElementalAttack, FullCharacterTimers, TimerGuard, EffectTimer, HitsTimer, StackTimer};
use crate::testutil;

use AttackType::*;
use WeaponType::*;
// use Vision::*;

// version 1.0

pub struct PrototypeCrescentR5;

impl SpecialAbility for PrototypeCrescentR5 {}

impl WeaponAbility for PrototypeCrescentR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Prototype Crescent").type_(Bow).version(1.0)
            .base_atk(510.0)
            .atk(41.3 + 72.0)
    }
}

pub struct CompoundBowR5 {
    timer: StackTimer,
}

impl CompoundBowR5 {
    pub fn new() -> Self {
        Self {
            timer: StackTimer::new(0.3, 6.0, 4),
        }
    }
}

impl WeaponAbility for CompoundBowR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Compound Bow").type_(Bow).version(1.0)
            .base_atk(454.0)
            .dmg_phy(69.0)
    }
}

impl SpecialAbility for CompoundBowR5 {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[FieldEnergy], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let should_update = timers.na_timer().is_active() || timers.ca_timer().is_active();
        self.timer.update(guard.second(should_update), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            let mut state = &mut modifiable_state[data.idx.0];
            state.atk     += 8.0 * self.timer.n as f32;
            state.atk_spd += 2.4 * self.timer.n as f32;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct TheViridescentHuntR5 {
    timer: HitsTimer,
    aa: Attack,
}

impl TheViridescentHuntR5 {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            timer: HitsTimer::new(10.0, 8),
            aa: Attack {
                kind: AdditionalAttack,
                gauge: &GAUGE1A,
                multiplier: 80.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            }
        }
    }
}

impl WeaponAbility for TheViridescentHuntR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("The Viridescent Hunt").type_(Bow).version(1.0)
            .base_atk(510.0)
            .cr(27.6)
    }
}

impl SpecialAbility for TheViridescentHuntR5 {
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

// one stack is always active
pub struct BlackcliffWarbowR5;

impl SpecialAbility for BlackcliffWarbowR5 {}

impl WeaponAbility for BlackcliffWarbowR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Blackcliff Warbow").type_(Bow).version(1.0)
            .base_atk(510.0)
            .atk(24.0).cd(55.1)
    }
}

pub struct RoyalBowR5;

impl SpecialAbility for RoyalBowR5 {}

impl WeaponAbility for RoyalBowR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Royal Bow").type_(Bow).version(1.0)
            .base_atk(510.0)
            .atk(41.3)
    }
}

pub struct SlingshotR5;

impl SpecialAbility for SlingshotR5 {}

impl WeaponAbility for SlingshotR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Slingshot").type_(Bow).version(1.0)
            .base_atk(354.0)
            .cr(31.2)
            .dmg_na(60.0).dmg_ca(60.0)
    }
}

pub struct RustR5;

impl SpecialAbility for RustR5 {}

impl WeaponAbility for RustR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Rust").type_(Bow).version(1.0)
            .base_atk(510.0)
            .atk(41.3)
            .dmg_na(80.0).dmg_ca(-10.0)
    }
}

pub struct TheStringlessR5;

impl SpecialAbility for TheStringlessR5 {}

impl WeaponAbility for TheStringlessR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("The Stringless").type_(Bow).version(1.0)
            .base_atk(510.0)
            .em(165.0)
            .dmg_skill(48.0).dmg_burst(48.0)
    }
}
