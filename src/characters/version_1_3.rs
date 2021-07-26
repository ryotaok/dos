use std::ptr;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Vision, Particle, GAUGE1A, GAUGE2B};
use crate::fc::{FieldCharacterIndex, SpecialAbility, CharacterAbility, CharacterData, CharacterRecord, Enemy};
use crate::action::{Attack, ElementalAttack, ElementalAttackVector, ElementalAbsorption, FullCharacterTimers, CharacterTimersBuilder, TimerGuard, EffectTimer, DurationTimer, StackTimer, HitsTimer, DotTimer, LoopTimer, StaminaTimer};
use crate::testutil;

use AttackType::*;
use WeaponType::*;
use Vision::*;

pub struct Xiao {
    burst_timer: DurationTimer,
    skill_a1: StackTimer,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    na_5: Attack,
    na_6: Attack,
    plunge: Attack,
    press: Attack,
}

impl Xiao {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            burst_timer: DurationTimer::new(18.0, 15.0),
            skill_a1: StackTimer::new(3.0, 4.0, 5),
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 49.14,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 101.58,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 122.3,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 67.2,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_5: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 127.64,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_6: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 170.97,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            plunge: Attack {
                kind: AttackType::Ca,
                gauge: &GAUGE1A,
                multiplier: 404.02,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE2B,
                multiplier: 455.04,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Xiao {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Xiao").vision(Anemo).weapon(Polearm).release_date("2020-12-23").version(1.3)
            .base_hp(12736.0).base_atk(349.0).base_def(799.0)
            .cr(24.2)
            .energy_cost(70.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(3.2, 6))
            .ca(HitsTimer::new(1.7, 1))
            .stamina(StaminaTimer::new(0.0))
            .press(DotTimer::single_hit(10.0))
            .burst(DotTimer::single_hit(18.0))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na_1.icd_timer = &mut timers.na_icd;
        self.na_2.icd_timer = &mut timers.na_icd;
        self.na_3.icd_timer = &mut timers.na_icd;
        self.na_4.icd_timer = &mut timers.na_icd;
        self.na_5.icd_timer = &mut timers.na_icd;
        self.na_6.icd_timer = &mut timers.na_icd;
        self.plunge.icd_timer = &mut timers.ca_icd;
        self.press.icd_timer = &mut timers.skill_icd;
    }

    fn use_ca(&self) -> bool {
        self.burst_timer.is_active()
    }
}

impl SpecialAbility for Xiao {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        self.burst_timer.update(guard.check_second(Burst), time);
        if self.burst_timer.is_active() {
            self.skill_a1.update(guard.second(true), time);
        }
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, _enemy: &Enemy) -> () {
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::anemo(&self.press));
            particles.push(Particle::new(Anemo, 3.0));
        }
        if timers.ca_timer().is_active() {
            atk_queue.push_anemo(data, &self.plunge);
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

    // TODO a4 is disabled for now
    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        let mut state = &mut modifiable_state[data.idx.0];
        if self.burst_timer.is_active() {
            state.infusion = true;
            state.na_dmg += 95.2;
            state.ca_dmg += 95.2;
            if self.skill_a1.is_active() {
                state.all_dmg += 5.0 * self.skill_a1.n as f32;
            }
        }
    }

    fn reset(&mut self) -> () {
        self.burst_timer.reset();
        self.skill_a1.reset();
    }
}

pub struct HuTao {
    skill_timer: DurationTimer,
    skill_expire: DurationTimer,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    na_5: Attack,
    na_6: Attack,
    ca: Attack,
    blood_blossom: Attack,
    burst: Attack,
}

impl HuTao {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            skill_timer: DurationTimer::new(16.0, 9.0),
            skill_expire: DurationTimer::new(0.0, 8.0),
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 83.65,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 86.09,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 108.92,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 117.11,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_5: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: (59.36 + 62.8) / 2.0,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_6: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 153.36,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            ca: Attack {
                kind: AttackType::Ca,
                gauge: &GAUGE1A,
                multiplier: 242.57,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            blood_blossom: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE2B,
                multiplier: 115.2,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE1A,
                multiplier: 617.44,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for HuTao {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Hu Tao").vision(Pyro).weapon(Polearm).release_date("2021-01-12").version(1.3)
            .base_hp(15552.0).base_atk(106.0).base_def(876.0)
            .cd(88.4)
            // a4
            .dmg_pyro(33.0)
            .energy_cost(60.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.933, 6))
            .ca(HitsTimer::new(0.915, 1))
            .stamina(StaminaTimer::new(25.0 + 15.0)) // TODO dash cancel
            .press(DotTimer::new(16.0, 4.0, 2))
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
        self.ca.icd_timer = &mut timers.ca_icd;
        self.blood_blossom.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
    }

    fn use_ca(&self) -> bool {
        self.skill_timer.is_active()
    }
}

impl SpecialAbility for HuTao {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        let before = self.skill_timer.is_active();
        self.skill_timer.update(guard.check_second(PressSkill), time);
        let after = self.skill_timer.is_active();
        self.skill_expire.update(guard.second(before && !after), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, _enemy: &Enemy) -> () {
        if timers.burst_timer().is_active() {
            atk_queue.push(ElementalAttack::pyro(&self.burst));
        }
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::pyro(&self.blood_blossom));
            particles.push(Particle::new(Pyro, 1.5));
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push_pyro(data, &self.na_1),
                2 => atk_queue.push_pyro(data, &self.na_2),
                3 => atk_queue.push_pyro(data, &self.na_3),
                4 => atk_queue.push_pyro(data, &self.na_4),
                5 => atk_queue.push_pyro(data, &self.na_5),
                6 => atk_queue.push_pyro(data, &self.na_6),
                _ => (),
            };
        }
        if timers.ca_timer().is_active() {
            atk_queue.push(ElementalAttack::pyro(&self.na_1));
            atk_queue.push(ElementalAttack::pyro(&self.ca));
        }
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.skill_timer.is_active() {
            let mut state = &mut modifiable_state[data.idx.0];
            state.infusion = true;
            state.flat_atk += data.state.HP() * 0.0626;
        }
        // a1
        if self.skill_expire.is_active() {
            for (i, s) in modifiable_state.iter_mut().enumerate() {
                if i != data.idx.0 {
                    s.cr += 12.0;
                }
            }
        }
    }

    fn reset(&mut self) -> () {
        self.skill_timer.reset();
        self.skill_expire.reset();
    }
}
