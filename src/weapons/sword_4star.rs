use std::ptr;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Particle, Vision, GAUGE1A};
use crate::fc::{FieldCharacterIndex, SpecialAbility, WeaponAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{Attack, ElementalAttack, FullCharacterTimers, TimerGuard, EffectTimer, StackTimer, SigilTimer};

use AttackType::*;
use WeaponType::*;
// use Vision::*;

// version 1.0

pub struct PrototypeRancourR5 {
    timer: StackTimer,
}

impl PrototypeRancourR5 {
    pub fn new() -> Self {
        Self {
            timer: StackTimer::new(0.3, 6.0, 4),
        }
    }
}

impl WeaponAbility for PrototypeRancourR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Prototype Rancour R5").type_(Sword).version(1.0)
            .base_atk(566.0)
            .dmg_phy(34.5)
    }
}

impl SpecialAbility for PrototypeRancourR5 {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        // let should_update = timers.na_timer().is_active() || timers.ca_timer().is_active();
        let should_update = guard.kind == Na || guard.kind == Ca;
        self.timer.update(guard.second(should_update), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            let mut state = &mut modifiable_state[data.idx.0];
            state.atk += 8.0 * self.timer.n as f32;
            state.def += 8.0 * self.timer.n as f32;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

// iron sting

pub struct TheBlackSwordR5;

impl SpecialAbility for TheBlackSwordR5 {}
impl WeaponAbility for TheBlackSwordR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("The Black Sword R5").type_(Sword).version(1.0)
            .base_atk(510.0)
            .cr(27.6)
            .dmg_na(40.0).dmg_ca(40.0)
    }
}

// one stack is always active
pub struct BlackcliffLongswordR5;

impl SpecialAbility for BlackcliffLongswordR5 {}
impl WeaponAbility for BlackcliffLongswordR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Blackcliff Longsword R5").type_(Sword).version(1.0)
            .base_atk(565.0)
            .atk(24.0).cd(36.8)
    }
}

pub struct RoyalLongswordR5;

impl SpecialAbility for RoyalLongswordR5 {}
impl WeaponAbility for RoyalLongswordR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Royal Longsword R5").type_(Sword).version(1.0)
            .base_atk(565.0)
            .atk(27.6).cr(0.0)
    }
}

// the passive is always active
pub struct HarbingerOfDawnR5;

impl SpecialAbility for HarbingerOfDawnR5 {}
impl WeaponAbility for HarbingerOfDawnR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Harbinger of Dawn R5").type_(Sword).version(1.0)
            .base_atk(401.0)
            .cr(28.0).cd(46.9)
    }
}

pub struct TheFluteR5 {
    timer: SigilTimer,
    aa: Attack,
}

impl TheFluteR5 {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            timer: SigilTimer::new(0.5, 0.5, 0.0, 5),
            aa: Attack {
                kind: AdditionalAttack,
                gauge: &GAUGE1A,
                multiplier: 200.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            }
        }
    }
}

impl WeaponAbility for TheFluteR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("The Flute R5").type_(Sword).version(1.0)
            .base_atk(510.0)
            .atk(41.3)
    }
}

impl SpecialAbility for TheFluteR5 {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let should_update = guard.kind == Na || guard.kind == Ca;
        self.timer.update(guard.second(should_update), time);
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

pub struct LionsRoarR5;

impl WeaponAbility for LionsRoarR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Lion's Roar R5").type_(Sword).version(1.0)
            .base_atk(510.0)
            .atk(41.3)
    }
}

impl SpecialAbility for LionsRoarR5 {
    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        match &enemy.aura.aura {
            Vision::Electro |
            Vision::Pyro => modifiable_state[data.idx.0].all_dmg += 36.0,
            _ => (),
        }
    }
}
