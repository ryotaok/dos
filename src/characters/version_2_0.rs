use std::ptr;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Vision, FieldEnergy, VecFieldEnergy, Particle, GAUGE1A, GAUGE2B};
use crate::fc::{FieldCharacterIndex, SpecialAbility, CharacterAbility, CharacterData, CharacterRecord, Enemy};
use crate::action::{Attack, ElementalAttack, ElementalAttackVector, FullCharacterTimers, CharacterTimersBuilder, StackTimer, TimerGuard, EffectTimer, DurationTimer, DotTimer, LoopTimer};

use AttackType::*;
use WeaponType::*;
use Vision::*;

pub struct Ayaka {
    skill_a1: DurationTimer,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    na_5: Attack,
    press: Attack,
    burst: Attack,
    burst_dot: Attack,
}

impl Ayaka {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            skill_a1: DurationTimer::new(0.0, 6.0),
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 90.39,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 96.24,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 123.79,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 44.77,
                hits: 3,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_5: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 154.55,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE2B,
                multiplier: 430.56,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE1A,
                multiplier: 202.14,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_dot: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 303.21,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Ayaka {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Ayaka").vision(Cryo).weapon(Sword).release_date("2021-07-20").version(2.0)
            .base_hp(12858.0).base_atk(342.0).base_def(784.0)
            .cd(88.4)
            .energy_cost(80.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.117, 5))
            .press(DotTimer::single_hit(10.0))
            .burst(DotTimer::new(20.0, 0.3333, 15))
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

impl SpecialAbility for Ayaka {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[FieldEnergy], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        self.skill_a1.update(guard.check_second(PressSkill), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<FieldEnergy>, timers: &FullCharacterTimers, data: &CharacterData, _enemy: &Enemy) -> () {
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
            particles.push_p(Particle::new(Cryo, 3.5));
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

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        let s = &mut modifiable_state[data.idx.0];
        // Alternate Sprint (Kamisato Art: Senho)
        s.infusion = true;
        s.cryo_dmg += 18.0;
        if self.skill_a1.is_active() {
            s.na_dmg += 30.0;
            s.ca_dmg += 30.0;
        }
    }

    fn reset(&mut self) -> () {
        self.skill_a1.reset();
    }
}

pub struct Yoimiya {
    skill_timer: DurationTimer,
    skill_a1: StackTimer,
    burst_a4: DurationTimer,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    na_5: Attack,
    skill_na_1: Attack,
    skill_na_2: Attack,
    skill_na_3: Attack,
    skill_na_4: Attack,
    skill_na_5: Attack,
    press: Attack,
    burst: Attack,
    burst_dot: Attack,
}

impl Yoimiya {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            skill_timer: DurationTimer::new(18.0, 10.0),
            skill_a1: StackTimer::new(0.0, 3.0, 10),
            burst_a4: DurationTimer::new(0.0, 15.0),
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 63.59,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 121.99,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 158.59,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 82.82,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_5: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 188.87,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            skill_na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 63.59 * 1.6174,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            skill_na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 121.99 * 1.6174,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            skill_na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 158.59 * 1.6174,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            skill_na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 82.82 * 1.6174,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            skill_na_5: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 188.87 * 1.6174,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE1A,
                multiplier: 161.74,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE1A,
                multiplier: 228.96,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_dot: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 219.6,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Yoimiya {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Yoimiya").vision(Pyro).weapon(Bow).release_date("2021-08-10").version(2.0)
            .base_hp(10164.0).base_atk(323.0).base_def(615.0)
            .cr(24.2)
            .energy_cost(60.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.0, 5))
            .press(DotTimer::single_hit(18.0))
            .burst(DotTimer::new(15.0, 2.0, 5))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na_1.icd_timer = &mut timers.na_icd;
        self.na_2.icd_timer = &mut timers.na_icd;
        self.na_3.icd_timer = &mut timers.na_icd;
        self.na_4.icd_timer = &mut timers.na_icd;
        self.na_5.icd_timer = &mut timers.na_icd;
        self.skill_na_1.icd_timer = &mut timers.na_icd;
        self.skill_na_2.icd_timer = &mut timers.na_icd;
        self.skill_na_3.icd_timer = &mut timers.na_icd;
        self.skill_na_4.icd_timer = &mut timers.na_icd;
        self.skill_na_5.icd_timer = &mut timers.na_icd;
        self.press.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
        self.burst_dot.icd_timer = &mut timers.burst_icd;
    }
}

impl SpecialAbility for Yoimiya {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[FieldEnergy], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        self.skill_timer.update(guard.check_second(PressSkill), time);
        self.burst_a4.update(guard.check_second(Burst), time);
        if self.skill_timer.is_active() {
            self.skill_a1.update(guard.check_second(Na), time);
        }
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<FieldEnergy>, timers: &FullCharacterTimers, _data: &CharacterData, _enemy: &Enemy) -> () {
        let burst = timers.burst_timer();
        if burst.is_active() {
            if burst.n() == 1 {
                atk_queue.push(ElementalAttack::pyro(&self.burst));
                atk_queue.push(ElementalAttack::pyro(&self.burst_dot));
            } else {
                atk_queue.push(ElementalAttack::pyro(&self.burst_dot));
            }
        }
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::pyro(&self.press));
            particles.push_p(Particle::new(Pyro, 4.0));
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push(if self.skill_timer.is_active() {
                    ElementalAttack::pyro(&self.skill_na_1)
                } else {
                    ElementalAttack::physical(&self.na_1)
                }),
                2 => atk_queue.push(if self.skill_timer.is_active() {
                    ElementalAttack::pyro(&self.skill_na_2)
                } else {
                    ElementalAttack::physical(&self.na_2)
                }),
                3 => atk_queue.push(if self.skill_timer.is_active() {
                    ElementalAttack::pyro(&self.skill_na_3)
                } else {
                    ElementalAttack::physical(&self.na_3)
                }),
                4 => atk_queue.push(if self.skill_timer.is_active() {
                    ElementalAttack::pyro(&self.skill_na_4)
                } else {
                    ElementalAttack::physical(&self.na_4)
                }),
                5 => atk_queue.push(if self.skill_timer.is_active() {
                    ElementalAttack::pyro(&self.skill_na_5)
                } else {
                    ElementalAttack::physical(&self.na_5)
                }),
                _ => (),
            };
        }
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.burst_a4.is_active() {
            for (i, s) in modifiable_state.iter_mut().enumerate() {
                if i != data.idx.0 {
                    s.atk += 10.0 + self.skill_a1.n as f32;
                }
            }
        }
        let state = &mut modifiable_state[data.idx.0];
        if self.skill_a1.is_active() {
            state.pyro_dmg += 2.0 * self.skill_a1.n as f32;
        }
        if self.skill_timer.is_active() {
            state.infusion = true;
        }
    }

    fn reset(&mut self) -> () {
        self.skill_timer.reset();
        self.skill_a1.reset();
        self.burst_a4.reset();
    }
}

pub struct Sayu {
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    press: Attack,
    burst: Attack,
    muji_muji_daruma: Attack,
}

impl Sayu {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 142.8,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 141.1,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 85.85,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 193.97,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            // TODO
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE2B,
                multiplier: 64.8 + 285.12 + 30.24 + 137.09,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE1A,
                multiplier: 210.24,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            muji_muji_daruma: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 93.6,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Sayu {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Sayu").vision(Anemo).weapon(Claymore).release_date("2021-08-10").version(2.0)
            .base_hp(11854.0).base_atk(244.0).base_def(745.0)
            .em(96.0)
            .energy_cost(80.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.616, 4))
            .press(DotTimer::single_hit(6.0))
            .burst(DotTimer::new(20.0, 2.0, 5))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na_1.icd_timer = &mut timers.na_icd;
        self.na_2.icd_timer = &mut timers.na_icd;
        self.na_3.icd_timer = &mut timers.na_icd;
        self.na_4.icd_timer = &mut timers.na_icd;
        self.press.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
        self.muji_muji_daruma.icd_timer = &mut timers.burst_icd;
    }
}

impl SpecialAbility for Sayu {
    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<FieldEnergy>, timers: &FullCharacterTimers, data: &CharacterData, _enemy: &Enemy) -> () {
        let burst = timers.burst_timer();
        if burst.is_active() {
            if burst.n() == 1 {
                atk_queue.push(ElementalAttack::anemo(&self.burst));
                atk_queue.push(ElementalAttack::anemo(&self.muji_muji_daruma));
            } else {
                atk_queue.push(ElementalAttack::anemo(&self.muji_muji_daruma));
            }
        }
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::anemo(&self.press));
            particles.push_p(Particle::new(Anemo, 2.5));
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push_anemo(data, &self.na_1),
                2 => atk_queue.push_anemo(data, &self.na_2),
                3 => atk_queue.push_anemo(data, &self.na_3),
                4 => atk_queue.push_anemo(data, &self.na_4),
                _ => (),
            };
        }
    }
}
