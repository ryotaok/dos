use std::ptr;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Vision, Particle, GAUGE1A, GAUGE2B};
use crate::fc::{FieldCharacterIndex, SpecialAbility, CharacterAbility, CharacterData, CharacterRecord, Enemy};
use crate::action::{Attack, ElementalAttack, ElementalAttackVector, FullCharacterTimers, CharacterTimersBuilder, TimerGuard, EffectTimer, DurationTimer, DotTimer, LoopTimer};


use AttackType::*;
use WeaponType::*;
use Vision::*;

// version 1.0

pub struct Barbara {
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
}

impl Barbara {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 68.11,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 63.94,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 73.87,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 99.36,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Barbara {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Barbara").vision(Hydro).weapon(Catalyst).release_date("2020-09-28").version(1.0)
            .base_hp(9787.0).base_atk(159.0).base_def(669.0)
            .hp(24.0)
            // .na_0(0.0).ca_1(299.23).ca_2(0.0).ca_time(1.7)
            // .press_cd(32.0).press_particle(0.0).press_dmg(0.0)
            // .burst_cd(20.0).energy_cost(80.0).burst_dmg(0.0)
            .energy_cost(80.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(1.5, 4))
            // .press(DotTimer::single_hit(10.0))
            // .burst(DotTimer::new(12.0, 0.5, 3))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na_1.icd_timer = &mut timers.na_icd;
        self.na_2.icd_timer = &mut timers.na_icd;
        self.na_3.icd_timer = &mut timers.na_icd;
        self.na_4.icd_timer = &mut timers.na_icd;
        // self.press.icd_timer = &mut timers.skill_icd;
        // self.burst.icd_timer = &mut timers.burst_icd;
    }
}

impl SpecialAbility for Barbara {
    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, _particles: &mut Vec<Particle>, timers: &FullCharacterTimers, _data: &CharacterData, _enemy: &Enemy) -> () {
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push(ElementalAttack::hydro(&self.na_1)),
                2 => atk_queue.push(ElementalAttack::hydro(&self.na_2)),
                3 => atk_queue.push(ElementalAttack::hydro(&self.na_3)),
                4 => atk_queue.push(ElementalAttack::hydro(&self.na_4)),
                _ => (),
            };
        }
    }
}

pub struct Xingqiu {
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    na_5: Attack,
    press: Attack,
    burst: Attack,
}

impl Xingqiu {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 92.14,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 94.18,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 56.44,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 110.67,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_5: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 70.89,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE1A,
                multiplier: (302.4 + 344.16) / 2.0,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 103.12,
                hits: 3,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Xingqiu {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Xingqiu").vision(Hydro).weapon(Sword).release_date("2020-09-28").version(1.0)
            .base_hp(10222.0).base_atk(202.0).base_def(758.0)
            .atk(24.0)
            // a4
            .dmg_hydro(20.0)
            .energy_cost(80.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.833, 5))
            .press(DotTimer::single_hit(21.0))
            .burst(DotTimer::new(20.0, 1.233, 13))
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

impl SpecialAbility for Xingqiu {
    // fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
    // }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, _enemy: &Enemy) -> () {
        if timers.burst_timer().is_active() {
            atk_queue.push(ElementalAttack::hydro(&self.burst));
        }
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::hydro(&self.press));
            particles.push(Particle::new(Hydro, 4.0));
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push_hydro(data, &self.na_1),
                2 => atk_queue.push_hydro(data, &self.na_2),
                3 => atk_queue.push_hydro(data, &self.na_3),
                4 => atk_queue.push_hydro(data, &self.na_4),
                5 => atk_queue.push_hydro(data, &self.na_5),
                _ => (),
            };
        }
    }

    // fn reset(&mut self) -> () {
    //     self.burst_aa.reset();
    // }
}

pub struct Mona {
    omen_timer: DurationTimer,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    press_dot: Attack,
    press_explosion: Attack,
    burst: Attack,
}

impl Mona {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            omen_timer: DurationTimer::new(15.0, 5.0),
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 67.68,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 64.8,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 80.64,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 101.09,
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
            press_explosion: Attack {
                kind: AttackType::SkillDot,
                gauge: &GAUGE1A,
                multiplier: 239.04,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE2B,
                multiplier: 796.32,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Mona {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Mona").vision(Hydro).weapon(Catalyst).release_date("2020-09-28").version(1.0)
            .base_hp(10409.0).base_atk(287.0).base_def(653.0)
            .er(32.0)
            .energy_cost(60.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(1.5, 4))
            .press(DotTimer::new(12.0, 1.0, 5))
            .burst(DotTimer::single_hit(15.0))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na_1.icd_timer = &mut timers.na_icd;
        self.na_2.icd_timer = &mut timers.na_icd;
        self.na_3.icd_timer = &mut timers.na_icd;
        self.na_4.icd_timer = &mut timers.na_icd;
        self.press_dot.icd_timer = &mut timers.skill_icd;
        self.press_explosion.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
    }
}

impl SpecialAbility for Mona {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let should_update = guard.kind == Burst;
        self.omen_timer.update(guard.second(should_update), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, _data: &CharacterData, _enemy: &Enemy) -> () {
        let burst = timers.burst_timer();
        if burst.is_active() {
            atk_queue.push(ElementalAttack::hydro(&self.burst));
        }
        let press = timers.press_timer();
        if press.is_active() {
            if press.n() == 5 {
                atk_queue.push(ElementalAttack::hydro(&self.press_explosion));
                particles.push(Particle::new(Hydro, 3.0));
            } else {
                atk_queue.push(ElementalAttack::hydro(&self.press_dot));
            }
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push(ElementalAttack::hydro(&self.na_1)),
                2 => atk_queue.push(ElementalAttack::hydro(&self.na_2)),
                3 => atk_queue.push(ElementalAttack::hydro(&self.na_3)),
                4 => atk_queue.push(ElementalAttack::hydro(&self.na_4)),
                _ => (),
            };
        }
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        let mut state = &mut modifiable_state[data.idx.0];
        // a4
        let er = 100.0 + data.state.er;
        state.hydro_dmg += er * 0.2;
        if self.omen_timer.is_active() {
            for s in modifiable_state.iter_mut() {
                s.all_dmg += 60.0;
            }
        }
    }

    fn reset(&mut self) -> () {
        self.omen_timer.reset();
    }
}
