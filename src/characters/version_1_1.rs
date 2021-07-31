use std::ptr;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Vision, FieldEnergy, VecFieldEnergy, Particle, GAUGE1A, GAUGE2B, GAUGE4C};
use crate::fc::{FieldCharacterIndex, SpecialAbility, CharacterAbility, CharacterData, CharacterRecord, Enemy};
use crate::action::{Attack, ElementalAttack, ElementalAttackVector, FullCharacterTimers, CharacterTimersBuilder, TimerGuard, EffectTimer, DurationTimer, HitsTimer, DotTimer, LoopTimer};

use AttackType::*;
use WeaponType::*;
use Vision::*;

// version 1.1

pub struct Tartaglia {
    skill_aa: HitsTimer,
    skill_timer: DurationTimer,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    na_5: Attack,
    na_6: Attack,
    press: Attack,
    riptide_slash: Attack,
    burst: Attack,
}

impl Tartaglia {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            skill_aa: HitsTimer::new(1.5, 1),
            skill_timer: DurationTimer::new(45.0, 30.0),
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 76.84,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 82.28,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 111.35,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 118.49,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_5: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 109.31,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_6: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: (70.04+74.46) / 2.0,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE1A,
                multiplier: 122.4,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            riptide_slash: Attack {
                kind: AttackType::SkillDot,
                gauge: &GAUGE1A,
                multiplier: 119.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE2B,
                multiplier: (835.2 + 216.0) / 2.0,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Tartaglia {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Tartaglia").vision(Hydro).weapon(Bow).release_date("2020-11-11").version(1.1)
            .base_hp(13103.0).base_atk(301.0).base_def(815.0)
            .dmg_hydro(28.8)
            .energy_cost(60.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.416, 6))
            .press(DotTimer::single_hit(45.0))
            .burst(DotTimer::single_hit(15.0))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na_1.icd_timer = &mut timers.na_icd;
        self.na_2.icd_timer = &mut timers.na_icd;
        self.na_3.icd_timer = &mut timers.na_icd;
        self.na_4.icd_timer = &mut timers.na_icd;
        self.na_5.icd_timer = &mut timers.na_icd;
        self.na_6.icd_timer = &mut timers.na_icd;
        self.press.icd_timer = &mut timers.skill_icd;
        self.riptide_slash.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
    }
}

impl SpecialAbility for Tartaglia {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[FieldEnergy], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        self.skill_timer.update(guard.check_second(PressSkill), time);
        self.skill_aa.update(guard.check_second(Na), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<FieldEnergy>, timers: &FullCharacterTimers, data: &CharacterData, _enemy: &Enemy) -> () {
        if timers.burst_timer().is_active() {
            atk_queue.push(ElementalAttack::hydro(&self.burst));
        }
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::hydro(&self.press));
            particles.push_p(Particle::new(Hydro, 10.0));
        }
        if self.skill_aa.is_active() {
            atk_queue.push(ElementalAttack::hydro(&self.riptide_slash));
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push_hydro(data, &self.na_1),
                2 => atk_queue.push_hydro(data, &self.na_2),
                3 => atk_queue.push_hydro(data, &self.na_3),
                4 => atk_queue.push_hydro(data, &self.na_4),
                5 => atk_queue.push_hydro(data, &self.na_5),
                6 => atk_queue.push_hydro(data, &self.na_6),
                _ => (),
            };
        }
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        // Master of Weaponry
        for s in modifiable_state.iter_mut() {
            s.na_talent += 5.0;
        }
        let state = &mut modifiable_state[data.idx.0];
        if self.skill_timer.is_active() {
            state.infusion = true;
        }
    }

    fn reset(&mut self) -> () {
        self.skill_timer.reset();
        self.skill_aa.reset();
    }
}

pub struct Diona {
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    na_5: Attack,
    press: Attack, // hold
    burst: Attack,
    burst_dot: Attack,
}

impl Diona {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 71.4,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 66.3,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 90.1,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 85.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_5: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 106.25,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE1A,
                multiplier: 75.46,
                hits: 5,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE1A,
                multiplier: 144.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_dot: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 94.75,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Diona {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Diona").vision(Cryo).weapon(Bow).release_date("2020-11-11").version(1.1)
            .base_hp(9570.0).base_atk(212.0).base_def(601.0)
            .dmg_cryo(24.0)
            .energy_cost(80.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.233, 5))
            .press(DotTimer::single_hit(15.0))
            .burst(DotTimer::new(20.0, 2.0, 6))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na_1.icd_timer = &mut timers.na_icd;
        self.na_2.icd_timer = &mut timers.na_icd;
        self.na_3.icd_timer = &mut timers.na_icd;
        self.na_4.icd_timer = &mut timers.na_icd;
        self.na_5.icd_timer = &mut timers.na_icd;
        self.press.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
        self.burst_dot.icd_timer = &mut timers.burst_icd;
    }
}

impl SpecialAbility for Diona {
    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<FieldEnergy>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        let burst = timers.burst_timer();
        if burst.is_active() {
            if burst.n() == 1 {
                atk_queue.push(ElementalAttack::cryo(&self.burst));
                atk_queue.push(ElementalAttack::cryo(&self.burst_dot));
            } else {
                atk_queue.push(ElementalAttack::cryo(&self.burst_dot));
            }
        }
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::cryo(&self.press));
            particles.push_p(Particle::new(Cryo, 4.5));
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push_cryo(data, &self.na_1),
                2 => atk_queue.push_cryo(data, &self.na_2),
                3 => atk_queue.push_cryo(data, &self.na_3),
                4 => atk_queue.push_cryo(data, &self.na_4),
                5 => atk_queue.push_cryo(data, &self.na_5),
                _ => (),
            };
        }
    }
}

pub struct Zhongli {
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    na_5: Attack,
    na_6: Attack,
    press: Attack,
    press_dot: Attack,
    burst: Attack,
}

impl Zhongli {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 60.82,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 61.58,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 76.26,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 84.88,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_5: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: (21.25*4.0) / 4.0,
                hits: 4,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_6: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 107.73,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE2B,
                multiplier: 28.8,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press_dot: Attack {
                kind: AttackType::SkillDot,
                gauge: &GAUGE1A,
                multiplier: 57.6,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE4C,
                multiplier: 899.72,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Zhongli {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Zhongli").vision(Geo).weapon(Polearm).release_date("2020-12-01").version(1.1)
            .base_hp(14695.0).base_atk(251.0).base_def(738.0)
            .dmg_geo(28.8)
            .energy_cost(40.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(3.0, 6))
            .press(DotTimer::new(8.0, 2.0, 4))
            .burst(DotTimer::single_hit(12.0))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na_1.icd_timer = &mut timers.na_icd;
        self.na_2.icd_timer = &mut timers.na_icd;
        self.na_3.icd_timer = &mut timers.na_icd;
        self.na_4.icd_timer = &mut timers.na_icd;
        self.na_5.icd_timer = &mut timers.na_icd;
        self.na_6.icd_timer = &mut timers.na_icd;
        self.press.icd_timer = &mut timers.skill_icd;
        self.press_dot.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
    }
}

impl SpecialAbility for Zhongli {
    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<FieldEnergy>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        if timers.burst_timer().is_active() {
            atk_queue.push(ElementalAttack::geo(&self.burst));
        }
        let press = timers.press_timer();
        if press.is_active() {
            if press.n() == 1 {
                atk_queue.push(ElementalAttack::geo(&self.press));
                atk_queue.push(ElementalAttack::geo(&self.press_dot));
                particles.push_p(Particle::new(Geo, 0.5));
            } else {
                atk_queue.push(ElementalAttack::geo(&self.press_dot));
                particles.push_p(Particle::new(Geo, 0.5));
            }
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push_geo(data, &self.na_1),
                2 => atk_queue.push_geo(data, &self.na_2),
                3 => atk_queue.push_geo(data, &self.na_3),
                4 => atk_queue.push_geo(data, &self.na_4),
                5 => atk_queue.push_geo(data, &self.na_5),
                6 => atk_queue.push_geo(data, &self.na_6),
                _ => (),
            };
        }
    }
}

pub struct Xinyan {
    skill_a4: DurationTimer,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    press: Attack,
    press_dot: Attack,
    burst: Attack,
    burst_dot: Attack,
}

impl Xinyan {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            skill_a4: DurationTimer::new(0.0, 12.0),
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 151.3,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 146.2,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 188.7,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 228.99,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE1A,
                multiplier: 305.28,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press_dot: Attack {
                kind: AttackType::SkillDot,
                gauge: &GAUGE1A,
                multiplier: 60.48,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE1A,
                multiplier: 613.44,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_dot: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 72.0,
                hits: 4,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Xinyan {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Xinyan").vision(Pyro).weapon(Claymore).release_date("2020-12-01").version(1.1)
            .base_hp(11201.0).base_atk(249.0).base_def(799.0)
            .atk(24.0)
            .energy_cost(60.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.8, 4))
            .press(DotTimer::new(18.0, 2.0, 6))
            .burst(DotTimer::single_hit(15.0))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na_1.icd_timer = &mut timers.na_icd;
        self.na_2.icd_timer = &mut timers.na_icd;
        self.na_3.icd_timer = &mut timers.na_icd;
        self.na_4.icd_timer = &mut timers.na_icd;
        self.press.icd_timer = &mut timers.skill_icd;
        self.press_dot.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
        self.burst_dot.icd_timer = &mut timers.burst_icd;
    }
}

impl SpecialAbility for Xinyan {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[FieldEnergy], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.skill_a4.update(guard.check_second(PressSkill), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<FieldEnergy>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        if timers.burst_timer().is_active() {
            atk_queue.push(ElementalAttack::physical(&self.burst));
            atk_queue.push(ElementalAttack::pyro(&self.burst_dot));
        }
        let press = timers.press_timer();
        if press.is_active() {
            if press.n() == 1 {
                atk_queue.push(ElementalAttack::pyro(&self.press));
                atk_queue.push(ElementalAttack::pyro(&self.press_dot));
                particles.push_p(Particle::new(Pyro, 4.0));
            } else {
                atk_queue.push(ElementalAttack::pyro(&self.press_dot));
            }
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push_pyro(data, &self.na_1),
                2 => atk_queue.push_pyro(data, &self.na_2),
                3 => atk_queue.push_pyro(data, &self.na_3),
                4 => atk_queue.push_pyro(data, &self.na_4),
                _ => (),
            };
        }
    }

    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        if self.skill_a4.is_active() {
            for s in modifiable_state.iter_mut() {
                s.physical_dmg += 15.0;
            }
        }
    }

    fn reset(&mut self) -> () {
        self.skill_a4.reset();
    }
}
