
use crate::sim2::state::State;
use crate::sim2::types::{AttackType, WeaponType, Vision, FieldEnergy, VecFieldEnergy, Particle, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, GEO_GAUGE4C, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::sim2::fc::{FieldCharacterIndex, SpecialAbility, SkillAbility, CharacterAbility, NoopAbility, CharacterData, CharacterRecord, Enemy};
use crate::sim2::action::{Attack, AttackEvent, ICDTimer, ElementalAbsorption, NaLoop, SimpleSkill, SimpleSkillDot, SkillDamage2Dot, SkillDamage2DotParticle, SimpleBurst, SimpleBurstDot, BurstDamage2Dot, NTimer, DurationTimer, ICDTimers};

use DamageType::*;
use WeaponType::*;
use Vision::*;

// version 1.1

#[derive(Debug)]
pub struct Tartaglia {
    once: bool,
    na: NaLoop,
    riptide_slash: Attack,
    riptide_timer: NTimer,
    ca: NoopAbility,
    skill: SimpleSkill,
    burst: SimpleBurst,
}

impl Tartaglia {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Tartaglia").vision(Hydro).weapon(Bow).version(1.1)
            .base_hp(13103.0).base_atk(301.0).base_def(815.0)
            .hydro_dmg(28.8)
            .energy_cost(60.)
    }

    pub fn new() -> Self {
        Self {
            once: true,
            na: NaLoop::new(
                // 6 attacks in 2.415 seconds
                &[0.4025,0.4025,0.4025,0.4025,0.4025,0.4025],
                vec![
                    Attack::na(76.84, 1, idx, &icd_timer),
                    Attack::na(82.28, 1, idx, &icd_timer),
                    Attack::na(111.35, 1, idx, &icd_timer),
                    Attack::na(118.49, 1, idx, &icd_timer),
                    Attack::na(109.31, 1, idx, &icd_timer),
                    Attack::na((70.04+74.46) / 2.0, 2, idx, &icd_timer),
                ]
            ),
            riptide_slash: Attack {
                kind: DamageType::SkillDot,
                element: &HYDRO_GAUGE1A,
                multiplier: 119.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            },
            riptide_timer: NTimer::new(&[1.5]),
            ca: NoopAbility,
            // a1
            skill: SimpleSkill::new(&[30.0, 45.0], Particle::new(Hydro, 10.0), Attack {
                kind: DamageType::PressSkill,
                element: &HYDRO_GAUGE1A,
                multiplier: 122.4,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }),
            burst: SimpleBurst::new(&[15.0], Attack {
                kind: DamageType::Burst,
                element: &HYDRO_GAUGE2B,
                multiplier: (835.2 + 216.0) / 2.0,
                hits: 2,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
        }
    }
}

impl CharacterAbility for Tartaglia {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

impl CharacterAttack for Tartaglia {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        if self.once {
            self.once = false;
        }
        self.riptide_timer.update(time, event.idx == self.skill.attack.idx && event.kind == Na);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        if self.riptide_timer.ping && self.riptide_timer.n == 1 {
            atk_queue.push(&self.riptide_slash);
        }
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if self.skill.timer.n == 1 {
            let state = &mut modifiable_data[self.skill.attack.idx.0].state;
            state.infusion = true;
        }
        if self.once {
            // Master of Weaponry
            for data in modifiable_data.iter_mut() {
                data.state.na_talent += 5.0;
            }
        }
    }

    fn reset(&mut self) -> () {
        self.once = true;
        self.riptide_timer.reset();
    }
}

#[derive(Debug)]
pub struct Diona {
    na: NaLoop,
    ca: NoopAbility,
    skill: SimpleSkill,
    burst: BurstDamage2Dot,
}

impl Diona {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Diona").vision(Cryo).weapon(Bow).version(1.1)
            .base_hp(9570.0).base_atk(212.0).base_def(601.0)
            .cryo_dmg(24.0)
            .energy_cost(80.)
    }

    pub fn new() -> Self {
        Self {
            na: NaLoop::new(
                // 5 attacks in 2.333 seconds
                &[0.4466,0.4466,0.4466,0.4466,0.4466],
                vec![
                    Attack::na(71.4, 1, idx, &icd_timer),
                    Attack::na(66.3, 1, idx, &icd_timer),
                    Attack::na(90.1, 1, idx, &icd_timer),
                    Attack::na(85.0, 1, idx, &icd_timer),
                    Attack::na(106.25, 1, idx, &icd_timer),
                ]
            ),
            ca: NoopAbility,
            skill: SimpleSkill::new(&[15.0], Particle::new(Cryo, 4.5), Attack {
                kind: DamageType::PressSkill,
                element: &CRYO_GAUGE1A,
                multiplier: 75.46,
                hits: 5,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }),
            burst: BurstDamage2Dot::new(&[2.0,2.0,2.0,2.0,2.0,2.0, 8.0], Attack {
                kind: DamageType::Burst,
                element: &CRYO_GAUGE1A,
                multiplier: 144.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }, Attack {
                kind: DamageType::BurstDot,
                element: &CRYO_GAUGE1A,
                multiplier: 94.75,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
        }
    }
}

impl CharacterAbility for Diona {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

impl CharacterAttack for Diona {}

#[derive(Debug)]
pub struct Zhongli {
    na: NaLoop,
    ca: NoopAbility,
    skill: SkillDamage2DotParticle,
    burst: SimpleBurst,
}

impl Zhongli {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Zhongli").vision(Geo).weapon(Polearm).version(1.1)
            .base_hp(14695.0).base_atk(251.0).base_def(738.0)
            .geo_dmg(28.8)
            .energy_cost(40.)
    }

    pub fn new() -> Self {
        Self {
            na: NaLoop::new(
                // 6 attacks in 2.925 seconds
                &[0.4875,0.4875,0.4875,0.4875,0.4875,0.4875],
                vec![
                    Attack::na(60.82, 1, idx, &icd_timer),
                    Attack::na(61.58, 1, idx, &icd_timer),
                    Attack::na(76.26, 1, idx, &icd_timer),
                    Attack::na(84.88, 1, idx, &icd_timer),
                    Attack::na((21.25*4.0) / 4.0, 4, idx, &icd_timer),
                    Attack::na(107.73, 1, idx, &icd_timer),
                ]
            ),
            ca: NoopAbility,
            skill: SkillDamage2DotParticle::new(&[2.0,2.0,2.0,2.0,2.0], Particle::new(Geo, 0.5), Attack {
                kind: DamageType::PressSkill,
                element: &GEO_GAUGE2B,
                multiplier: 28.8,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }, Attack {
                kind: DamageType::SkillDot,
                element: &GEO_GAUGE1A,
                multiplier: 57.6,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }),
            burst: SimpleBurst::new(&[12.0], Attack {
                kind: DamageType::Burst,
                element: &GEO_GAUGE4C,
                multiplier: 899.72,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
        }
    }
}

impl CharacterAbility for Zhongli {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

impl CharacterAttack for Zhongli {}

#[derive(Debug)]
pub struct Xinyan {
    na: NaLoop,
    ca: NoopAbility,
    skill: SkillDamage2Dot,
    burst: BurstDamage2Dot,
}

impl Xinyan {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Xinyan").vision(Pyro).weapon(Claymore).version(1.1)
            .base_hp(11201.0).base_atk(249.0).base_def(799.0)
            .atk(24.0)
            .energy_cost(60.)
    }

    pub fn new() -> Self {
        Self {
            na: NaLoop::new(
                // 4 attacks in 2.8 seconds
                &[0.7,0.7,0.7,0.7],
                vec![
                    Attack::na(151.3, 1, idx, &icd_timer),
                    Attack::na(146.2, 1, idx, &icd_timer),
                    Attack::na(188.7, 1, idx, &icd_timer),
                    Attack::na(228.99, 1, idx, &icd_timer),
                ]
            ),
            ca: NoopAbility,
            skill: SkillDamage2Dot::new(&[2.0,2.0,2.0,2.0,2.0,2.0, 6.0], Particle::new(Pyro, 4.0), Attack {
                kind: DamageType::PressSkill,
                element: &PYRO_GAUGE1A,
                multiplier: 305.28,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }, Attack {
                kind: DamageType::SkillDot,
                element: &PYRO_GAUGE1A,
                multiplier: 60.48,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }),
            burst: BurstDamage2Dot::new(&[0.25,0.25,0.25,0.25, 14.0], Attack {
                kind: DamageType::Burst,
                element: &PHYSICAL_GAUGE,
                multiplier: 613.44,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }, Attack {
                kind: DamageType::BurstDot,
                element: &PYRO_GAUGE1A,
                multiplier: 72.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
        }
    }
}

impl CharacterAbility for Xinyan {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

impl CharacterAttack for Xinyan {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if self.skill.timer.ping && 1 <= self.skill.timer.n && self.skill.timer.n < 7 {
            for data in modifiable_data.iter_mut() {
                data.state.physical_dmg += 15.0;
            }
        }
    }
}
