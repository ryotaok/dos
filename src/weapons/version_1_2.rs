use std::ptr;
use crate::state::State;
use crate::types::{AttackType, WeaponType, FieldEnergy, VecFieldEnergy, Particle, Vision, GAUGE1A};
use crate::fc::{FieldCharacterIndex, SpecialAbility, WeaponAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{Attack, ElementalAttack, FullCharacterTimers, TimerGuard, EffectTimer, HitsTimer};

use AttackType::*;
use WeaponType::*;
use Vision::*;

pub struct FesteringDesire;

impl WeaponAbility for FesteringDesire {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Festering Desire").type_(Sword).version(1.2)
            .base_atk(510.0)
            .er(45.9).dmg_skill(32.0)
    }
}

impl SpecialAbility for FesteringDesire {
    fn intensify(&self, attack: &Attack) -> Option<State> {
        match &attack.kind {
            PressSkill |
            HoldSkill |
            SkillDot => Some(State::new().cr(12.0)),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct FrostBurial {
    timer: HitsTimer,
    aa: Attack,
    aa_cryo: Attack,
}

impl FrostBurial {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            timer: HitsTimer::new(10.0, 1),
            aa: Attack {
                kind: AdditionalAttack,
                gauge: &GAUGE1A,
                multiplier: 140.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            aa_cryo: Attack {
                kind: AdditionalAttack,
                gauge: &GAUGE1A,
                multiplier: 360.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl SpecialAbility for FrostBurial {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[FieldEnergy], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let should_update = guard.kind == Na || guard.kind == Ca;
        self.timer.update(guard.second(should_update), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, _particles: &mut Vec<FieldEnergy>, _timers: &FullCharacterTimers, _data: &CharacterData, enemy: &Enemy) -> () {
        if self.timer.is_active() {
            if enemy.aura.aura == Cryo {
                atk_queue.push(ElementalAttack::physical(&self.aa_cryo));
            } else {
                atk_queue.push(ElementalAttack::physical(&self.aa));
            }
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct SnowTombedStarsilver(FrostBurial);

impl SnowTombedStarsilver {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self(FrostBurial::new(idx))
    }
}

impl WeaponAbility for SnowTombedStarsilver {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Snow-Tombed Starsilver").type_(Claymore).version(1.2)
            .base_atk(565.0)
            .dmg_phy(34.5)
    }
}

impl SpecialAbility for SnowTombedStarsilver {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[FieldEnergy], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.0.update(guard, timers, attack, particles, data, enemy, time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<FieldEnergy>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        self.0.additional_attack(atk_queue, particles, timers, data, enemy);
    }

    fn reset(&mut self) -> () { self.0.reset() }
}

pub struct DragonspineSpear(FrostBurial);

impl DragonspineSpear {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self(FrostBurial::new(idx))
    }
}

impl WeaponAbility for DragonspineSpear {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Dragonspine Spear").type_(Polearm).version(1.2)
            .base_atk(454.0)
            .dmg_phy(69.0)
    }
}

impl SpecialAbility for DragonspineSpear {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[FieldEnergy], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.0.update(guard, timers, attack, particles, data, enemy, time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<FieldEnergy>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        self.0.additional_attack(atk_queue, particles, timers, data, enemy);
    }

    fn reset(&mut self) -> () { self.0.reset() }
}

pub struct Frostbearer(FrostBurial);

impl Frostbearer {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self(FrostBurial::new(idx))
    }
}

impl WeaponAbility for Frostbearer {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Frostbearer").type_(Catalyst).version(1.2)
            .base_atk(510.0)
            .atk(41.3)
    }
}

impl SpecialAbility for Frostbearer {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[FieldEnergy], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.0.update(guard, timers, attack, particles, data, enemy, time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<FieldEnergy>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        self.0.additional_attack(atk_queue, particles, timers, data, enemy);
    }

    fn reset(&mut self) -> () { self.0.reset() }
}
