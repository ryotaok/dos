
use crate::sim2::state::State;
use crate::sim2::types::{AttackType, WeaponType, Vision, FieldEnergy, VecFieldEnergy, Particle, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::sim2::fc::{FieldCharacterIndex, SpecialAbility, SkillAbility, CharacterAbility, NoopAbility, CharacterData, CharacterRecord, Enemy};
use crate::sim2::action::{Attack, AttackEvent, ICDTimer, ElementalAbsorption, NaLoop, SimpleSkill, SimpleSkillDot, SkillDamage2Dot, SimpleBurst, SimpleBurstDot, BurstDamage2Dot, NTimer, DurationTimer, ICDTimers};

use DamageType::*;
use WeaponType::*;
use Vision::*;

#[derive(Debug)]
pub struct Ayaka {
    once: bool,
    na: NaLoop,
    ca: NoopAbility,
    skill: SimpleSkill,
    burst: BurstDamage2Dot,
}

impl Ayaka {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Ayaka").vision(Cryo).weapon(Sword).version(2.0)
            .base_hp(12858.0).base_atk(342.0).base_def(784.0)
            .cd(88.4)
            .energy_cost(80.)
    }

    pub fn new() -> Self {
        Self {
            once: true,
            na: NaLoop::new(
                // 5 attacks in 2.117 seconds
                &[0.4234,0.4234,0.4234,0.4234,0.4234],
                vec![
                    Attack::na(90.39, 1, idx, &icd_timer),
                    Attack::na(96.24, 1, idx, &icd_timer),
                    Attack::na(123.79, 1, idx, &icd_timer),
                    Attack::na(44.77, 3, idx, &icd_timer),
                    Attack::na(154.55, 1, idx, &icd_timer),
                ]
            ),
            ca: NoopAbility,
            skill: SimpleSkill::new(&[6.0, 4.0], Particle::new(Cryo, 3.5), Attack {
                kind: DamageType::PressSkill,
                element: &CRYO_GAUGE2B,
                multiplier: 430.56,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }),
            burst: BurstDamage2Dot::new(&[0.3333,0.3333,0.3333,0.3333,0.3333,0.3333,0.3333,0.3333,0.3333,0.3333,0.3333,0.3333,0.3333,0.3333,0.3333, 15.0005], Attack {
                kind: DamageType::Burst,
                element: &CRYO_GAUGE1A,
                multiplier: 202.14,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }, Attack {
                kind: DamageType::BurstDot,
                element: &CRYO_GAUGE1A,
                multiplier: 303.21,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
        }
    }
}

impl CharacterAbility for Ayaka {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

impl CharacterAttack for Ayaka {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        let state = &mut modifiable_data[self.skill.attack.idx.0].state;
        // Alternate Sprint (Kamisato Art: Senho)
        state.infusion = true;
        state.cryo_dmg += 18.0;
        if self.skill.timer.n == 1 {
            state.na_dmg += 30.0;
            state.ca_dmg += 30.0;
        }
    }

    fn reset(&mut self) -> () {
    }
}

#[derive(Debug)]
pub struct Yoimiya {
    skill_a1: DurationTimer,
    na: NaLoop,
    ca: NoopAbility,
    skill: SimpleSkill,
    burst: BurstDamage2Dot,
}

impl Yoimiya {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Yoimiya").vision(Pyro).weapon(Bow).version(2.0)
            .base_hp(10164.0).base_atk(323.0).base_def(615.0)
            .cr(24.2)
            .energy_cost(60.)
    }

    pub fn new() -> Self {
        Self {
            skill_a1: DurationTimer::new(3.0, &[0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0]),
            na: NaLoop::new(
                // 5 attacks in 2.1 seconds
                &[0.42,0.42,0.42,0.42,0.42],
                vec![
                    Attack::na(63.59, 2, idx, &icd_timer),
                    Attack::na(121.99, 1, idx, &icd_timer),
                    Attack::na(158.59, 1, idx, &icd_timer),
                    Attack::na(82.82, 2, idx, &icd_timer),
                    Attack::na(188.87, 1, idx, &icd_timer),
                ]
            ),
            ca: NoopAbility,
            skill: SimpleSkill::new(&[10.0, 8.0], Particle::new(Pyro, 4.0), Attack {
                kind: DamageType::PressSkill,
                element: &PYRO_GAUGE1A,
                multiplier: 0.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }),
            burst: BurstDamage2Dot::new(&[2.0,2.0,2.0,2.0,2.0, 5.0], Attack {
                kind: DamageType::Burst,
                element: &PYRO_GAUGE2B,
                multiplier: 228.96,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }, Attack {
                kind: DamageType::BurstDot,
                element: &PYRO_GAUGE1A,
                multiplier: 219.6,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
        }
    }
}

impl CharacterAbility for Yoimiya {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

impl CharacterAttack for Yoimiya {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.skill_a1.update(time, self.skill.timer.n == 1 && event.idx == self.skill.attack.idx && event.kind == Na);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if self.skill_a1.n > 0 {
            let state = &mut modifiable_data[self.skill.attack.idx.0].state;
            state.pyro_dmg += 2.0 * self.skill_a1.n as f32;
        }
        if 1 <= self.burst.timer.n && self.burst.timer.n < 6 {
            for (i, data) in modifiable_data.iter_mut().enumerate() {
                if i != self.burst.attack.idx.0 {
                    data.state.atk += 20.0; // TODO should use skill_a1
                }
            }
        }
        if self.skill.timer.n == 1 {
            let state = &mut modifiable_data[self.skill.attack.idx.0].state;
            state.infusion = true;
            state.na_talent += 61.74;
        }
    }

    fn reset(&mut self) -> () {
        self.skill_a1.reset();
    }
}

#[derive(Debug)]
pub struct Sayu {
    na: NaLoop,
    ca: NoopAbility,
    skill: SimpleSkill,
    burst: BurstDamage2Dot,
    skill_ea: ElementalAbsorption,
}

impl Sayu {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Sayu").vision(Anemo).weapon(Claymore).version(2.0)
            .base_hp(11854.0).base_atk(244.0).base_def(745.0)
            .em(96.0)
            .energy_cost(80.)
    }

    pub fn new() -> Self {
        Self {
            na: NaLoop::new(
                // 4 attacks in 2.616 seconds
                &[0.654,0.654,0.654,0.654],
                vec![
                    Attack::na(142.8, 1, idx, &icd_timer),
                    Attack::na(141.1, 1, idx, &icd_timer),
                    Attack::na(85.85, 2, idx, &icd_timer),
                    Attack::na(193.97, 1, idx, &icd_timer),
                ]
            ),
            ca: NoopAbility,
            skill: SimpleSkill::new(&[6.0], Particle::new(Anemo, 3.5), Attack {
                kind: DamageType::PressSkill,
                element: &ANEMO_GAUGE1A,
                multiplier: 64.8 + 285.12,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }),
            skill_ea: ElementalAbsorption::new(idx, SkillDot, 30.24 + 137.09, NTimer::new(&[6.0]), icd_timer),
            burst: BurstDamage2Dot::new(&[2.0,2.0,2.0,2.0,2.0,2.0, 8.0], Attack {
                kind: DamageType::Burst,
                element: &ANEMO_GAUGE1A,
                multiplier: 210.24,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }, Attack {
                kind: DamageType::BurstDot,
                element: &ANEMO_GAUGE1A,
                multiplier: 93.6,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
        }
    }
}

impl CharacterAbility for Sayu {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

impl CharacterAttack for Sayu {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.skill_ea.absorb(time, event == &self.skill.attack, enemy);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        if self.skill.timer.ping && self.skill.timer.n == 1 {
            if let Some(a) = self.skill_ea.attack() {
                atk_queue.push(a);
            }
        }
    }
}
