use std::ptr;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Vision, FieldEnergy, VecFieldEnergy, Particle, GAUGE1A, GAUGE2B};
use crate::fc::{FieldCharacterIndex, SpecialAbility, CharacterAbility, CharacterData, CharacterRecord, Enemy, Debuff};
use crate::action::{Attack, ElementalAttack, ElementalAttackVector, FullCharacterTimers, CharacterTimersBuilder, TimerGuard, EffectTimer, DurationTimer, HitsTimer, DotTimer, LoopTimer, StaminaTimer};

use AttackType::*;
use WeaponType::*;
use Vision::*;

pub struct Yanfei {
    burst_duration: DurationTimer,
    burst_grant_interval: DotTimer,
    scarlet_seal: usize,
    ca_a1: DurationTimer,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    ca_00: Attack,
    ca_01: Attack,
    ca_1: Attack,
    ca_2: Attack,
    ca_3: Attack,
    a4_blazing_eye: Attack,
    press: Attack,
    burst: Attack,
}

impl Yanfei {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            burst_duration: DurationTimer::new(20.0, 15.0),
            burst_grant_interval: DotTimer::new(20.0, 1.0, 15), // scarlet seal grant interval
            scarlet_seal: 0,
            ca_a1: DurationTimer::new(0.0, 6.0),
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 105.01,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 93.83,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 136.82,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            ca_00: Attack {
                kind: AttackType::Ca,
                gauge: &GAUGE1A,
                multiplier: 159.99,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            ca_01: Attack {
                kind: AttackType::Ca,
                gauge: &GAUGE1A,
                multiplier: 188.22,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            ca_1: Attack {
                kind: AttackType::Ca,
                gauge: &GAUGE1A,
                multiplier: 216.46,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            ca_2: Attack {
                kind: AttackType::Ca,
                gauge: &GAUGE1A,
                multiplier: 244.69,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            ca_3: Attack {
                kind: AttackType::Ca,
                gauge: &GAUGE1A,
                multiplier: 272.92,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            a4_blazing_eye: Attack {
                kind: AttackType::Ca,
                gauge: &GAUGE1A,
                multiplier: 80.0,
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
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE2B,
                multiplier: 328.32,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Yanfei {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Yanfei").vision(Pyro).weapon(Catalyst).release_date("2020-12-23").version(1.5)
            .base_hp(9352.0).base_atk(240.0).base_def(587.0)
            .dmg_pyro(24.0)
            .energy_cost(80.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(1.34, 3))
            .ca(HitsTimer::new(1.0, 1))
            .stamina(StaminaTimer::new(50.0 - 22.5)) // scarlet seal
            .press(DotTimer::single_hit(9.0))
            .burst(DotTimer::single_hit(20.0))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na_1.icd_timer = &mut timers.na_icd;
        self.na_2.icd_timer = &mut timers.na_icd;
        self.na_3.icd_timer = &mut timers.na_icd;
        self.ca_00.icd_timer = &mut timers.ca_icd;
        self.ca_01.icd_timer = &mut timers.ca_icd;
        self.ca_1.icd_timer = &mut timers.ca_icd;
        self.ca_2.icd_timer = &mut timers.ca_icd;
        self.ca_3.icd_timer = &mut timers.ca_icd;
        self.a4_blazing_eye.icd_timer = &mut timers.ca_icd;
        self.press.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
    }

    fn use_ca(&self) -> bool {
        self.scarlet_seal >= 3
    }
}

impl SpecialAbility for Yanfei {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[FieldEnergy], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        match &guard.kind {
            Na => self.scarlet_seal += 1,
            Ca => self.scarlet_seal = 0,
            PressSkill => self.scarlet_seal += 3,
            Burst => {
                self.burst_duration.update(guard.second(true), time);
                self.burst_grant_interval.update(guard.second(true), time);
            },
            _ => (),
        }
        if self.burst_grant_interval.is_active() {
            self.scarlet_seal += 1;
        }
        self.ca_a1.update(guard.second(self.use_ca()), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<FieldEnergy>, timers: &FullCharacterTimers, data: &CharacterData, _enemy: &Enemy) -> () {
        if timers.burst_timer().is_active() {
            atk_queue.push(ElementalAttack::pyro(&self.burst));
        }
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::pyro(&self.press));
            particles.push_p(Particle::new(Pyro, 3.0));
        }
        let ca = timers.ca_timer();
        if ca.is_active() {
            match self.scarlet_seal {
                0 => atk_queue.push(ElementalAttack::pyro(&self.ca_00)),
                1 => atk_queue.push(ElementalAttack::pyro(&self.ca_1)),
                2 => atk_queue.push(ElementalAttack::pyro(&self.ca_2)),
                _ => atk_queue.push(ElementalAttack::pyro(&self.ca_3)),
            };
            // TODO always crit
            atk_queue.push(ElementalAttack::pyro(&self.a4_blazing_eye));
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push_pyro(data, &self.na_1),
                2 => atk_queue.push_pyro(data, &self.na_2),
                3 => atk_queue.push_pyro(data, &self.na_3),
                _ => (),
            };
        }
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        let mut state = &mut modifiable_state[data.idx.0];
        if self.burst_duration.is_active() {
            state.ca_dmg += 54.4;
        }
        if self.ca_a1.is_active() {
            state.pyro_dmg += 15.0;
        }
    }

    fn reset(&mut self) -> () {
        self.burst_duration.reset();
        self.burst_grant_interval.reset();
        self.scarlet_seal = 0;
        self.ca_a1.reset();
    }
}

fn eula_stack(idx: FieldCharacterIndex, level: f32) -> Attack {
    Attack {
        kind: AttackType::BurstDot,
        gauge: &GAUGE1A,
        multiplier: 148.24 * level,
        hits: 1,
        icd_timer: ptr::null_mut(),
        idx,
    }
}

pub struct Eula {
    grimheart: usize,
    lightfall_sword_stack: usize,
    lightfall_sword_expire: bool,
    lightfall_sword_timer: DurationTimer,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    na_5: Attack,
    press: Attack,
    hold: Attack,
    icewhirl_brand_1: Attack,
    icewhirl_brand_2: Attack,
    hold_a1: Attack,
    burst: Attack,
    burst_lightfall_sword: Attack,
    burst_stack_1: Attack,
    burst_stack_2: Attack,
    burst_stack_3: Attack,
    burst_stack_4: Attack,
    burst_stack_5: Attack,
    burst_stack_6: Attack,
    burst_stack_7: Attack,
    burst_stack_8: Attack,
    burst_stack_9: Attack,
    burst_stack_10: Attack,
    burst_stack_11: Attack,
    burst_stack_12: Attack,
    burst_stack_13: Attack,
    burst_stack_14: Attack,
    burst_stack_15: Attack,
    burst_stack_16: Attack,
    burst_stack_17: Attack,
    burst_stack_18: Attack,
    burst_stack_19: Attack,
    burst_stack_20: Attack,
    burst_stack_21: Attack,
    burst_stack_22: Attack,
    burst_stack_23: Attack,
    burst_stack_24: Attack,
    burst_stack_25: Attack,
    burst_stack_26: Attack,
    burst_stack_27: Attack,
    burst_stack_28: Attack,
    burst_stack_29: Attack,
    burst_stack_30: Attack,
}

impl Eula {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            grimheart: 0,
            lightfall_sword_stack: 0,
            lightfall_sword_expire: false,
            lightfall_sword_timer: DurationTimer::new(20.0, 7.0),
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 177.38,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 184.93,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 112.28,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 222.67,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_5: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 142.0,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE1A,
                multiplier: 263.52,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            hold: Attack {
                kind: AttackType::HoldSkill,
                gauge: &GAUGE1A,
                multiplier: 442.08,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            icewhirl_brand_1: Attack {
                kind: AttackType::SkillDot,
                gauge: &GAUGE1A,
                multiplier: 172.8,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            icewhirl_brand_2: Attack {
                kind: AttackType::SkillDot,
                gauge: &GAUGE1A,
                multiplier: 172.8,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            hold_a1: Attack {
                kind: AttackType::SkillDot,
                gauge: &GAUGE1A,
                multiplier: 725.56 * 0.5,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE2B,
                multiplier: 617.44,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_lightfall_sword: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 725.56,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_stack_1: eula_stack(idx, 1.0),
            burst_stack_2: eula_stack(idx, 2.0),
            burst_stack_3: eula_stack(idx, 3.0),
            burst_stack_4: eula_stack(idx, 4.0),
            burst_stack_5: eula_stack(idx, 5.0),
            burst_stack_6: eula_stack(idx, 6.0),
            burst_stack_7: eula_stack(idx, 7.0),
            burst_stack_8: eula_stack(idx, 8.0),
            burst_stack_9: eula_stack(idx, 9.0),
            burst_stack_10: eula_stack(idx, 10.0),
            burst_stack_11: eula_stack(idx, 11.0),
            burst_stack_12: eula_stack(idx, 12.0),
            burst_stack_13: eula_stack(idx, 13.0),
            burst_stack_14: eula_stack(idx, 14.0),
            burst_stack_15: eula_stack(idx, 15.0),
            burst_stack_16: eula_stack(idx, 16.0),
            burst_stack_17: eula_stack(idx, 17.0),
            burst_stack_18: eula_stack(idx, 18.0),
            burst_stack_19: eula_stack(idx, 19.0),
            burst_stack_20: eula_stack(idx, 20.0),
            burst_stack_21: eula_stack(idx, 21.0),
            burst_stack_22: eula_stack(idx, 22.0),
            burst_stack_23: eula_stack(idx, 23.0),
            burst_stack_24: eula_stack(idx, 24.0),
            burst_stack_25: eula_stack(idx, 25.0),
            burst_stack_26: eula_stack(idx, 26.0),
            burst_stack_27: eula_stack(idx, 27.0),
            burst_stack_28: eula_stack(idx, 28.0),
            burst_stack_29: eula_stack(idx, 29.0),
            burst_stack_30: eula_stack(idx, 30.0),
        }
    }
}

impl CharacterAbility for Eula {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Eula").vision(Cryo).weapon(Claymore).release_date("2021-01-12").version(1.5)
            .base_hp(13226.0).base_atk(342.0).base_def(751.0)
            .cd(88.4)
            .energy_cost(80.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(3.85, 5))
            .press(DotTimer::single_hit(4.0))
            .hold(DotTimer::single_hit(10.0))
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
        self.hold.icd_timer = &mut timers.skill_icd;
        self.icewhirl_brand_1.icd_timer = &mut timers.skill_icd;
        self.icewhirl_brand_2.icd_timer = &mut timers.skill_icd;
        self.hold_a1.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
        self.burst_lightfall_sword.icd_timer = &mut timers.burst_icd;
        self.burst_stack_1.icd_timer = &mut timers.burst_icd;
        self.burst_stack_2.icd_timer = &mut timers.burst_icd;
        self.burst_stack_3.icd_timer = &mut timers.burst_icd;
        self.burst_stack_4.icd_timer = &mut timers.burst_icd;
        self.burst_stack_5.icd_timer = &mut timers.burst_icd;
        self.burst_stack_6.icd_timer = &mut timers.burst_icd;
        self.burst_stack_7.icd_timer = &mut timers.burst_icd;
        self.burst_stack_8.icd_timer = &mut timers.burst_icd;
        self.burst_stack_9.icd_timer = &mut timers.burst_icd;
        self.burst_stack_10.icd_timer = &mut timers.burst_icd;
        self.burst_stack_11.icd_timer = &mut timers.burst_icd;
        self.burst_stack_12.icd_timer = &mut timers.burst_icd;
        self.burst_stack_13.icd_timer = &mut timers.burst_icd;
        self.burst_stack_14.icd_timer = &mut timers.burst_icd;
        self.burst_stack_15.icd_timer = &mut timers.burst_icd;
        self.burst_stack_16.icd_timer = &mut timers.burst_icd;
        self.burst_stack_17.icd_timer = &mut timers.burst_icd;
        self.burst_stack_18.icd_timer = &mut timers.burst_icd;
        self.burst_stack_19.icd_timer = &mut timers.burst_icd;
        self.burst_stack_20.icd_timer = &mut timers.burst_icd;
        self.burst_stack_21.icd_timer = &mut timers.burst_icd;
        self.burst_stack_22.icd_timer = &mut timers.burst_icd;
        self.burst_stack_23.icd_timer = &mut timers.burst_icd;
        self.burst_stack_24.icd_timer = &mut timers.burst_icd;
        self.burst_stack_25.icd_timer = &mut timers.burst_icd;
        self.burst_stack_26.icd_timer = &mut timers.burst_icd;
        self.burst_stack_27.icd_timer = &mut timers.burst_icd;
        self.burst_stack_28.icd_timer = &mut timers.burst_icd;
        self.burst_stack_29.icd_timer = &mut timers.burst_icd;
        self.burst_stack_30.icd_timer = &mut timers.burst_icd;
    }

    fn use_hold(&self) -> bool {
        self.grimheart >= 2
    }
}

impl SpecialAbility for Eula {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, attack: &[ElementalAttack], _particles: &[FieldEnergy], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        // update lightfall_sword_timer
        let before = self.lightfall_sword_timer.is_active();
        self.lightfall_sword_timer.update(guard.check_second(Burst), time);
        let after  = self.lightfall_sword_timer.is_active();
        self.lightfall_sword_expire = before && !after;

        match &guard.kind {
            PressSkill => self.grimheart += 1,
            HoldSkill => self.grimheart = 0,
            Burst => self.grimheart += 1,
            _ => (),
        }

        // accumulate stacks
        if self.lightfall_sword_timer.is_active() {
            unsafe {
                for &a in attack {
                    let atk = &(*a.atk);
                    match &atk.kind {
                        Na | Ca | PressSkill | HoldSkill | SkillDot | Burst => self.lightfall_sword_stack += atk.hits,
                        _ => (),
                    };
                }
            }
        // do not clear the stacks on expire
        } else if !self.lightfall_sword_timer.is_active() && !self.lightfall_sword_expire {
            self.lightfall_sword_stack = 0;
        }
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<FieldEnergy>, timers: &FullCharacterTimers, data: &CharacterData, _enemy: &Enemy) -> () {
        if self.lightfall_sword_expire {
            atk_queue.push(ElementalAttack::physical(&self.burst_lightfall_sword));
            match self.lightfall_sword_stack {
                1 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_1)),
                2 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_2)),
                3 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_3)),
                4 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_4)),
                5 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_5)),
                6 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_6)),
                7 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_7)),
                8 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_8)),
                9 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_9)),
                10 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_10)),
                11 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_11)),
                12 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_12)),
                13 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_13)),
                14 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_14)),
                15 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_15)),
                16 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_16)),
                17 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_17)),
                18 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_18)),
                19 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_19)),
                20 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_20)),
                21 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_21)),
                22 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_22)),
                23 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_23)),
                24 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_24)),
                25 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_25)),
                26 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_26)),
                27 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_27)),
                28 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_28)),
                29 => atk_queue.push(ElementalAttack::physical(&self.burst_stack_29)),
                _ => atk_queue.push(ElementalAttack::physical(&self.burst_stack_30)),
            }
        }
        if timers.burst_timer().is_active() {
            atk_queue.push(ElementalAttack::cryo(&self.burst));
        }
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::cryo(&self.press));
            particles.push_p(Particle::new(Cryo, 1.5));
        }
        if timers.hold_timer().is_active() {
            atk_queue.push(ElementalAttack::cryo(&self.hold));
            particles.push_p(Particle::new(Cryo, 2.5));
            match self.grimheart {
                0 => (),
                1 => atk_queue.push(ElementalAttack::cryo(&self.icewhirl_brand_1)),
                _ => {
                    atk_queue.push(ElementalAttack::cryo(&self.icewhirl_brand_2));
                    atk_queue.push(ElementalAttack::physical(&self.hold_a1));
                },
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

    fn modify(&self, _modifiable_state: &mut [State], timers: &FullCharacterTimers, _data: &CharacterData, enemy: &mut Enemy) -> () {
        if timers.hold_timer().is_active() {
            enemy.element_res_debuff.push(Debuff::eula_cryo());
            enemy.physical_res_debuff.push(Debuff::eula_physical());
        }
    }

    // TODO accelerate

    fn reset(&mut self) -> () {
        self.grimheart = 0;
        self.lightfall_sword_stack = 0;
        self.lightfall_sword_expire = false;
        self.lightfall_sword_timer.reset();
    }
}
