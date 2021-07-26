use std::ptr;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Vision, Particle, GAUGE1A, GAUGE2B, GAUGE4C};
use crate::fc::{FieldCharacterIndex, SpecialAbility, CharacterAbility, CharacterData, CharacterRecord, Enemy};
use crate::action::{Attack, ElementalAttack, ElementalAttackVector, FullCharacterTimers, CharacterTimersBuilder, StackTimer, TimerGuard, EffectTimer, DurationTimer, HitsTimer, DotTimer, LoopTimer};

use AttackType::*;
use WeaponType::*;
use Vision::*;

pub struct RaidenShogun {
    resolve_stack: f32,
    musou_isshin: DurationTimer,
    musou_isshin_energy: DotTimer,
    a1_timer: HitsTimer,
    dmg_bonus: f32,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    na_5: Attack,
    press: Attack,
    press_dot: Attack,
    burst: Attack,
    burst_na_1: Attack,
    burst_na_2: Attack,
    burst_na_3: Attack,
    burst_na_4: Attack,
    burst_na_5: Attack,
}

impl RaidenShogun {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            resolve_stack: 0.0,
            musou_isshin: DurationTimer::new(20.0, 7.0),
            musou_isshin_energy: DotTimer::new(20.0, 1.0, 5),
            a1_timer: HitsTimer::new(3.0, 1),
            dmg_bonus: 0.0,
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 78.37,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 78.54,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 98.6,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 57.29,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_5: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 120.24,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE2B,
                multiplier: 210.96,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press_dot: Attack {
                kind: AttackType::SkillDot,
                gauge: &GAUGE1A,
                multiplier: 75.6,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE4C,
                multiplier: 706.32,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_na_1: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 79.82,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_na_2: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 78.42,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_na_3: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 96.02,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_na_4: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: (55.11 + 55.26) / 2.0,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_na_5: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 131.92,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for RaidenShogun {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Raiden Shogun").vision(Electro).weapon(Polearm).release_date("2021-07-20").version(2.1)
            .base_hp(12907.0).base_atk(337.0).base_def(789.0)
            .dmg_electro(28.0)
            .energy_cost(80.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.217, 5))
            .press(DotTimer::new(10.0, 0.9, 28))
            .burst(DotTimer::single_hit(20.0))
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
        self.burst_na_1.icd_timer = &mut timers.burst_icd;
        self.burst_na_2.icd_timer = &mut timers.burst_icd;
        self.burst_na_3.icd_timer = &mut timers.burst_icd;
        self.burst_na_4.icd_timer = &mut timers.burst_icd;
        self.burst_na_5.icd_timer = &mut timers.burst_icd;
    }
}

impl SpecialAbility for RaidenShogun {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        self.a1_timer.update(guard.second(particles.len() > 0), time);
        if self.a1_timer.is_active() {
            self.resolve_stack += 2.0;
        }
        self.musou_isshin.update(guard.check_second(Burst), time);
        if guard.second {
            self.dmg_bonus = self.resolve_stack * 0.97;
            self.resolve_stack = 0.0;
        }
        self.musou_isshin_energy.update(guard.second(self.musou_isshin.is_active() && guard.kind == Na), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, _enemy: &Enemy) -> () {
        let burst = timers.burst_timer();
        if burst.is_active() {
            atk_queue.push(ElementalAttack::electro(&self.burst));
        }
        let press = timers.press_timer();
        if press.is_active() {
            if press.n() == 1 {
                atk_queue.push(ElementalAttack::electro(&self.press));
                atk_queue.push(ElementalAttack::electro(&self.press_dot));
                particles.push(Particle::new(Electro, 0.5));
            } else {
                atk_queue.push(ElementalAttack::electro(&self.press_dot));
                particles.push(Particle::new(Electro, 0.5));
            }
        }
        let na = timers.na_timer();
        if na.is_active() {
            if self.musou_isshin.is_active() {
                match na.n() {
                    1 => atk_queue.push(ElementalAttack::electro(&self.burst_na_1)),
                    2 => atk_queue.push(ElementalAttack::electro(&self.burst_na_2)),
                    3 => atk_queue.push(ElementalAttack::electro(&self.burst_na_3)),
                    4 => atk_queue.push(ElementalAttack::electro(&self.burst_na_4)),
                    5 => atk_queue.push(ElementalAttack::electro(&self.burst_na_5)),
                    _ => (),
                };
                // TODO recharge directly
                if self.musou_isshin_energy.is_active() {
                    // a4
                    let bonus = 1.0 + 0.6 * data.state.er / 100.0;
                    particles.push(Particle::new(Physical, 1.25 * bonus));
                }
            } else {
                match na.n() {
                    1 => atk_queue.push_electro(data, &self.na_1),
                    2 => atk_queue.push_electro(data, &self.na_2),
                    3 => atk_queue.push_electro(data, &self.na_3),
                    4 => atk_queue.push_electro(data, &self.na_4),
                    5 => atk_queue.push_electro(data, &self.na_5),
                    _ => (),
                };
            }
        }
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        let state = &mut modifiable_state[data.idx.0];
        if self.musou_isshin.is_active() {
            state.infusion = true;
            state.burst_dmg += self.dmg_bonus;
        }
        // a4
        state.electro_dmg += 0.4 * data.state.er;
    }

    fn reset(&mut self) -> () {
        self.resolve_stack = 0.0;
        self.musou_isshin.reset();
        self.musou_isshin_energy.reset();
        self.a1_timer.reset();
        self.dmg_bonus = 0.0;
    }
}

pub struct SangonomiyaKokomi {
    burst_timer: DurationTimer,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    press: Attack,
    burst: Attack,
}

impl SangonomiyaKokomi {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            burst_timer: DurationTimer::new(18.0, 10.0),
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 123.08,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 110.77,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 169.75,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::SkillDot,
                gauge: &GAUGE1A,
                multiplier: 148.9,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE1A,
                multiplier: 18.75,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for SangonomiyaKokomi {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Sangonomiya Kokomi").vision(Hydro).weapon(Catalyst).release_date("2021-08-10").version(2.1)
            .base_hp(11695.0).base_atk(222.0).base_def(615.0)
            // passive 2?
            .cr(-100.0)
            .dmg_hydro(28.8)
            .energy_cost(70.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(1.467, 3))
            // TODO ca
            .press(DotTimer::new(20.0, 2.0, 6))
            .burst(DotTimer::single_hit(18.0))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na_1.icd_timer = &mut timers.na_icd;
        self.na_2.icd_timer = &mut timers.na_icd;
        self.na_3.icd_timer = &mut timers.na_icd;
        self.press.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
    }
}

impl SpecialAbility for SangonomiyaKokomi {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        self.burst_timer.update(guard.check_second(Burst), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, _data: &CharacterData, _enemy: &Enemy) -> () {
        let burst = timers.burst_timer();
        if burst.is_active() {
            atk_queue.push(ElementalAttack::hydro(&self.burst));
        }
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::hydro(&self.press));
            particles.push(Particle::new(Hydro, 1.0));
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push(ElementalAttack::hydro(&self.na_1)),
                2 => atk_queue.push(ElementalAttack::hydro(&self.na_2)),
                3 => atk_queue.push(ElementalAttack::hydro(&self.na_3)),
                _ => (),
            };
        }
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.burst_timer.is_active() {
            let state = &mut modifiable_state[data.idx.0];
            let hp = data.state.HP();
            // TODO incorrect
            state.na_dmg += hp * 0.000871;
            state.ca_dmg += hp * 0.00122;
        }
    }

    fn reset(&mut self) -> () {
        self.burst_timer.reset();
    }
}

pub struct KujouSara {
    skill_timer: DurationTimer,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    na_5: Attack,
    press: Attack,
    burst: Attack,
    burst_dot: Attack,
}

impl KujouSara {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            skill_timer: DurationTimer::new(10.0, 10.0),
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 78.08,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 81.9,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 95.88,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 99.62,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_5: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 114.75,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE2B,
                multiplier: 226.37,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE1A,
                multiplier: 737.28,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_dot: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 61.42,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for KujouSara {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Kujou Sara").vision(Electro).weapon(Bow).release_date("2021-08-10").version(2.1)
            .base_hp(9570.0).base_atk(195.0).base_def(628.0)
            .atk(24.0)
            .energy_cost(80.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.1, 5))
            .press(DotTimer::single_hit(10.0))
            .burst(DotTimer::new(20.0, 2.0, 4))
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

    fn accelerate(&self, timers: &mut FullCharacterTimers) -> () {
        // TODO cooldown
        if timers.burst_timer().is_active() {
            // a1
            timers.reduce_cd(1.0);
        }
    }
}

impl SpecialAbility for KujouSara {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        self.skill_timer.update(guard.check_second(PressSkill), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, _enemy: &Enemy) -> () {
        let burst = timers.burst_timer();
        if burst.is_active() {
            if burst.n() == 1 {
                atk_queue.push(ElementalAttack::electro(&self.burst));
                atk_queue.push(ElementalAttack::electro(&self.burst_dot));
            } else {
                atk_queue.push(ElementalAttack::electro(&self.burst_dot));
            }
        }
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::electro(&self.press));
            particles.push(Particle::new(Electro, 2.5));
            // a4 TODO recharge directly
            let er = 100.0 + data.state.er;
            particles.push(Particle::new(Physical, 0.012 * er / 2.0));
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push_electro(data, &self.na_1),
                2 => atk_queue.push_electro(data, &self.na_2),
                3 => atk_queue.push_electro(data, &self.na_3),
                4 => atk_queue.push_electro(data, &self.na_4),
                5 => atk_queue.push_electro(data, &self.na_5),
                _ => (),
            };
        }
    }

    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        let state = &mut modifiable_state[data.idx.0];
        if self.skill_timer.is_active() || timers.burst_timer().is_active() {
            for s in modifiable_state.iter_mut() {
                s.flat_atk += data.state.base_atk * 0.8162;
            }
        }
    }

    fn reset(&mut self) -> () {
        self.skill_timer.reset();
    }
}

pub struct Aloy {
    coils: DurationTimer,
    coil_level: usize,
    skill_a1: DurationTimer,
    skill_a4: StackTimer,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    press: Attack,
    press_dot: Attack,
    burst: Attack,
}

impl Aloy {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            coils: DurationTimer::new(30.0, 10.0),
            coil_level: 0,
            skill_a1: DurationTimer::new(20.0, 10.0),
            skill_a4: StackTimer::new(1.0, 1.1, 10),
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: (37.68 + 42.39) / 2.0,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 76.93,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 94.2,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 117.12,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE1A,
                multiplier: 319.68,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press_dot: Attack {
                kind: AttackType::SkillDot,
                gauge: &GAUGE1A,
                multiplier: 72.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE1A,
                multiplier: 646.56,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Aloy {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Aloy").vision(Electro).weapon(Bow).release_date("2021-08-10").version(2.1)
            .base_hp(10899.0).base_atk(234.0).base_def(676.0)
            .dmg_cryo(28.8)
            .energy_cost(40.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(1.6, 4))
            .press(DotTimer::new(20.0, 1.0, 4))
            .burst(DotTimer::single_hit(12.0))
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
    }
}

impl SpecialAbility for Aloy {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let press = timers.press_timer();
        self.coil_level = if press.n() > self.coil_level {
            press.n()
        } else {
            self.coil_level
        };
        self.coils.update(guard.second(self.coil_level == 4), time);
        self.skill_a1.update(guard.second(press.is_active()), time);
        self.skill_a4.update(guard.second(self.coils.is_active()), time);
        if self.coil_level > 0 && !self.coils.is_active() {
            self.coil_level = 0;
        }
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, _enemy: &Enemy) -> () {
        let burst = timers.burst_timer();
        if burst.is_active() {
            atk_queue.push(ElementalAttack::cryo(&self.burst));
        }
        let press = timers.press_timer();
        if press.is_active() {
            if press.n() == 1 {
                atk_queue.push(ElementalAttack::electro(&self.press));
                atk_queue.push(ElementalAttack::electro(&self.press_dot));
                particles.push(Particle::new(Cryo, 1.0));
            } else {
                atk_queue.push(ElementalAttack::electro(&self.press_dot));
                particles.push(Particle::new(Cryo, 1.0));
            }
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push_cryo(data, &self.na_1),
                2 => atk_queue.push_cryo(data, &self.na_2),
                3 => atk_queue.push_cryo(data, &self.na_3),
                4 => atk_queue.push_cryo(data, &self.na_4),
                _ => (),
            };
        }
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.skill_a1.is_active() {
            for (i, s) in modifiable_state.iter_mut().enumerate() {
                if i == data.idx.0 {
                    s.atk += 16.0;
                } else {
                    s.atk += 8.0;
                }
            }
        }
        let state = &mut modifiable_state[data.idx.0];
        match self.coil_level {
            1 => state.na_dmg += 9.52,
            2 => state.na_dmg += 19.04,
            3 => state.na_dmg += 28.56,
            4 => state.na_dmg += 47.6,
            _ => (),
        }
        if self.coils.is_active() {
            state.infusion = true;
        }
        if self.skill_a4.is_active() {
            state.cryo_dmg += 3.5 * self.skill_a4.n() as f32;
        }
    }

    fn reset(&mut self) -> () {
        self.coils.reset();
        self.coil_level = 0;
        self.skill_a1.reset();
        self.skill_a4.reset();
    }
}
