use std::ptr;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Vision, Particle, GAUGE1A, GAUGE2B};
use crate::fc::{FieldCharacterIndex, SpecialAbility, CharacterAbility, CharacterData, CharacterRecord, Enemy};
use crate::action::{Attack, ElementalAttack, ElementalAttackVector, ElementalAbsorption, FullCharacterTimers, CharacterTimersBuilder, TimerGuard, EffectTimer, DurationTimer, HitsTimer, DotTimer, LoopTimer, StaminaTimer};
use crate::testutil;

use AttackType::*;
use WeaponType::*;
use Vision::*;

// version 1.0

pub struct Sucrose {
    skill_a1: bool,
    skill_a4: DurationTimer,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    press: Attack,
    burst_dot: Attack,
    burst_ea: ElementalAbsorption,
}

impl Sucrose {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            skill_a1: false,
            skill_a4: DurationTimer::new(15.0, 8.0),
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 60.24,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 55.11,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 69.21,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 86.25,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE1A,
                multiplier: 380.16,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_dot: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 65.81,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_ea: ElementalAbsorption::new(idx, BurstDot, 79.2, DurationTimer::new(20.0, 6.0)),
        }
    }
}

impl CharacterAbility for Sucrose {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Sucrose").vision(Anemo).weapon(Catalyst).release_date("2020-09-28").version(1.0)
            .base_hp(9244.0).base_atk(170.0).base_def(703.0)
            .dmg_anemo(24.0)
            .energy_cost(80.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(1.5, 4))
            .press(DotTimer::single_hit(15.0))
            .burst(DotTimer::new(20.0, 2.0, 3))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na_1.icd_timer = &mut timers.na_icd;
        self.na_2.icd_timer = &mut timers.na_icd;
        self.na_3.icd_timer = &mut timers.na_icd;
        self.na_4.icd_timer = &mut timers.na_icd;
        self.press.icd_timer = &mut timers.skill_icd;
        self.burst_dot.icd_timer = &mut timers.burst_icd;
        *(self.burst_ea.icd()) = &mut timers.burst_icd;
    }
}

impl SpecialAbility for Sucrose {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.skill_a1 = attack.iter().any(|a| enemy.trigger_er(&a.element).is_swirl());
        let should_update = unsafe {
            attack.iter().any(|&a|
                match (*a.atk).kind {
                    PressSkill | BurstDot => true,
                    _ => false,
                }
            )
        };
        self.skill_a4.update(guard.second(should_update), time);
        self.burst_ea.absorb(guard.check_second(Burst), enemy, time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        let burst = timers.burst_timer();
        if burst.is_active() {
            atk_queue.push(ElementalAttack::anemo(&self.burst_dot));
            if let Some(a) = self.burst_ea.attack() {
                atk_queue.push(a);
            }
        }
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::anemo(&self.press));
            particles.push(Particle::new(Anemo, 4.0));
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push(ElementalAttack::anemo(&self.na_1)),
                2 => atk_queue.push(ElementalAttack::anemo(&self.na_2)),
                3 => atk_queue.push(ElementalAttack::anemo(&self.na_3)),
                4 => atk_queue.push(ElementalAttack::anemo(&self.na_4)),
                _ => (),
            };
        }
    }

    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        // TODO inaccurate
        if self.skill_a1 {
            for s in modifiable_state.iter_mut() {
                s.em += 50.0;
            }
        }
        if self.skill_a4.is_active() {
            for (i, s) in modifiable_state.iter_mut().enumerate() {
                if i != data.idx.0 {
                    s.em += data.state.em * 0.2;
                }
            }
        }
    }

    fn reset(&mut self) -> () {
        self.skill_a1 = false;
        self.skill_a4.reset();
    }
}

pub struct TravelerAnemo {
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    na_5: Attack,
    na_last: Attack,
    press_cutting: Attack,
    press_storm: Attack,
    // TODO what is multiplier?
    // press_ea: ElementalAbsorption,
    burst: Attack,
    burst_ea: ElementalAbsorption,
}

impl TravelerAnemo {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 87.89,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 85.85,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 104.72,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 115.26,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_5: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 139.91,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_last: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 60.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press_cutting: Attack {
                kind: AttackType::SkillDot,
                gauge: &GAUGE1A,
                multiplier: 30.24,
                hits: 7,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press_storm: Attack {
                kind: AttackType::SkillDot,
                gauge: &GAUGE1A,
                multiplier: 345.6,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 145.44,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_ea: ElementalAbsorption::new(idx, BurstDot, 44.64, DurationTimer::new(15.0, 10.0)),
        }
    }
}

impl CharacterAbility for TravelerAnemo {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Traveler (Anemo)").vision(Anemo).weapon(Sword).release_date("2020-09-28").version(1.0)
            .base_hp(10875.0).base_atk(212.0).base_def(683.0)
            .atk(24.0)
            .energy_cost(60.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.55, 5))
            .press(DotTimer::new(8.0, 1.0, 2))
            .burst(DotTimer::new(15.0, 1.0, 10))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na_1.icd_timer = &mut timers.na_icd;
        self.na_2.icd_timer = &mut timers.na_icd;
        self.na_3.icd_timer = &mut timers.na_icd;
        self.na_4.icd_timer = &mut timers.na_icd;
        self.na_5.icd_timer = &mut timers.na_icd;
        self.na_last.icd_timer = &mut timers.na_icd;
        self.press_cutting.icd_timer = &mut timers.skill_icd;
        self.press_storm.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
        *(self.burst_ea.icd()) = &mut timers.burst_icd;
    }
}

impl SpecialAbility for TravelerAnemo {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.burst_ea.absorb(guard.check_second(Burst), enemy, time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        let burst = timers.burst_timer();
        if burst.is_active() {
            atk_queue.push(ElementalAttack::anemo(&self.burst));
            if let Some(a) = self.burst_ea.attack() {
                atk_queue.push(a);
            }
        }
        let press = timers.press_timer();
        if press.is_active() {
            if press.n() == 1 {
                atk_queue.push(ElementalAttack::anemo(&self.press_cutting));
            } else {
                atk_queue.push(ElementalAttack::anemo(&self.press_storm));
                particles.push(Particle::new(Anemo, 3.5));
            }
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push_anemo(data, &self.na_1),
                2 => atk_queue.push_anemo(data, &self.na_2),
                3 => atk_queue.push_anemo(data, &self.na_3),
                4 => atk_queue.push_anemo(data, &self.na_4),
                5 => {
                    atk_queue.push_anemo(data, &self.na_5);
                    atk_queue.push(ElementalAttack::anemo(&self.na_last));
                },
                _ => (),
            };
        }
    }
}

pub struct Jean {
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    na_5: Attack,
    press: Attack,
    burst: Attack,
    burst_dot: Attack,
}

impl Jean {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 95.54,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 90.1,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 119.17,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 130.22,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_5: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 156.57,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE2B,
                multiplier: 525.6,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE2B,
                multiplier: 764.64,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_dot: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 141.12,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Jean {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Jean").vision(Anemo).weapon(Sword).release_date("2020-09-28").version(1.0)
            .base_hp(14695.0).base_atk(239.0).base_def(769.0)
            // a4
            .energy_cost(80.0 * 0.8)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.55, 5))
            .press(DotTimer::single_hit(6.0))
            .burst(DotTimer::new(20.0, 1.0, 3))
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

impl SpecialAbility for Jean {
    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        let burst = timers.burst_timer();
        if burst.is_active() {
            if burst.n() == 1 {
                atk_queue.push(ElementalAttack::anemo(&self.burst));
                atk_queue.push(ElementalAttack::anemo(&self.burst_dot));
            } else {
                atk_queue.push(ElementalAttack::anemo(&self.burst_dot));
            }
        }
        let press = timers.press_timer();
        if press.is_active() {
            atk_queue.push(ElementalAttack::anemo(&self.press));
            particles.push(Particle::new(Anemo, 3.0));
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push_anemo(data, &self.na_1),
                2 => atk_queue.push_anemo(data, &self.na_2),
                3 => atk_queue.push_anemo(data, &self.na_3),
                4 => atk_queue.push_anemo(data, &self.na_4),
                5 => atk_queue.push_anemo(data, &self.na_5),
                _ => (),
            };
        }
    }
}

pub struct Venti {
    first_absorption: bool,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    na_5: Attack,
    na_6: Attack,
    press: Attack,
    burst: Attack,
    burst_ea: ElementalAbsorption,
}

impl Venti {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            first_absorption: false,
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 40.29,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 87.72,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 103.53,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 51.51,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_5: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 100.13,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_6: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 140.25,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE2B,
                multiplier: 496.8,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 67.68,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_ea: ElementalAbsorption::new(idx, BurstDot, 33.84, DurationTimer::new(15.0, 8.0)),
        }
    }
}

impl CharacterAbility for Venti {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Venti").vision(Anemo).weapon(Bow).release_date("2020-09-28").version(1.0)
            .base_hp(10531.0).base_atk(263.0).base_def(669.0)
            .er(32.0)
            // a4
            .energy_cost(60.0 - 15.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.55, 6))
            .press(DotTimer::single_hit(6.0))
            .burst(DotTimer::new(15.0, 1.0, 8))
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
        self.burst.icd_timer = &mut timers.burst_icd;
        *(self.burst_ea.icd()) = &mut timers.burst_icd;
    }
}

impl SpecialAbility for Venti {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        let before = self.burst_ea.did_absort();
        self.burst_ea.absorb(guard.check_second(Burst), enemy, time);
        let after = self.burst_ea.did_absort();
        self.first_absorption = !before && after;
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        let burst = timers.burst_timer();
        if burst.is_active() {
            atk_queue.push(ElementalAttack::anemo(&self.burst));
            if let Some(a) = self.burst_ea.attack() {
                atk_queue.push(a);
                if self.first_absorption {
                    particles.push(Particle::new(a.element, 5.0));
                }
            }
        }
        let press = timers.press_timer();
        if press.is_active() {
            atk_queue.push(ElementalAttack::anemo(&self.press));
            particles.push(Particle::new(Anemo, 3.0));
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push_anemo(data, &self.na_1),
                2 => atk_queue.push_anemo(data, &self.na_2),
                3 => atk_queue.push_anemo(data, &self.na_3),
                4 => atk_queue.push_anemo(data, &self.na_4),
                5 => atk_queue.push_anemo(data, &self.na_5),
                6 => atk_queue.push_anemo(data, &self.na_6),
                _ => (),
            };
        }
    }

    fn reset(&mut self) -> () {
        self.first_absorption = false;
    }
}
