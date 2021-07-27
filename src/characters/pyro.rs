use std::ptr;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Vision, Particle, GAUGE1A, GAUGE2B};
use crate::fc::{FieldCharacterIndex, SpecialAbility, CharacterAbility, CharacterData, CharacterRecord, Enemy};
use crate::action::{Attack, ElementalAttack, ElementalAttackVector, FullCharacterTimers, CharacterTimersBuilder, TimerGuard, EffectTimer, DurationTimer, HitsTimer, DotTimer, LoopTimer, StaminaTimer};
use crate::testutil;

use AttackType::*;
use WeaponType::*;
use Vision::*;

// version 1.0

pub struct Amber {
    ca_timer: DurationTimer,
    ca: Attack,
    press: Attack,
    burst: Attack,
}

impl Amber {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            ca_timer: DurationTimer::new(0.0, 10.0),
            ca: Attack {
                kind: AttackType::Ca,
                gauge: &GAUGE2B,
                multiplier: 223.2,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE2B,
                multiplier: 221.76,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE1A,
                multiplier: 50.54,
                hits: 18,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Amber {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Amber").vision(Pyro).weapon(Bow).release_date("2020-09-28").version(1.0)
            .base_hp(9461.0).base_atk(223.0).base_def(601.0)
            .atk(24.0)
            .energy_cost(40.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .ca(HitsTimer::new(2.0, 1))
            .stamina(StaminaTimer::new(0.0))
            .press(DotTimer::single_hit(15.0))
            .burst(DotTimer::single_hit(12.0))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.ca.icd_timer = &mut timers.ca_icd;
        self.press.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
    }

    fn use_ca(&self) -> bool {
        true
    }
}

impl SpecialAbility for Amber {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let should_update = guard.kind == Ca;
        self.ca_timer.update(guard.second(should_update), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, _data: &CharacterData, _enemy: &Enemy) -> () {
        if timers.burst_timer().is_active() {
            atk_queue.push(ElementalAttack::pyro(&self.burst));
        }
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::pyro(&self.press));
            particles.push(Particle::new(Pyro, 4.0));
        }
        if timers.ca_timer().is_active() {
            atk_queue.push(ElementalAttack::pyro(&self.ca));
        }
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.ca_timer.is_active() {
            // a4
            modifiable_state[data.idx.0].atk += 15.0;
        }
    }

    // a1
    fn intensify(&self, attack: &Attack) -> Option<State> {
        match &attack.kind {
            Burst => Some(State::new().cr(10.0)),
            _ => None,
        }
    }

    fn reset(&mut self) -> () {
        self.ca_timer.reset();
    }
}

pub struct Bennett {
    burst_timer: DurationTimer,
    bonus: f32,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    na_5: Attack,
    press: Attack,
    burst: Attack,
}

impl Bennett {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            burst_timer: DurationTimer::new(0.0, 12.0),
            bonus: 1.008,
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 88.06,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 84.49,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 107.95,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 117.98,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_5: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 142.12,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE2B,
                multiplier: 261.44,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE2B,
                multiplier: 419.04,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Bennett {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Bennett").vision(Pyro).weapon(Sword).release_date("2020-09-28").version(1.0)
            .base_hp(12397.0).base_atk(191.0).base_def(771.0)
            .er(26.7)
            .energy_cost(60.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.567, 5))
            // a1
            .press(DotTimer::single_hit(5.0 * 0.8))
            .burst(DotTimer::single_hit(15.0))
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
    }
}

impl SpecialAbility for Bennett {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let should_update = guard.kind == Burst;
        self.burst_timer.update(guard.second(should_update), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, _enemy: &Enemy) -> () {
        if timers.burst_timer().is_active() {
            atk_queue.push(ElementalAttack::pyro(&self.burst));
        }
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::pyro(&self.press));
            particles.push(Particle::new(Pyro, 2.0));
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push_pyro(data, &self.na_1),
                2 => atk_queue.push_pyro(data, &self.na_2),
                3 => atk_queue.push_pyro(data, &self.na_3),
                4 => atk_queue.push_pyro(data, &self.na_4),
                5 => atk_queue.push_pyro(data, &self.na_5),
                _ => (),
            };
        }
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.burst_timer.is_active() {
            for s in modifiable_state.iter_mut() {
                s.flat_atk += data.state.base_atk * self.bonus;
            }
        }
    }

    // TODO accelerate?
    // fn accelerate(&self, _na: &mut NormalAttackAction, skill: &mut SkillAction, _burst: &mut BurstAction) -> () {
    //     if self.burst_timer.is_active() {
    //         // a4
    //         skill.spd += 100.0;
    //     }
    // }

    fn reset(&mut self) -> () {
        self.burst_timer.reset();
    }
}

pub struct Xiangling {
    skill_a4: DurationTimer,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    na_5: Attack,
    press: Attack,
    burst: Attack,
    burst_aa: Attack,
}

impl Xiangling {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            skill_a4: DurationTimer::new(0.0, 10.0),
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 83.13,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 83.3,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 103.02,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 111.52,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_5: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 140.42,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::SkillDot,
                gauge: &GAUGE1A,
                multiplier: 200.3,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE1A,
                multiplier: (129.6 + 158.4 + 197.28) / 3.0,
                hits: 3,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_aa: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 201.6,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Xiangling {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Xiangling").vision(Pyro).weapon(Polearm).release_date("2020-09-28").version(1.0)
            .base_hp(10875.0).base_atk(225.0).base_def(669.0)
            .em(96.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.4, 5))
            .press(DotTimer::new(12.0, 2.0, 4))
            .burst(DotTimer::new(20.0, 1.0, 10))
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
        self.burst_aa.icd_timer = &mut timers.burst_icd;
    }
}

impl SpecialAbility for Xiangling {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let should_update = guard.kind == PressSkill;
        self.skill_a4.update(guard.second(should_update), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, _enemy: &Enemy) -> () {
        let burst = timers.burst_timer();
        if burst.is_active() {
            if burst.n() == 1 {
                atk_queue.push(ElementalAttack::pyro(&self.burst));
                atk_queue.push(ElementalAttack::pyro(&self.burst_aa));
            } else {
                atk_queue.push(ElementalAttack::pyro(&self.burst_aa));
            }
        }
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::pyro(&self.press));
            particles.push(Particle::new(Pyro, 1.0));
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push_pyro(data, &self.na_1),
                2 => atk_queue.push_pyro(data, &self.na_2),
                3 => atk_queue.push_pyro(data, &self.na_3),
                4 => atk_queue.push_pyro(data, &self.na_4),
                5 => atk_queue.push_pyro(data, &self.na_5),
                _ => (),
            };
        }
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, _data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.skill_a4.is_active() {
            modifiable_state[0].atk += 10.0;
        }
    }

    fn reset(&mut self) -> () {
        self.skill_a4.reset();
    }
}

pub struct Diluc {
    burst_a4: DurationTimer,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    press: Attack,
    burst: Attack,
    burst_aa: Attack,
}

impl Diluc {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            burst_a4: DurationTimer::new(0.0, 12.0),
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 177.31,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 173.23,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 195.33,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 264.86,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE1A,
                multiplier: (169.92 + 175.68 + 231.84) / 3.0,
                hits: 3,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE1A,
                multiplier: 367.2,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_aa: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 367.2,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Diluc {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Diluc").vision(Pyro).weapon(Claymore).release_date("2020-09-28").version(1.0)
            .base_hp(12981.0).base_atk(335.0).base_def(784.0)
            .cr(24.2)
            .energy_cost(40.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.834, 4))
            .press(DotTimer::single_hit(10.0))
            .burst(DotTimer::new(12.0, 0.5, 3))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na_1.icd_timer = &mut timers.na_icd;
        self.na_2.icd_timer = &mut timers.na_icd;
        self.na_3.icd_timer = &mut timers.na_icd;
        self.na_4.icd_timer = &mut timers.na_icd;
        self.press.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
        self.burst_aa.icd_timer = &mut timers.burst_icd;
    }
}

impl SpecialAbility for Diluc {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let should_update = guard.kind == Burst;
        self.burst_a4.update(guard.second(should_update), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, _enemy: &Enemy) -> () {
        let burst = timers.burst_timer();
        if burst.is_active() {
            if burst.n() == 1 {
                atk_queue.push(ElementalAttack::pyro(&self.burst));
                atk_queue.push(ElementalAttack::pyro(&self.burst_aa));
            } else {
                atk_queue.push(ElementalAttack::pyro(&self.burst_aa));
            }
        }
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::pyro(&self.press));
            particles.push(Particle::new(Pyro, 4.0));
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

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.burst_a4.is_active() {
            let mut state = &mut modifiable_state[data.idx.0];
            state.pyro_dmg += 20.0;
            state.infusion = true;
        }
    }

    fn reset(&mut self) -> () {
        self.burst_a4.reset();
    }
}

pub struct Klee {
    ca_a4: HitsTimer,
    na_a1: HitsTimer,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    press: Attack,
    press_aa: Attack,
    burst: Attack,
}

impl Klee {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            ca_a4: HitsTimer::new(1.0, 1),
            na_a1: HitsTimer::new(1.0, 1),
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 129.89,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 112.32,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 161.86,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE2B,
                multiplier: 171.36,
                hits: 3,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press_aa: Attack {
                kind: AttackType::SkillDot,
                gauge: &GAUGE1A,
                multiplier: 59.04,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 76.76,
                hits: 4,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Klee {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Klee").vision(Pyro).weapon(Catalyst).release_date("2020-09-28").version(1.0)
            .base_hp(10287.0).base_atk(311.0).base_def(615.0)
            .dmg_pyro(28.8)
            .energy_cost(60.0)
    }

    // TODO CA
    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(1.467, 3))
            .press(DotTimer::new(20.0, 1.0, 4))
            .burst(DotTimer::new(15.0, 1.0, 6))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na_1.icd_timer = &mut timers.na_icd;
        self.na_2.icd_timer = &mut timers.na_icd;
        self.na_3.icd_timer = &mut timers.na_icd;
        self.press.icd_timer = &mut timers.skill_icd;
        self.press_aa.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
    }
}

impl SpecialAbility for Klee {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let ca = guard.kind == Ca;
        let na_a1 = testutil::chance() < 0.5 && (guard.kind == Na || guard.kind == PressSkill);
        self.ca_a4.update(guard.second(ca), time);
        self.na_a1.update(guard.second(na_a1), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, _enemy: &Enemy) -> () {
        let burst = timers.burst_timer();
        if burst.is_active() {
            atk_queue.push(ElementalAttack::pyro(&self.burst));
        }
        let press = timers.press_timer();
        if press.is_active() {
            if press.n() == 1 {
                atk_queue.push(ElementalAttack::pyro(&self.press));
                atk_queue.push(ElementalAttack::pyro(&self.press_aa));
                particles.push(Particle::new(Pyro, 3.5));
            } else {
                atk_queue.push(ElementalAttack::pyro(&self.press_aa));
            }
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push(ElementalAttack::pyro(&self.na_1)),
                2 => atk_queue.push(ElementalAttack::pyro(&self.na_2)),
                3 => atk_queue.push(ElementalAttack::pyro(&self.na_3)),
                _ => (),
            };
        }
        if self.ca_a4.is_active() {
            particles.push(Particle::neutral(1.0 * data.state.cr / 100.0));
        }
    }

    // a1
    fn intensify(&self, attack: &Attack) -> Option<State> {
        if attack.kind == Ca && self.na_a1.is_active() {
            Some(State::new().ca_dmg(50.0))
        } else {
            None
        }
    }

    fn reset(&mut self) -> () {
        self.ca_a4.reset();
        self.na_a1.reset();
    }
}
