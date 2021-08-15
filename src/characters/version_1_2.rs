use std::ptr;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Vision, FieldEnergy, VecFieldEnergy, Particle, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::fc::{FieldCharacterIndex, FieldAbilityBuilder, SpecialAbility, SkillAbility, CharacterData, CharacterRecord, Enemy};
use crate::action::{Attack, AttackEvent, ElementalAbsorption, NaLoop, SimpleCa, SimpleSkill, SimpleSkillDot, SkillDamage2Dot, SkillDamage2DotParticle, SimpleBurst, SimpleBurstDot, BurstDamage2Dot, NTimer, DurationTimer, ICDTimers};
use crate::testutil;

use AttackType::*;
use WeaponType::*;
use Vision::*;

// version 1.2

pub struct Albedo {
    na: NaLoop,
    skill: SkillDamage2DotParticle,
    burst: BurstDamage2Dot,
}

impl Albedo {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Albedo").vision(Geo).weapon(Sword).release_date("2020-12-23").version(1.2)
            .base_hp(13226.0).base_atk(251.0).base_def(876.0)
            .geo_dmg(28.8)
            .energy_cost(40.0)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            na: NaLoop::new(
                // 5 attacks in 2.567 seconds
                &[0.5134,0.5134,0.5134,0.5134,0.5134],
                vec![
                    Attack::na(72.62, 1, idx),
                    Attack::na(72.62, 1, idx),
                    Attack::na(93.81, 1, idx),
                    Attack::na(98.35, 1, idx),
                    Attack::na(122.7, 1, idx),
                ]
            ),
            skill: SkillDamage2DotParticle::new(&[2.0,2.0,2.0,2.0,2.0], Particle::new(Geo, 0.8), Attack {
                kind: AttackType::PressSkill,
                element: &GEO_GAUGE1A,
                multiplier: 234.72,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            }, Attack {
                kind: AttackType::SkillDot,
                element: &GEO_GAUGE1A,
                multiplier: 240.48,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            }),
            burst: BurstDamage2Dot::new(&[0.5,0.5,0.5, 8.5, 2.0], Attack {
                kind: AttackType::Burst,
                element: &GEO_GAUGE1A,
                multiplier: 660.96,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            }, Attack {
                kind: AttackType::BurstDot,
                element: &GEO_GAUGE1A,
                multiplier: 129.6,
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

impl SpecialAbility for Albedo {
    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, enemy: &mut Enemy) -> () {
        if self.burst.timer.ping {
            // a4
            match self.burst.timer.n {
                1 => for s in modifiable_state.iter_mut() {
                    s.em += 125.0;
                },
                5 => for s in modifiable_state.iter_mut() {
                    s.em -= 125.0;
                },
                _ => (),
            }
        }
    }
}

pub struct Ganyu {
    ca: SimpleCa,
    skill: SimpleSkillDot,
    burst: SimpleBurstDot,
}

impl Ganyu {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Ganyu").vision(Cryo).weapon(Bow).release_date("2021-01-12").version(1.2)
            .base_hp(9797.0).base_atk(335.0).base_def(630.0)
            .cd(88.4)
            .energy_cost(60.0)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            ca: SimpleCa::new(0.0, 2.466, Attack {
                kind: AttackType::Ca,
                element: &CRYO_GAUGE1A,
                multiplier: 230.4 + 391.68,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            }),
            skill: SimpleSkillDot::new(&[5.0,5.0], Particle::new(Cryo, 2.0), Attack {
                kind: AttackType::SkillDot,
                element: &CRYO_GAUGE1A,
                multiplier: 237.6,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            }),
            burst: SimpleBurstDot::new(&[0.83,0.83,0.83,0.83,0.83,0.83,0.83,0.83,0.83,0.83,0.83,0.83,0.83,0.83,0.83,0.83,0.83,0.83,], Attack {
                kind: AttackType::BurstDot,
                element: &CRYO_GAUGE1A,
                multiplier: 126.49,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            }),
        }
    }

    pub fn build(&mut self, builder: &mut FieldAbilityBuilder) -> () {
        builder.ca(&mut self.ca).skill(&mut self.skill).burst(&mut self.burst).passive(self);
    }
}

impl SpecialAbility for Ganyu {
    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, enemy: &mut Enemy) -> () {
        if self.burst.timer.ping {
            // a4
            match self.burst.timer.n {
                1 => for s in modifiable_state.iter_mut() {
                    s.cryo_dmg += 20.0;
                },
                18 => for s in modifiable_state.iter_mut() {
                    s.cryo_dmg -= 20.0;
                },
                _ => (),
            }
        }
    }

    // a1
    fn intensify(&self, attack: &Attack) -> Option<State> {
        if self.ca.attack.most_eq(attack) {
            Some(State::new().cr(20.0))
        } else {
            None
        }
    }
}
