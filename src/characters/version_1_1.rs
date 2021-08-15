use std::rc::Rc;
use std::cell::RefCell;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Vision, FieldEnergy, VecFieldEnergy, Particle, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, GEO_GAUGE4C, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::fc::{FieldCharacterIndex, FieldAbilityBuilder, SpecialAbility, SkillAbility, CharacterData, CharacterRecord, Enemy};
use crate::action::{Attack, AttackEvent, ICDTimer, ElementalAbsorption, NaLoop, SimpleSkill, SimpleSkillDot, SkillDamage2Dot, SkillDamage2DotParticle, SimpleBurst, SimpleBurstDot, BurstDamage2Dot, NTimer, DurationTimer, ICDTimers};

use AttackType::*;
use WeaponType::*;
use Vision::*;

// version 1.1

pub struct Tartaglia {
    once: bool,
    na: NaLoop,
    riptide_slash: Attack,
    riptide_timer: NTimer,
    skill: SimpleSkill,
    burst: SimpleBurst,
}

impl Tartaglia {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Tartaglia").vision(Hydro).weapon(Bow).release_date("2020-11-11").version(1.1)
            .base_hp(13103.0).base_atk(301.0).base_def(815.0)
            .hydro_dmg(28.8)
            .energy_cost(60.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &Rc<RefCell<ICDTimer>>) -> Self {
        Self {
            once: true,
            na: NaLoop::new(
                // 6 attacks in 2.415 seconds
                &[0.4025,0.4025,0.4025,0.4025,0.4025,0.4025],
                vec![
                    Attack::na(76.84, 1, idx),
                    Attack::na(82.28, 1, idx),
                    Attack::na(111.35, 1, idx),
                    Attack::na(118.49, 1, idx),
                    Attack::na(109.31, 1, idx),
                    Attack::na((70.04+74.46) / 2.0, 2, idx),
                ]
            ),
            riptide_slash: Attack {
                kind: AttackType::SkillDot,
                element: &HYDRO_GAUGE1A,
                multiplier: 119.0,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            },
            riptide_timer: NTimer::new(&[1.5]),
            // a1
            skill: SimpleSkill::new(&[30.0, 45.0], Particle::new(Hydro, 10.0), Attack {
                kind: AttackType::PressSkill,
                element: &HYDRO_GAUGE1A,
                multiplier: 122.4,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }),
            burst: SimpleBurst::new(&[15.0], Attack {
                kind: AttackType::Burst,
                element: &HYDRO_GAUGE2B,
                multiplier: (835.2 + 216.0) / 2.0,
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

impl SpecialAbility for Tartaglia {
    fn init(&mut self, timers: &mut ICDTimers) -> () {
        self.riptide_slash.icd_timer = &mut timers.na;
    }
 
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        if self.once {
            self.once = false;
        }
        self.riptide_timer.update(time, event.idx == data.idx && event.kind == Na);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        match (self.riptide_timer.ping, self.riptide_timer.n) {
            (true, 1) => atk_queue.push(&self.riptide_slash),
            _ => (),
        }
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, enemy: &mut Enemy) -> () {
        if self.skill.timer.ping {
            let state = &mut modifiable_state[data.idx.0];
            match self.skill.timer.n {
                1 => state.infusion = true,
                2 => state.infusion = false,
                _ => (),
            }
        }
        if self.once {
            // Master of Weaponry
            for s in modifiable_state.iter_mut() {
                s.na_talent += 5.0;
            }
        }
    }

    fn reset(&mut self) -> () {
        self.once = true;
        self.riptide_timer.reset();
    }
}

pub struct Diona {
    na: NaLoop,
    skill: SimpleSkill,
    burst: BurstDamage2Dot,
}

impl Diona {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Diona").vision(Cryo).weapon(Bow).release_date("2020-11-11").version(1.1)
            .base_hp(9570.0).base_atk(212.0).base_def(601.0)
            .cryo_dmg(24.0)
            .energy_cost(80.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &Rc<RefCell<ICDTimer>>) -> Self {
        Self {
            na: NaLoop::new(
                // 5 attacks in 2.333 seconds
                &[0.4466,0.4466,0.4466,0.4466,0.4466],
                vec![
                    Attack::na(71.4, 1, idx),
                    Attack::na(66.3, 1, idx),
                    Attack::na(90.1, 1, idx),
                    Attack::na(85.0, 1, idx),
                    Attack::na(106.25, 1, idx),
                ]
            ),
            skill: SimpleSkill::new(&[15.0], Particle::new(Cryo, 4.5), Attack {
                kind: AttackType::PressSkill,
                element: &CRYO_GAUGE1A,
                multiplier: 75.46,
                hits: 5,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }),
            burst: BurstDamage2Dot::new(&[2.0,2.0,2.0,2.0,2.0,2.0, 8.0], Attack {
                kind: AttackType::Burst,
                element: &CRYO_GAUGE1A,
                multiplier: 144.0,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }, Attack {
                kind: AttackType::BurstDot,
                element: &CRYO_GAUGE1A,
                multiplier: 94.75,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }),
        }
    }

    pub fn build(&mut self, builder: &mut FieldAbilityBuilder) -> () {
        builder.na(&mut self.na).skill(&mut self.skill).burst(&mut self.burst).passive(self);
    }
}

impl SpecialAbility for Diona {}

pub struct Zhongli {
    na: NaLoop,
    skill: SkillDamage2DotParticle,
    burst: SimpleBurst,
}

impl Zhongli {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Zhongli").vision(Geo).weapon(Polearm).release_date("2020-12-01").version(1.1)
            .base_hp(14695.0).base_atk(251.0).base_def(738.0)
            .geo_dmg(28.8)
            .energy_cost(40.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &Rc<RefCell<ICDTimer>>) -> Self {
        Self {
            na: NaLoop::new(
                // 6 attacks in 2.925 seconds
                &[0.4875,0.4875,0.4875,0.4875,0.4875,0.4875],
                vec![
                    Attack::na(60.82, 1, idx),
                    Attack::na(61.58, 1, idx),
                    Attack::na(76.26, 1, idx),
                    Attack::na(84.88, 1, idx),
                    Attack::na((21.25*4.0) / 4.0, 4, idx),
                    Attack::na(107.73, 1, idx),
                ]
            ),
            skill: SkillDamage2DotParticle::new(&[2.0,2.0,2.0,2.0,2.0], Particle::new(Geo, 0.5), Attack {
                kind: AttackType::PressSkill,
                element: &GEO_GAUGE2B,
                multiplier: 28.8,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }, Attack {
                kind: AttackType::SkillDot,
                element: &GEO_GAUGE1A,
                multiplier: 57.6,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }),
            burst: SimpleBurst::new(&[12.0], Attack {
                kind: AttackType::Burst,
                element: &GEO_GAUGE4C,
                multiplier: 899.72,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }),
        }
    }

    pub fn build(&mut self, builder: &mut FieldAbilityBuilder) -> () {
        builder.na(&mut self.na).skill(&mut self.skill).burst(&mut self.burst).passive(self);
    }
}

impl SpecialAbility for Zhongli {}

pub struct Xinyan {
    na: NaLoop,
    skill: SkillDamage2Dot,
    burst: BurstDamage2Dot,
}

impl Xinyan {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Xinyan").vision(Pyro).weapon(Claymore).release_date("2020-12-01").version(1.1)
            .base_hp(11201.0).base_atk(249.0).base_def(799.0)
            .atk(24.0)
            .energy_cost(60.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &Rc<RefCell<ICDTimer>>) -> Self {
        Self {
            na: NaLoop::new(
                // 4 attacks in 2.8 seconds
                &[0.7,0.7,0.7,0.7],
                vec![
                    Attack::na(151.3, 1, idx),
                    Attack::na(146.2, 1, idx),
                    Attack::na(188.7, 1, idx),
                    Attack::na(228.99, 1, idx),
                ]
            ),
            skill: SkillDamage2Dot::new(&[2.0,2.0,2.0,2.0,2.0,2.0, 6.0], Particle::new(Pyro, 4.0), Attack {
                kind: AttackType::PressSkill,
                element: &PYRO_GAUGE1A,
                multiplier: 305.28,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }, Attack {
                kind: AttackType::SkillDot,
                element: &PYRO_GAUGE1A,
                multiplier: 60.48,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }),
            burst: BurstDamage2Dot::new(&[0.25,0.25,0.25,0.25, 14.0], Attack {
                kind: AttackType::Burst,
                element: &PHYSICAL_GAUGE,
                multiplier: 613.44,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }, Attack {
                kind: AttackType::BurstDot,
                element: &PYRO_GAUGE1A,
                multiplier: 72.0,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }),
        }
    }

    pub fn build(&mut self, builder: &mut FieldAbilityBuilder) -> () {
        builder.na(&mut self.na).skill(&mut self.skill).burst(&mut self.burst).passive(self);
    }
}

impl SpecialAbility for Xinyan {
    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, enemy: &mut Enemy) -> () {
        if self.skill.timer.ping && self.skill.timer.n == 1 {
            for s in modifiable_state.iter_mut() {
                s.physical_dmg += 15.0;
            }
        } else if self.skill.timer.ping && self.skill.timer.n == 7 {
            for s in modifiable_state.iter_mut() {
                s.physical_dmg -= 15.0;
            }
        }
    }
}
