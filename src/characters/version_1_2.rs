use std::rc::Rc;
use std::cell::RefCell;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Vision, FieldEnergy, VecFieldEnergy, Particle, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::fc::{FieldCharacterIndex, SpecialAbility, SkillAbility, CharacterAbility, NoopAbility, CharacterData, CharacterRecord, Enemy};
use crate::action::{Attack, AttackEvent, ICDTimer, ElementalAbsorption, NaLoop, SimpleCa, SimpleSkill, SimpleSkillDot, SkillDamage2Dot, SkillDamage2DotParticle, SimpleBurst, SimpleBurstDot, BurstDamage2Dot, NTimer, DurationTimer, ICDTimers};
use crate::testutil;

use AttackType::*;
use WeaponType::*;
use Vision::*;

// version 1.2

pub struct Albedo {
    na: NaLoop,
    ca: NoopAbility,
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

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            na: NaLoop::new(
                // 5 attacks in 2.567 seconds
                &[0.5134,0.5134,0.5134,0.5134,0.5134],
                vec![
                    Attack::na(72.62, 1, idx, &icd_timer),
                    Attack::na(72.62, 1, idx, &icd_timer),
                    Attack::na(93.81, 1, idx, &icd_timer),
                    Attack::na(98.35, 1, idx, &icd_timer),
                    Attack::na(122.7, 1, idx, &icd_timer),
                ]
            ),
            ca: NoopAbility,
            skill: SkillDamage2DotParticle::new(&[2.0,2.0,2.0,2.0,2.0], Particle::new(Geo, 0.8), Attack {
                kind: AttackType::PressSkill,
                element: &GEO_GAUGE1A,
                multiplier: 234.72,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }, Attack {
                kind: AttackType::SkillDot,
                element: &GEO_GAUGE1A,
                multiplier: 240.48,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }),
            burst: BurstDamage2Dot::new(&[0.5,0.5,0.5, 8.5, 2.0], Attack {
                kind: AttackType::Burst,
                element: &GEO_GAUGE1A,
                multiplier: 660.96,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }, Attack {
                kind: AttackType::BurstDot,
                element: &GEO_GAUGE1A,
                multiplier: 129.6,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
        }
    }
}

impl CharacterAbility for Albedo {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

impl SpecialAbility for Albedo {
    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        // a4
        if 1 <= self.burst.timer.n && self.burst.timer.n < 5 {
            for data in modifiable_data.iter_mut() {
                data.state.em += 125.0;
            }
        }
    }
}

pub struct Ganyu {
    na: NoopAbility,
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

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            na: NoopAbility,
            ca: SimpleCa::new(0.0, 2.466, Attack {
                kind: AttackType::Ca,
                element: &CRYO_GAUGE1A,
                multiplier: 230.4 + 391.68,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.ca),
                idx,
            }),
            skill: SimpleSkillDot::new(&[5.0,5.0], Particle::new(Cryo, 2.0), Attack {
                kind: AttackType::SkillDot,
                element: &CRYO_GAUGE1A,
                multiplier: 237.6,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }),
            burst: SimpleBurstDot::new(&[0.83,0.83,0.83,0.83,0.83,0.83,0.83,0.83,0.83,0.83,0.83,0.83,0.83,0.83,0.83,0.83,0.83,0.83,], Attack {
                kind: AttackType::BurstDot,
                element: &CRYO_GAUGE1A,
                multiplier: 126.49,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
        }
    }
}

impl CharacterAbility for Ganyu {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

impl SpecialAbility for Ganyu {
    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        // a4
        if 1 <= self.burst.timer.n && self.burst.timer.n < 18 {
            for data in modifiable_data.iter_mut() {
                data.state.cryo_dmg += 20.0;
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
