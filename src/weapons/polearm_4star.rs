use std::ptr;

use crate::state::State;
use crate::types::{AttackType, WeaponType, FieldEnergy, VecFieldEnergy, Particle, Vision, GAUGE1A};
use crate::fc::{FieldCharacterIndex, SpecialAbility, WeaponAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{Attack, ElementalAttack, FullCharacterTimers, TimerGuard, EffectTimer, StackTimer, DurationTimer};
// use crate::testutil;

use AttackType::*;
use WeaponType::*;
// use Vision::*;


// version 1.0

pub struct PrototypeStarglitterR5 {
    timer: StackTimer,
}

impl PrototypeStarglitterR5 {
    pub fn new() -> Self {
        Self {
            timer: StackTimer::new(0.0, 12.0, 2),
        }
    }
}

impl WeaponAbility for PrototypeStarglitterR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Prototype Starglitter").type_(Polearm).version(1.0)
            .base_atk(510.0)
            .er(45.9)
    }
}

impl SpecialAbility for PrototypeStarglitterR5 {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[FieldEnergy], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let should_update = guard.kind == PressSkill || guard.kind == HoldSkill;
        self.timer.update(guard.second(should_update), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            let mut state = &mut modifiable_state[data.idx.0];
            state.na_dmg += 16.0 * self.timer.n as f32;
            state.ca_dmg += 16.0 * self.timer.n as f32;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct CrescentPikeR5 {
    timer: DurationTimer,
    aa: Attack,
}

impl CrescentPikeR5 {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            timer: DurationTimer::new(0.0, 5.0),
            aa: Attack {
                kind: AdditionalAttack,
                gauge: &GAUGE1A,
                multiplier: 40.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            }
        }
    }
}

impl WeaponAbility for CrescentPikeR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Crescent Pike").type_(Polearm).version(1.0)
            .base_atk(566.0)
            .dmg_phy(34.5)
    }
}

impl SpecialAbility for CrescentPikeR5 {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], particles: &[FieldEnergy], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let should_update = particles.has_particles();
        self.timer.update(guard.second(should_update), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, _particles: &mut Vec<FieldEnergy>, timers: &FullCharacterTimers, _data: &CharacterData, _enemy: &Enemy) -> () {
        let did_attack = timers.na_timer().is_active() || timers.ca_timer().is_active();
        if self.timer.is_active() && did_attack {
            atk_queue.push(ElementalAttack::physical(&self.aa));
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct DeathmatchR5;

impl SpecialAbility for DeathmatchR5 {}

impl WeaponAbility for DeathmatchR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Deathmatch").type_(Polearm).version(1.0)
            .base_atk(454.0)
            .atk(48.0).cr(36.8)
    }
}

// one stack is always active
pub struct BlackcliffPoleR5;

impl SpecialAbility for BlackcliffPoleR5 {}

impl WeaponAbility for BlackcliffPoleR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Blackcliff Pole").type_(Polearm).version(1.0)
            .base_atk(510.0)
            .atk(24.0).cd(55.1)
    }
}

pub struct RoyalSpearR5;

impl SpecialAbility for RoyalSpearR5 {}

impl WeaponAbility for RoyalSpearR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Royal Spear").type_(Polearm).version(1.0)
            .base_atk(565.0)
            .atk(27.6)
    }
}

pub struct WhiteTasselR5;

impl SpecialAbility for WhiteTasselR5 {}

impl WeaponAbility for WhiteTasselR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("White Tassel").type_(Polearm).version(1.0)
            .base_atk(401.0)
            .cr(23.4)
            .dmg_na(48.0)
    }
}

pub struct DragonsBaneR5;

impl WeaponAbility for DragonsBaneR5 {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Dragon's Bane").type_(Polearm).version(1.0)
            .base_atk(454.0)
            .em(221.0)
    }
}

impl SpecialAbility for DragonsBaneR5 {
    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        match &enemy.aura.aura {
            Vision::Hydro |
            Vision::Pyro => modifiable_state[data.idx.0].all_dmg += 36.0,
            _ => (),
        }
    }
}
