use std::ptr;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Vision, Particle, GAUGE1A, GAUGE2B};
use crate::fc::{FieldCharacterIndex, SpecialAbility, CharacterAbility, CharacterData, CharacterRecord, Enemy, Debuff};
use crate::action::{Attack, ElementalAttack, ElementalAttackVector, FullCharacterTimers, CharacterTimersBuilder, TimerGuard, EffectTimer, DurationTimer, HitsTimer, DotTimer, LoopTimer, StaminaTimer};
use crate::testutil;

use AttackType::*;
use WeaponType::*;
use Vision::*;

// version 1.0

pub struct Chongyun {
    skill_infusion: DurationTimer,
    skill_timer: DurationTimer,
    skill_expire: bool, // a4
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    press: Attack,
    burst: Attack,
}

impl Chongyun {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            skill_infusion: DurationTimer::new(15.0, 3.0),
            skill_timer: DurationTimer::new(15.0, 10.0),
            skill_expire: false,
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 138.38,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 124.78,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 158.78,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 200.09,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE2B,
                multiplier: 309.67,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 256.32,
                hits: 3,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Chongyun {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Chongyun").vision(Cryo).weapon(Claymore).release_date("2020-09-28").version(1.0)
            .base_hp(10984.0).base_atk(223.0).base_def(648.0)
            .atk(24.0)
            .energy_cost(40.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.834, 4))
            .press(DotTimer::single_hit(15.0))
            .burst(DotTimer::single_hit(12.0))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na_1.icd_timer = &mut timers.na_icd;
        self.na_2.icd_timer = &mut timers.na_icd;
        self.na_3.icd_timer = &mut timers.na_icd;
        self.na_4.icd_timer = &mut timers.na_icd;
        self.press.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
    }
}

impl SpecialAbility for Chongyun {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        let skill = guard.kind == PressSkill;
        self.skill_infusion.update(guard.second(skill), time);
        let before = self.skill_timer.is_active();
        self.skill_timer.update(guard, time);
        let after = self.skill_timer.is_active();
        self.skill_expire = before && !after;
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        let burst = timers.burst_timer();
        if burst.is_active() {
            atk_queue.push(ElementalAttack::cryo(&self.burst));
        }
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::cryo(&self.press));
            particles.push(Particle::new(Cryo, 4.0));
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
        if self.skill_expire {
            atk_queue.push(ElementalAttack::cryo(&self.press));
        }
    }

    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        // a1
        if self.skill_timer.is_active() {
            for s in modifiable_state.iter_mut() {
                s.atk_spd += 8.0; // TODO only melee characters
            }
        }
        if self.skill_infusion.is_active() {
            for s in modifiable_state.iter_mut() {
                s.infusion = true;
            }
        }
        if self.skill_expire {
            enemy.element_res_debuff.push(Debuff::chongyun_a4());
        }
    }

    fn reset(&mut self) -> () {
        self.skill_infusion.reset();
        self.skill_timer.reset();
        self.skill_expire = false;
    }
}

pub struct Kaeya {
    skill_a4: bool,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    na_5: Attack,
    press: Attack,
    burst: Attack,
}

impl Kaeya {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            skill_a4: false,
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 106.25,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 102.17,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 129.03,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 140.08,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_5: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 174.42,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE2B,
                multiplier: 344.16,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 139.92,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Kaeya {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Kaeya").vision(Cryo).weapon(Sword).release_date("2020-09-28").version(1.0)
            .base_hp(11636.0).base_atk(223.0).base_def(792.0)
            .er(26.7)
            .energy_cost(60.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.734, 5))
            .press(DotTimer::single_hit(6.0))
            .burst(DotTimer::new(15.0, 0.66666, 12))
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

impl SpecialAbility for Kaeya {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.skill_a4 = guard.kind == PressSkill && enemy.aura.aura == Hydro;
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        let burst = timers.burst_timer();
        if burst.is_active() {
            atk_queue.push(ElementalAttack::cryo(&self.burst));
        }
        let press = timers.press_timer();
        if press.is_active() {
            atk_queue.push(ElementalAttack::cryo(&self.press));
            particles.push(Particle::new(Cryo, 2.5));
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
        if self.skill_a4 {
            particles.push(Particle::new(Cryo, 2.0));
        }
    }

    fn reset(&mut self) -> () {
        self.skill_a4 = false;
    }
}

pub struct Qiqi {
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    na_5: Attack,
    press: Attack,
    press_dot: Attack,
    burst: Attack,
}

impl Qiqi {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 74.63,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 76.84,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 47.77,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 48.79,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_5: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 124.61,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE1A,
                multiplier: 172.8,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press_dot: Attack {
                kind: AttackType::SkillDot,
                gauge: &GAUGE1A,
                multiplier: 64.8,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE2B,
                multiplier: 512.64,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Qiqi {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Qiqi").vision(Cryo).weapon(Sword).release_date("2020-09-28").version(1.0)
            .base_hp(12368.0).base_atk(287.0).base_def(922.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.25, 5))
            .press(DotTimer::new(30.0, 3.0, 4))
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
    }
}

impl SpecialAbility for Qiqi {
    // fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
    // }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        let burst = timers.burst_timer();
        if burst.is_active() {
            atk_queue.push(ElementalAttack::cryo(&self.burst));
        }
        let press = timers.press_timer();
        if press.is_active() {
            if press.n() == 1 {
                atk_queue.push(ElementalAttack::cryo(&self.press));
                atk_queue.push(ElementalAttack::cryo(&self.press_dot));
            } else {
                atk_queue.push(ElementalAttack::cryo(&self.press_dot));
            }
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

    // fn reset(&mut self) -> () {
    // }
}
