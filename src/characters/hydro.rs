use std::ptr;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Vision, FieldEnergy, VecFieldEnergy, Particle, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::fc::{FieldCharacterIndex, FieldAbilityBuilder, SpecialAbility, SkillAbility, CharacterData, CharacterRecord, Enemy};
use crate::action::{Attack, AttackEvent, ElementalAbsorption, NaLoop, SimpleSkill, SimpleSkillDot, SkillDamage2Dot, SimpleBurst, SimpleBurstDot, BurstDamage2Dot, NTimer, DurationTimer, ICDTimers};


use AttackType::*;
use WeaponType::*;
use Vision::*;

// version 1.0

pub struct Barbara {
    na: NaLoop,
}

impl Barbara {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Barbara").vision(Hydro).weapon(Catalyst).release_date("2020-09-28").version(1.0)
            .infusion(true)
            .base_hp(9787.0).base_atk(159.0).base_def(669.0)
            .hp(24.0)
            // .press_cd(32.0).press_particle(0.0).press_dmg(0.0)
            // .burst_cd(20.0).energy_cost(80.0).burst_dmg(0.0)
            .energy_cost(80.0)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            na: NaLoop::new(
                // 4 attacks in 1.5 seconds
                &[0.375,0.375,0.375,0.375],
                vec![
                    Attack::na(68.11, 1, idx),
                    Attack::na(63.94, 1, idx),
                    Attack::na(73.87, 1, idx),
                    Attack::na(99.36, 1, idx),
                ]
            ),
        }
    }

    pub fn build(&mut self, builder: &mut FieldAbilityBuilder) -> () {
        builder.na(&mut self.na).passive(self);
    }
}

impl SpecialAbility for Barbara {}

pub struct Xingqiu {
    na: NaLoop,
    skill: SimpleSkill,
    burst: SimpleBurstDot,
}

impl Xingqiu {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Xingqiu").vision(Hydro).weapon(Sword).release_date("2020-09-28").version(1.0)
            .base_hp(10222.0).base_atk(202.0).base_def(758.0)
            .atk(24.0)
            // a4
            .hydro_dmg(20.0)
            .energy_cost(80.0)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            na: NaLoop::new(
                // 5 attacks in 2.833 seconds
                &[0.5666,0.5666,0.5666,0.5666,0.5666],
                vec![
                    Attack::na(92.14, 1, idx),
                    Attack::na(94.18, 1, idx),
                    Attack::na(56.44, 2, idx),
                    Attack::na(110.67, 1, idx),
                    Attack::na(70.89, 2, idx),
                ]
            ),
            skill: SimpleSkill::new(&[21.0], Particle::new(Hydro, 4.0), Attack {
                kind: AttackType::PressSkill,
                element: &HYDRO_GAUGE1A,
                multiplier: (302.4 + 344.16) / 2.0,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            }),
            burst: SimpleBurstDot::new(&[1.233,1.233,1.233,1.233,1.233,1.233,1.233,1.233,1.233,1.233,1.233,1.233,1.233, 3.971], Attack {
                kind: AttackType::BurstDot,
                element: &HYDRO_GAUGE1A,
                multiplier: 103.12,
                hits: 3,
                icd_timer: ptr::null_mut(),
                idx,
            }),
        }
    }

    pub fn build(&mut self, builder: &mut FieldAbilityBuilder) -> () {
        builder.na(&mut self.na).skill(&mut self.skill).burst(&mut self.burst).passive(self);
    }
}

impl SpecialAbility for Xingqiu {}

pub struct Mona {
    once: bool,
    na: NaLoop,
    skill: SkillDamage2Dot,
    burst: SimpleBurst,
}

impl Mona {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Mona").vision(Hydro).weapon(Catalyst).release_date("2020-09-28").version(1.0)
            .infusion(true)
            .base_hp(10409.0).base_atk(287.0).base_def(653.0)
            .er(32.0)
            .energy_cost(60.0)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            once: true,
            na: NaLoop::new(
                // 4 attacks in 1.5 seconds
                &[0.375,0.375,0.375,0.375],
                vec![
                    Attack::na(67.68, 1, idx),
                    Attack::na(64.8, 1, idx),
                    Attack::na(80.64, 1, idx),
                    Attack::na(101.09, 1, idx),
                ]
            ),
            skill: SkillDamage2Dot::new(&[1.0,1.0,1.0,1.0,1.0, 7.0], Particle::new(Hydro, 3.0), Attack {
                kind: AttackType::PressSkill,
                element: &HYDRO_GAUGE1A,
                multiplier: 239.04,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            }, Attack {
                kind: AttackType::SkillDot,
                element: &HYDRO_GAUGE1A,
                multiplier: 57.6,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            }),
            burst: SimpleBurst::new(&[5.0, 10.0], Attack {
                kind: AttackType::Burst,
                element: &HYDRO_GAUGE2B,
                multiplier: 796.32,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            }),
        }
    }

    pub fn build(&mut self, builder: &mut FieldAbilityBuilder) -> () {
        builder.na(&mut self.na).skill(&mut self.skill).burst(&mut self.burst).passive(self);
    }
}

impl SpecialAbility for Mona {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        if self.once {
            self.once = false;
        }
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, enemy: &mut Enemy) -> () {
        if self.once {
            let state = &mut modifiable_state[data.idx.0];
            let er = 100.0 + state.er;
            // a4
            state.hydro_dmg += er * 0.2;
        }
        match (self.burst.timer.ping, self.burst.timer.n) {
            (true, 1) => for s in modifiable_state.iter_mut() {
                s.all_dmg += 60.0;;
            },
            (true, 2) => for s in modifiable_state.iter_mut() {
                s.all_dmg -= 60.0;;
            },
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.once = true;
    }
}
