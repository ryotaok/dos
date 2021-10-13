use std::rc::Rc;
use std::cell::RefCell;

use crate::sim1::state::State;
use crate::sim1::types::{AttackType, WeaponType, Vision, FieldEnergy, VecFieldEnergy, Particle, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::sim1::fc::{FieldCharacterIndex, SpecialAbility, SkillAbility, CharacterAbility, NoopAbility, CharacterData, CharacterRecord, Enemy};
use crate::sim1::action::{Attack, AttackEvent, ICDTimer, ElementalAbsorption, NaLoop, SimpleSkill, SimpleSkillDot, SkillDamage2Dot, SimpleBurst, SimpleBurstDot, BurstDamage2Dot, NTimer, DurationTimer, ICDTimers};
use crate::sim1::testutil;

use AttackType::*;
use WeaponType::*;
use Vision::*;

pub struct Rosaria {
    na: NaLoop,
    ca: NoopAbility,
    skill: SimpleSkill,
    burst: BurstDamage2Dot,
}

impl Rosaria {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Rosaria").vision(Cryo).weapon(Polearm).release_date("2020-12-23").version(1.4)
            .base_hp(12289.0).base_atk(240.0).base_def(710.0)
            .atk(24.0)
            .energy_cost(60.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            na: NaLoop::new(
                // 5 attacks in 2.733 seconds
                &[0.5466,0.5466,0.5466,0.5466,0.5466],
                vec![
                    Attack::na(103.7, 1, idx, &icd_timer),
                    Attack::na(102.0, 1, idx, &icd_timer),
                    Attack::na(62.9, 2, idx, &icd_timer),
                    Attack::na(137.7, 1, idx, &icd_timer),
                    Attack::na((82.28 + 85.0) / 2.0, 2, idx, &icd_timer),
                ]
            ),
            ca: NoopAbility,
            skill: SimpleSkill::new(&[5.0, 1.0], Particle::new(Cryo, 3.0), Attack {
                kind: AttackType::PressSkill,
                element: &CRYO_GAUGE1A,
                multiplier: (105.21 + 244.8) / 2.0,
                hits: 2,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }),
            burst: BurstDamage2Dot::new(&[2.0,2.0,2.0,2.0, 2.0, 5.0,], Attack {
                kind: AttackType::Burst,
                element: &CRYO_GAUGE1A,
                multiplier: (187.2 + 273.6) / 2.0,
                hits: 2,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }, Attack {
                kind: AttackType::BurstDot,
                element: &CRYO_GAUGE1A,
                multiplier: 237.6,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
        }
    }
}

impl CharacterAbility for Rosaria {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

impl SpecialAbility for Rosaria {
    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if self.skill.timer.n == 1 {
            let state = &mut modifiable_data[self.skill.attack.idx.0].state;
            state.cr += 12.0;
        }
        if 1 <= self.burst.timer.n && self.burst.timer.n < 6 {
            let cr = modifiable_data[self.skill.attack.idx.0].state.cr;
            for (i, data) in modifiable_data.iter_mut().enumerate() {
                if i != self.burst.attack.idx.0 {
                    data.state.cr += cr * 0.15;
                }
            }
        }
    }
}
