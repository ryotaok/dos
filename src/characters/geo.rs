use std::rc::Rc;
use std::cell::RefCell;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Vision, FieldEnergy, VecFieldEnergy, Particle, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::fc::{FieldCharacterIndex, FieldAbilityBuilder, SpecialAbility, SkillAbility, CharacterData, CharacterRecord, Enemy};
use crate::action::{Attack, AttackEvent, ICDTimer, ElementalAbsorption, NaLoop, SimpleSkill, SimpleSkillDot, SkillDamage2Dot, SimpleBurst, SimpleBurstDot, BurstDamage2Dot, NTimer, DurationTimer, ICDTimers};
use crate::testutil;

use AttackType::*;
use WeaponType::*;
use Vision::*;

// version 1.0

#[derive(Debug)]
pub struct NingguangCa {
    star_jade: usize,
    na_count: usize,
    timer: NTimer,
    attack: Attack,
    ca_star_jade_1: Attack,
    ca_star_jade_2: Attack,
    ca_star_jade_3: Attack,
}

impl NingguangCa {
    pub fn new(idx: FieldCharacterIndex, icd_timer: &Rc<RefCell<ICDTimer>>) -> Self {
        Self {
            star_jade: 0,
            na_count: 0,
            timer: NTimer::with_condition(&[1.8]), // TODO 2.0?
            attack: Attack {
                kind: AttackType::Ca,
                element: &GEO_GAUGE1A,
                multiplier: 313.34,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            },
            ca_star_jade_1: Attack {
                kind: AttackType::Ca,
                element: &GEO_GAUGE1A,
                multiplier: 89.28,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            },
            ca_star_jade_2: Attack {
                kind: AttackType::Ca,
                element: &GEO_GAUGE1A,
                multiplier: 89.28,
                hits: 2,
                icd_timer: Rc::clone(icd_timer),
                idx,
            },
            ca_star_jade_3: Attack {
                kind: AttackType::Ca,
                element: &GEO_GAUGE1A,
                multiplier: 89.28,
                hits: 3,
                icd_timer: Rc::clone(icd_timer),
                idx,
            },
        }
    }
}

impl SpecialAbility for NingguangCa {
    fn init(&mut self, timers: &mut ICDTimers) -> () {
        self.attack.icd_timer = &mut timers.ca;
        self.ca_star_jade_1.icd_timer = &mut timers.ca;
        self.ca_star_jade_2.icd_timer = &mut timers.ca;
        self.ca_star_jade_3.icd_timer = &mut timers.ca;
    }

    fn maybe_attack(&self, _data: &CharacterData) -> Option<AttackEvent> {
        self.attack.to_event(&self.timer)
    }

    fn update(&mut self, time: f32, event: &AttackEvent, _data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        if event.idx == self.attack.idx {
            if event.kind == Na {
                self.star_jade += 1;
            } else if event.kind == Ca {
                self.star_jade = 0;
            }
        }
        self.timer.update(time, self.star_jade >= 1);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, _particles: &mut Vec<FieldEnergy>, _data: &CharacterData) -> () {
        match (self.timer.ping, self.timer.n, self.star_jade) {
            (true, 1, 0) => atk_queue.push(&self.attack),
            (true, 1, 1) => {
                atk_queue.push(&self.attack);
                atk_queue.push(&self.ca_star_jade_1);
            },
            (true, 1, 2) => {
                atk_queue.push(&self.attack);
                atk_queue.push(&self.ca_star_jade_2);
            },
            (true, 1, 3) => {
                atk_queue.push(&self.attack);
                atk_queue.push(&self.ca_star_jade_3);
            },
            _ => (),
        }
    }
}

pub struct Ningguang {
    na: NaLoop,
    ca: NingguangCa,
    skill: SimpleSkill,
    burst: SimpleBurst,
    burst_aa: Attack,
}

impl Ningguang {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Ningguang").vision(Geo).weapon(Catalyst).release_date("2020-09-28").version(1.0)
            .infusion(true)
            .base_hp(9787.0).base_atk(212.0).base_def(573.0)
            .geo_dmg(24.0)
            .energy_cost(40.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &Rc<RefCell<ICDTimer>>) -> Self {
        Self {
            na: NaLoop::new(
                // 1 attacks in 0.9 seconds
                &[0.9],
                vec![
                    Attack::na(50.4, 1, idx),
                ]
            ),
            ca: NingguangCa::new(idx),
            skill: SimpleSkill::new(&[10.0, 2.0], Particle::new(Geo, 3.5), Attack {
                kind: AttackType::PressSkill,
                element: &GEO_GAUGE1A,
                multiplier: 414.72,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }),
            burst: SimpleBurst::new(&[12.0], Attack {
                kind: AttackType::Burst,
                element: &GEO_GAUGE1A,
                multiplier: 156.53,
                hits: 6,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }),
            burst_aa: Attack {
                kind: AttackType::BurstDot,
                element: &GEO_GAUGE1A,
                multiplier: 156.53,
                hits: 4,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }
        }
    }

    pub fn build(&mut self, builder: &mut FieldAbilityBuilder) -> () {
        builder.na(&mut self.na).ca(&mut self.ca).skill(&mut self.skill).burst(&mut self.burst).passive(self);
    }
}

impl SpecialAbility for Ningguang {
    fn init(&mut self, timers: &mut ICDTimers) -> () {
        self.burst_aa.icd_timer = &mut timers.burst;
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        if self.burst.timer.ping && self.burst.timer.n == 1 && self.skill.timer.n == 1 {
            atk_queue.push(&self.burst_aa);
        }
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, enemy: &mut Enemy) -> () {
        // a4
        match (self.skill.timer.ping, self.skill.timer.n) {
            (true, 1) => for s in modifiable_state.iter_mut() {
                s.geo_dmg += 12.0;
            },
            (true, 2) => for s in modifiable_state.iter_mut() {
                s.geo_dmg -= 12.0;
            },
            _ => (),
        }
    }
}

pub struct Noelle {
    na: NaLoop,
    skill: SimpleSkill,
    burst: SimpleBurst,
}

impl Noelle {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Noelle").vision(Geo).weapon(Claymore).release_date("2020-09-28").version(1.0)
            .base_hp(12071.0).base_atk(191.0).base_def(799.0)
            .def(30.0)
            .energy_cost(60.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &Rc<RefCell<ICDTimer>>) -> Self {
        Self {
            na: NaLoop::new(
                // 4 attacks in 2.616 seconds
                &[0.654,0.654,0.654,0.654],
                vec![
                    Attack::na(156.4, 1, idx),
                    Attack::na(145.01, 1, idx),
                    Attack::na(170.51, 1, idx),
                    Attack::na(224.23, 1, idx),
                ]
            ),
            // a1
            skill: SimpleSkill::new(&[24.0], Particle::new(Geo, 0.0), Attack {
                kind: AttackType::PressSkill,
                element: &GEO_GAUGE2B,
                multiplier: 216.0,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }),
            burst: SimpleBurst::new(&[15.0], Attack {
                kind: AttackType::Burst,
                element: &GEO_GAUGE1A,
                multiplier: (120.96 + 167.76) / 2.0,
                hits: 2,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }),
        }
    }

    pub fn build(&mut self, builder: &mut FieldAbilityBuilder) -> () {
        builder.na(&mut self.na).skill(&mut self.skill).burst(&mut self.burst).passive(self);
    }
}

impl SpecialAbility for Noelle {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        // a4
        match (self.na.timer.ping, self.na.timer.n) {
            (true, 4) => self.skill.timer.update(1.0, false),
            _ => (),
        }
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, enemy: &mut Enemy) -> () {
        let state = &mut modifiable_state[data.idx.0];
        match (self.burst.timer.ping, self.burst.timer.n) {
            (true, 1) => {
                state.flat_atk += state.DEF() * 0.72;
                state.infusion = true;
            },
            (true, 0) => {
                state.flat_atk -= state.DEF() * 0.72;
                state.infusion = false;
            },
            _ => (),
        }
    }
}

pub struct TravelerGeo {
    na: NaLoop,
    na_last: Attack,
    skill: SimpleSkill,
    burst: SimpleBurst,
}

impl TravelerGeo {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Traveler (Geo)").vision(Geo).weapon(Sword).release_date("2020-09-28").version(1.0)
            .base_hp(10875.0).base_atk(212.0).base_def(683.0)
            .atk(24.0)
            .energy_cost(60.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &Rc<RefCell<ICDTimer>>) -> Self {
        Self {
            na: NaLoop::new(
                // 5 attacks in 2.55 seconds
                &[0.51,0.51,0.51,0.51,0.51],
                vec![
                    Attack::na(87.89, 1, idx),
                    Attack::na(85.85, 1, idx),
                    Attack::na(104.72, 1, idx),
                    Attack::na(115.26, 1, idx),
                    Attack::na(139.91, 1, idx),
                ]
            ),
            na_last: Attack {
                kind: AttackType::Na,
                element: &GEO_GAUGE1A,
                multiplier: 60.0,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            },
            skill: SimpleSkill::new(&[6.0], Particle::new(Geo, 3.5), Attack {
                kind: AttackType::PressSkill,
                element: &GEO_GAUGE2B,
                multiplier: 446.4,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }),
            burst: SimpleBurst::new(&[15.0], Attack {
                kind: AttackType::Burst,
                element: &GEO_GAUGE2B,
                multiplier: 266.4,
                hits: 4,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }),
        }
    }

    pub fn build(&mut self, builder: &mut FieldAbilityBuilder) -> () {
        builder.na(&mut self.na).skill(&mut self.skill).burst(&mut self.burst).passive(self);
    }
}

impl SpecialAbility for TravelerGeo {
    fn init(&mut self, timers: &mut ICDTimers) -> () {
        self.na_last.icd_timer = &mut timers.na;
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        match (self.na.timer.ping, self.na.timer.n) {
            (true, 5) => atk_queue.push(&self.na_last),
            _ => (),
        }
    }
}
