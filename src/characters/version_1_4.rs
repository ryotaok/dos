use std::ptr;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Vision, Particle, GAUGE1A, GAUGE2B};
use crate::fc::{FieldCharacterIndex, SpecialAbility, CharacterAbility, CharacterData, CharacterRecord, Enemy};
use crate::action::{Attack, ElementalAttack, ElementalAttackVector, ElementalAbsorption, FullCharacterTimers, CharacterTimersBuilder, TimerGuard, EffectTimer, DurationTimer, HitsTimer, DotTimer, LoopTimer, StaminaTimer};
use crate::testutil;

use AttackType::*;
use WeaponType::*;
use Vision::*;

pub struct Rosaria {
    skill_a1: DurationTimer,
    burst_a4: DurationTimer,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    na_5: Attack,
    press: Attack,
    burst: Attack,
    burst_dot: Attack,
}

impl Rosaria {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            skill_a1: DurationTimer::new(0.0, 5.0),
            burst_a4: DurationTimer::new(0.0, 10.0),
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 103.7,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 102.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 62.9,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 137.7,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_5: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: (82.28 + 85.0) / 2.0,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE1A,
                multiplier: (105.21 + 244.8) / 2.0,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE1A,
                multiplier: (187.2 + 273.6) / 2.0,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_dot: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE1A,
                multiplier: 237.6,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Rosaria {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Rosaria").vision(Cryo).weapon(Polearm).release_date("2020-12-23").version(1.4)
            .base_hp(12289.0).base_atk(240.0).base_def(710.0)
            .atk(24.0)
            .energy_cost(60.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.733, 5))
            .press(DotTimer::single_hit(6.0))
            .burst(DotTimer::new(15.0, 2.0, 4))
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

impl SpecialAbility for Rosaria {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.skill_a1.update(guard.check_second(PressSkill), time);
        self.burst_a4.update(guard.check_second(Burst), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, _enemy: &Enemy) -> () {
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
            particles.push(Particle::new(Cryo, 3.0));
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

    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        if self.skill_a1.is_active() {
            modifiable_state[data.idx.0].cr += 12.0;
        }
        if self.burst_a4.is_active() {
            for (i, s) in modifiable_state.iter_mut().enumerate() {
                if i != data.idx.0 {
                    s.cr += data.state.cr * 0.15;
                }
            }
        }
    }

    fn reset(&mut self) -> () {
        self.skill_a1.reset();
        self.burst_a4.reset();
    }
}
