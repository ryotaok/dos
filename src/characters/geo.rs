use std::ptr;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Vision, FieldEnergy, VecFieldEnergy, Particle, GAUGE1A, GAUGE2B};
use crate::fc::{FieldCharacterIndex, SpecialAbility, CharacterAbility, CharacterData, CharacterRecord, Enemy};
use crate::action::{Attack, ElementalAttack, ElementalAttackVector, ElementalAbsorption, FullCharacterTimers, CharacterTimersBuilder, TimerGuard, EffectTimer, CDTimer, DurationTimer, HitsTimer, DotTimer, LoopTimer, StaminaTimer};
use crate::testutil;

use AttackType::*;
use WeaponType::*;
use Vision::*;

// version 1.0

pub struct Ningguang {
    star_jade: usize,
    skill_a4: DurationTimer,
    jade_screen: DurationTimer,
    na: Attack,
    ca: Attack,
    ca_star_jade_1: Attack,
    ca_star_jade_2: Attack,
    ca_star_jade_3: Attack,
    press: Attack,
    burst: Attack,
    burst_aa: Attack,
}

impl Ningguang {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            star_jade: 0,
            skill_a4: DurationTimer::new(0.0, 10.0),
            jade_screen: DurationTimer::new(0.0, 20.0),
            na: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 50.4,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            ca: Attack {
                kind: AttackType::Ca,
                gauge: &GAUGE1A,
                multiplier: 313.34,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            ca_star_jade_1: Attack {
                kind: AttackType::Ca,
                gauge: &GAUGE1A,
                multiplier: 89.28,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            ca_star_jade_2: Attack {
                kind: AttackType::Ca,
                gauge: &GAUGE1A,
                multiplier: 89.28,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            ca_star_jade_3: Attack {
                kind: AttackType::Ca,
                gauge: &GAUGE1A,
                multiplier: 89.28,
                hits: 3,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE1A,
                multiplier: 414.72,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 156.53,
                hits: 6,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_aa: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE1A,
                multiplier: 156.53,
                hits: 4,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Ningguang {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Ningguang").vision(Geo).weapon(Catalyst).release_date("2020-09-28").version(1.0)
            .base_hp(9787.0).base_atk(212.0).base_def(573.0)
            .dmg_geo(24.0)
            .energy_cost(40.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(0.9, 1))
            .ca(HitsTimer::new(2.0, 1))
            // a1
            .stamina(StaminaTimer::new(0.0))
            .press(DotTimer::single_hit(12.0))
            .burst(DotTimer::single_hit(12.0))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na.icd_timer = &mut timers.na_icd;
        self.ca.icd_timer = &mut timers.ca_icd;
        self.ca_star_jade_1.icd_timer = &mut timers.ca_icd;
        self.ca_star_jade_2.icd_timer = &mut timers.ca_icd;
        self.ca_star_jade_3.icd_timer = &mut timers.ca_icd;
        self.press.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
        self.burst_aa.icd_timer = &mut timers.burst_icd;
    }

    fn use_ca(&self) -> bool {
        self.star_jade >= 1
    }
}

impl SpecialAbility for Ningguang {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[FieldEnergy], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        guard.check_second(PressSkill);
        self.skill_a4.update(guard, time);
        self.jade_screen.update(guard, time);
        match guard.kind {
            Na => self.star_jade += 1,
            Ca => self.star_jade = 0,
            _ => (),
        }
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<FieldEnergy>, timers: &FullCharacterTimers, _data: &CharacterData, _enemy: &Enemy) -> () {
        if timers.burst_timer().is_active() {
            atk_queue.push(ElementalAttack::geo(&self.burst));
            if self.jade_screen.is_active() {
                atk_queue.push(ElementalAttack::geo(&self.burst_aa));
            }
        }
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::geo(&self.press));
            particles.push_p(Particle::new(Geo, 3.0));
        }
        if timers.ca_timer().is_active() {
            atk_queue.push(ElementalAttack::geo(&self.ca));
            match self.star_jade {
                1 => atk_queue.push(ElementalAttack::geo(&self.ca_star_jade_1)),
                2 => atk_queue.push(ElementalAttack::geo(&self.ca_star_jade_2)),
                3 => atk_queue.push(ElementalAttack::geo(&self.ca_star_jade_3)),
                _ => (),
            };
        }
        if timers.na_timer().is_active() {
            atk_queue.push(ElementalAttack::geo(&self.na));
        }
    }

    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        // a4
        if self.skill_a4.is_active() {
            for s in modifiable_state.iter_mut() {
                s.geo_dmg += 12.0;
            }
        }
    }

    fn reset(&mut self) -> () {
        self.skill_a4.reset();
        self.jade_screen.reset();
    }
}

pub struct Noelle {
    burst_timer: CDTimer,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    press: Attack,
    burst: Attack,
}

impl Noelle {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            burst_timer: CDTimer::new(15.0),
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 156.4,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 145.01,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 170.51,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 224.23,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE2B,
                multiplier: 216.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE1A,
                multiplier: (120.96 + 167.76) / 2.0,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for Noelle {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Noelle").vision(Geo).weapon(Claymore).release_date("2020-09-28").version(1.0)
            .base_hp(12071.0).base_atk(191.0).base_def(799.0)
            .def(30.0)
            .energy_cost(60.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.616, 4))
            .press(DotTimer::single_hit(24.0))
            .burst(DotTimer::single_hit(15.0))
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

    fn accelerate(&self, timers: &mut FullCharacterTimers) -> () {
        let na = timers.na_timer();
        if na.is_active() && na.n() == 4 {
            // a4
            timers.reduce_cd(1.0);
        }
    }
}

impl SpecialAbility for Noelle {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[FieldEnergy], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.burst_timer.update(guard.check_second(Burst), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<FieldEnergy>, timers: &FullCharacterTimers, data: &CharacterData, _enemy: &Enemy) -> () {
        let burst = timers.burst_timer();
        if burst.is_active() {
            atk_queue.push(ElementalAttack::geo(&self.burst));
        }
        if timers.press_timer().is_active() {
            atk_queue.push(ElementalAttack::geo(&self.press));
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push_geo(data, &self.na_1),
                2 => atk_queue.push_geo(data, &self.na_2),
                3 => atk_queue.push_geo(data, &self.na_3),
                4 => atk_queue.push_geo(data, &self.na_4),
                _ => (),
            };
        }
    }

    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        if self.burst_timer.is_active() {
            let mut state = &mut modifiable_state[data.idx.0];
            state.flat_atk += data.state.DEF() * 0.72;
            state.infusion = true;
        }
    }

    fn reset(&mut self) -> () {
        self.burst_timer.reset();
    }
}

pub struct TravelerGeo {
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    na_5: Attack,
    na_last: Attack,
    press: Attack,
    burst: Attack,
}

impl TravelerGeo {
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
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE2B,
                multiplier: 446.4,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::BurstDot,
                gauge: &GAUGE2B,
                multiplier: 266.4,
                hits: 4,
                icd_timer: ptr::null_mut(),
                idx,
            },
        }
    }
}

impl CharacterAbility for TravelerGeo {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Traveler (Geo)").vision(Geo).weapon(Sword).release_date("2020-09-28").version(1.0)
            .base_hp(10875.0).base_atk(212.0).base_def(683.0)
            .atk(24.0)
            .energy_cost(60.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.55, 5))
            // a1
            .press(DotTimer::single_hit(8.0 - 2.0))
            .burst(DotTimer::single_hit(15.0))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na_1.icd_timer = &mut timers.na_icd;
        self.na_2.icd_timer = &mut timers.na_icd;
        self.na_3.icd_timer = &mut timers.na_icd;
        self.na_4.icd_timer = &mut timers.na_icd;
        self.na_5.icd_timer = &mut timers.na_icd;
        self.na_last.icd_timer = &mut timers.na_icd;
        self.press.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
    }
}

impl SpecialAbility for TravelerGeo {
    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<FieldEnergy>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        let burst = timers.burst_timer();
        if burst.is_active() {
            atk_queue.push(ElementalAttack::geo(&self.burst));
        }
        let press = timers.press_timer();
        if press.is_active() {
            atk_queue.push(ElementalAttack::geo(&self.press));
            particles.push_p(Particle::new(Geo, 3.5));
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push_geo(data, &self.na_1),
                2 => atk_queue.push_geo(data, &self.na_2),
                3 => atk_queue.push_geo(data, &self.na_3),
                4 => atk_queue.push_geo(data, &self.na_4),
                5 => {
                    atk_queue.push_geo(data, &self.na_5);
                    atk_queue.push(ElementalAttack::geo(&self.na_last));
                },
                _ => (),
            };
        }
    }
}
