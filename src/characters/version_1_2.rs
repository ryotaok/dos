use std::ptr;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Vision, FieldEnergy, VecFieldEnergy, Particle, GAUGE1A, GAUGE2B, GAUGE4C};
use crate::fc::{FieldCharacterIndex, SpecialAbility, CharacterAbility, CharacterData, CharacterRecord, Enemy};
use crate::action::{Attack, ElementalAttack, ElementalAttackVector, ElementalAbsorption, FullCharacterTimers, CharacterTimersBuilder, TimerGuard, EffectTimer, DurationTimer, HitsTimer, DotTimer, LoopTimer, StaminaTimer};
use crate::testutil;

use AttackType::*;
use WeaponType::*;
use Vision::*;

// version 1.2

pub struct Albedo {
    burst_timer: DurationTimer,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    na_5: Attack,
    press: Attack,
    press_dot: Attack,
    burst: Attack,
    burst_dot: Attack,
}

impl Albedo {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            burst_timer: DurationTimer::new(12.0, 10.0),
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 72.62,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 72.62,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 93.81,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 98.35,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_5: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 122.7,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE1A,
                multiplier: 234.72,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press_dot: Attack {
                kind: AttackType::SkillDot,
                gauge: &GAUGE1A,
                multiplier: 240.48,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE1A,
                multiplier: 660.96,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_dot: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 129.6,
                hits: 3,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Albedo {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Albedo").vision(Geo).weapon(Sword).release_date("2020-12-23").version(1.2)
            .base_hp(13226.0).base_atk(251.0).base_def(876.0)
            .dmg_geo(28.8)
            .energy_cost(40.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.567, 5))
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
        self.press.icd_timer = &mut timers.skill_icd;
        self.press_dot.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
        self.burst_dot.icd_timer = &mut timers.burst_icd;
    }
}

impl SpecialAbility for Albedo {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[FieldEnergy], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.burst_timer.update(guard.check_second(Burst), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<FieldEnergy>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        if timers.burst_timer().is_active() {
            atk_queue.push(ElementalAttack::geo(&self.burst));
            atk_queue.push(ElementalAttack::geo(&self.burst_dot));
        }
        let press = timers.press_timer();
        if press.is_active() {
            if press.n() == 1 {
                atk_queue.push(ElementalAttack::geo(&self.press));
                atk_queue.push(ElementalAttack::geo(&self.press_dot));
                particles.push_p(Particle::new(Geo, 0.8));
            } else {
                atk_queue.push(ElementalAttack::geo(&self.press_dot));
                particles.push_p(Particle::new(Geo, 0.8));
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
                _ => (),
            };
        }
    }

    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        // a4
        if self.burst_timer.is_active() {
            for s in modifiable_state.iter_mut() {
                s.em += 125.0;
            }
        }
    }

    fn reset(&mut self) -> () {
        self.burst_timer.reset();
    }
}

pub struct Ganyu {
    burst_timer: DurationTimer,
    frostflake_arrow: Attack,
    frostflake_bloom: Attack,
    press: Attack,
    burst: Attack,
}

impl Ganyu {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            burst_timer: DurationTimer::new(15.0, 15.0),
            frostflake_arrow: Attack {
                kind: AttackType::Ca,
                gauge: &GAUGE1A,
                multiplier: 230.4,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            frostflake_bloom: Attack {
                kind: AttackType::Ca,
                gauge: &GAUGE1A,
                multiplier: 391.68,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE1A,
                multiplier: 237.6,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE1A,
                multiplier: 126.49,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Ganyu {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Ganyu").vision(Cryo).weapon(Bow).release_date("2021-01-12").version(1.2)
            .base_hp(9797.0).base_atk(335.0).base_def(630.0)
            .cd(88.4)
            .energy_cost(60.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .ca(HitsTimer::new(2.466, 2))
            .stamina(StaminaTimer::new(0.0))
            .press(DotTimer::new(10.0, 5.0, 2))
            .burst(DotTimer::new(15.0, 0.83, 18))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.frostflake_arrow.icd_timer = &mut timers.ca_icd;
        self.frostflake_bloom.icd_timer = &mut timers.ca_icd;
        self.press.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
    }

    fn use_ca(&self) -> bool {
        true
    }
}

impl SpecialAbility for Ganyu {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[FieldEnergy], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        self.burst_timer.update(guard.check_second(Burst), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<FieldEnergy>, timers: &FullCharacterTimers, _data: &CharacterData, _enemy: &Enemy) -> () {
        if timers.burst_timer().is_active() {
            atk_queue.push(ElementalAttack::cryo(&self.burst));
        }
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::cryo(&self.press));
            particles.push_p(Particle::new(Cryo, 2.0)); // cast and explosion
        }
        let ca = timers.ca_timer();
        if ca.is_active() {
            if ca.n() == 1 {
                atk_queue.push(ElementalAttack::cryo(&self.frostflake_arrow));
            } else {
                atk_queue.push(ElementalAttack::cryo(&self.frostflake_bloom));
            }
        }
    }

    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        // a4
        if self.burst_timer.is_active() {
            for s in modifiable_state.iter_mut() {
                s.cryo_dmg += 20.0;
            }
        }
    }

    // a1
    fn intensify(&self, attack: &Attack) -> Option<State> {
        if attack.kind == Ca {
            Some(State::new().cr(20.0))
        } else {
            None
        }
    }

    fn reset(&mut self) -> () {
        self.burst_timer.reset();
    }
}
