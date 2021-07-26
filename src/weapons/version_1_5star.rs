use std::ptr;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Particle, GAUGE1A};
use crate::fc::{FieldCharacterIndex, SpecialAbility, WeaponAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{Attack, ElementalAttack, TimerGuard, FullCharacterTimers, EffectTimer, DurationTimer, HitsTimer, DotTimer, StackTimer};
use crate::testutil;

use AttackType::*;
use WeaponType::*;
// use Vision::*;

// version 1.0

pub struct SkywardBlade {
    burst_timer: DurationTimer,
    na: HitsTimer,
    na_or_ca: bool,
    aa: Attack,
}

impl SkywardBlade {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            burst_timer: DurationTimer::new(0.0, 12.0),
            na: HitsTimer::new(0.0, 1),
            na_or_ca: false,
            aa: Attack {
                kind: AdditionalAttack,
                gauge: &GAUGE1A,
                multiplier: 20.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl WeaponAbility for SkywardBlade {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Skyward Blade").type_(Sword).version(1.0)
            .base_atk(608.0)
            .cr(4.0).er(55.1)
    }
}

impl SpecialAbility for SkywardBlade {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        self.na_or_ca = guard.kind == Na || guard.kind == Ca;
        let burst = guard.kind == Burst;
        self.burst_timer.update(guard.second(burst), time);
        // self.na.update(guard.second(na_or_ca), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, _particles: &mut Vec<Particle>, _timers: &FullCharacterTimers, _data: &CharacterData, _enemy: &Enemy) -> () {
        if self.burst_timer.is_active() && self.na_or_ca {
            atk_queue.push(ElementalAttack::physical(&self.aa))
        }
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.burst_timer.is_active() {
            modifiable_state[data.idx.0].atk_spd += 10.0;
        }
    }

    fn reset(&mut self) -> () {
        self.burst_timer.reset();
        self.na.reset();
    }
}

pub struct AquilaFavonia {
    na: HitsTimer,
    aa: Attack,
}

impl AquilaFavonia {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            na: HitsTimer::new(15.0, 1),
            aa: Attack {
                kind: AdditionalAttack,
                gauge: &GAUGE1A,
                multiplier: 200.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl WeaponAbility for AquilaFavonia {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Aquila Favonia").type_(Sword).version(1.0)
            .base_atk(674.0)
            .atk(20.0)
            .dmg_phy(41.3)
    }
}

impl SpecialAbility for AquilaFavonia {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let should_update = guard.kind == Na || guard.kind == Ca;
        self.na.update(guard.second(should_update), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, _particles: &mut Vec<Particle>, _timers: &FullCharacterTimers, _data: &CharacterData, _enemy: &Enemy) -> () {
        if self.na.is_active() {
            atk_queue.push(ElementalAttack::physical(&self.aa))
        }
    }

    fn reset(&mut self) -> () {
        self.na.reset();
    }
}

pub struct SkywardPride {
    burst_timer: DurationTimer,
    na: HitsTimer,
    aa: Attack,
}

impl SkywardPride {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            burst_timer: DurationTimer::new(0.0, 20.0),
            na: HitsTimer::new(20.0, 8),
            aa: Attack {
                kind: AdditionalAttack,
                gauge: &GAUGE1A,
                multiplier: 80.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl WeaponAbility for SkywardPride {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Skyward Pride").type_(Claymore).version(1.0)
            .base_atk(674.0)
            .er(36.8)
            .dmg_na(8.0).dmg_ca(8.0).dmg_skill(8.0).dmg_burst(8.0)
    }
}

impl SpecialAbility for SkywardPride {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        self.burst_timer.update(guard.second(guard.kind == Burst), time);
        self.na.update(guard.second(guard.kind == Na || guard.kind == Ca), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, _particles: &mut Vec<Particle>, _timers: &FullCharacterTimers, _data: &CharacterData, _enemy: &Enemy) -> () {
        if self.burst_timer.is_active() && self.na.is_active() {
            atk_queue.push(ElementalAttack::physical(&self.aa))
        }
    }

    fn reset(&mut self) -> () {
        self.burst_timer.reset();
        self.na.reset();
    }
}

pub struct WolfsGravestone;

impl SpecialAbility for WolfsGravestone {}

impl WeaponAbility for WolfsGravestone {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Wolf's Gravestone").type_(Claymore).version(1.0)
            .base_atk(608.0)
            .atk(49.6 + 20.0)
    }

    // TODO Box::new(FixedStack::new(MovementActivator, 30.0, 12.0, Atk(40.0)))
}

pub struct SkywardSpine {
    timer: HitsTimer,
    aa: Attack,
}

impl SkywardSpine {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            timer: HitsTimer::new(2.0, 1),
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

impl WeaponAbility for SkywardSpine {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Skyward Spine").type_(Polearm).version(1.0)
            .base_atk(674.0)
            .cr(8.0).er(36.8).atk_spd(12.0)
    }
}

impl SpecialAbility for SkywardSpine {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let should_update = guard.kind == Na || guard.kind == Ca;
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

pub struct PrimordialJadeWingedSpear {
    timer: StackTimer,
}

impl PrimordialJadeWingedSpear {
    pub fn new() -> Self {
        Self { timer: StackTimer::new(0.3, 6.0, 7) }
    }
}

impl WeaponAbility for PrimordialJadeWingedSpear {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Primordial Jade Winged-Spear").type_(Polearm).version(1.0)
            .base_atk(674.0)
            .cr(22.1)
    }
}

impl SpecialAbility for PrimordialJadeWingedSpear {
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

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            let mut state = &mut modifiable_state[data.idx.0];
            state.atk += 3.2 * self.timer.n as f32;
            state.all_dmg += if self.timer.n == 7 { 12.0 } else { 0.0 };
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct SkywardHarp {
    timer: HitsTimer,
    aa: Attack,
}

impl SkywardHarp {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            timer: HitsTimer::new(4.0, 1),
            aa: Attack {
                kind: AdditionalAttack,
                gauge: &GAUGE1A,
                multiplier: 125.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            }
        }
    }
}

impl WeaponAbility for SkywardHarp {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Skyward Harp").type_(Bow).version(1.0)
            .base_atk(674.0)
            .cr(22.1).cd(20.0)
    }
}

impl SpecialAbility for SkywardHarp {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let should_update = unsafe {
            attack.iter().any(|&a|
                match (*a.atk).kind {
                    Na | Ca | PressSkill | HoldSkill | SkillDot | Burst | BurstDot => true,
                    _ => false,
                }
            )
        };
        self.timer.update(guard.second(testutil::chance() < 0.6 && should_update), time);
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

pub struct AmosBow;

impl SpecialAbility for AmosBow {}

impl WeaponAbility for AmosBow {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Amos' Bow").type_(Bow).version(1.0)
            .base_atk(608.0)
            .atk(49.6)
            .dmg_na(12.0 + 40.0).dmg_ca(12.0 + 40.0)
    }
}

pub struct SkywardAtlas {
    timer: DotTimer,
    aa: Attack,
}

impl SkywardAtlas {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            timer: DotTimer::new(30.0, 2.0, 8),
            aa: Attack {
                kind: AdditionalAttack,
                gauge: &GAUGE1A,
                multiplier: 160.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            }
        }
    }
}

impl WeaponAbility for SkywardAtlas {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Skyward Atlas").type_(Catalyst).version(1.0)
            .base_atk(674.0)
            .atk(33.1)
            .dmg_pyro(12.0).dmg_cryo(12.0).dmg_hydro(12.0).dmg_electro(12.0).dmg_anemo(12.0).dmg_geo(12.0).dmg_dendro(12.0)
    }
}

impl SpecialAbility for SkywardAtlas {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let should_update = guard.kind == Na;
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

pub struct LostPrayerToTheSacredWinds {
    timer: StackTimer,
}

impl LostPrayerToTheSacredWinds {
    pub fn new() -> Self {
        Self { timer: StackTimer::new(4.0, 8.0, 4) }
    }
}

impl WeaponAbility for LostPrayerToTheSacredWinds {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Lost Prayer to the Sacred Winds").type_(Catalyst).version(1.0)
            .base_atk(608.0)
            .cr(33.1)
    }
}

impl SpecialAbility for LostPrayerToTheSacredWinds {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        // only the attacker can activate the passive
        self.timer.update(guard.second(data.idx.0 == 0), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            modifiable_state[data.idx.0].elemental_dmg += 8.0 * self.timer.n as f32;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}
