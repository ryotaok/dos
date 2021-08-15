use std::ptr;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Vision, FieldEnergy, VecFieldEnergy, Particle, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::fc::{FieldCharacterIndex, FieldAbilityBuilder, SpecialAbility, SkillAbility, CharacterData, CharacterRecord, Enemy};
use crate::action::{Attack, AttackEvent, ElementalAbsorption, NaLoop, SimpleSkill, SimpleSkillDot, SkillDamage2Dot, SimpleBurst, SimpleBurstDot, BurstDamage2Dot, NTimer, DurationTimer, ICDTimers};

use AttackType::*;
use WeaponType::*;
use Vision::*;

pub struct Ayaka {
    once: bool,
    na: NaLoop,
    skill: SimpleSkill,
    burst: BurstDamage2Dot,
}

impl Ayaka {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Ayaka").vision(Cryo).weapon(Sword).release_date("2021-07-20").version(2.0)
            .base_hp(12858.0).base_atk(342.0).base_def(784.0)
            .cd(88.4)
            .energy_cost(80.0)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            once: true,
            na: NaLoop::new(
                // 5 attacks in 2.117 seconds
                &[0.4234,0.4234,0.4234,0.4234,0.4234],
                vec![
                    Attack::na(90.39, 1, idx),
                    Attack::na(96.24, 1, idx),
                    Attack::na(123.79, 1, idx),
                    Attack::na(44.77, 3, idx),
                    Attack::na(154.55, 1, idx),
                ]
            ),
            skill: SimpleSkill::new(&[6.0, 4.0], Particle::new(Cryo, 3.5), Attack {
                kind: AttackType::PressSkill,
                element: &CRYO_GAUGE2B,
                multiplier: 430.56,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            }),
            burst: BurstDamage2Dot::new(&[0.3333,0.3333,0.3333,0.3333,0.3333,0.3333,0.3333,0.3333,0.3333,0.3333,0.3333,0.3333,0.3333,0.3333,0.3333, 15.0005], Attack {
                kind: AttackType::Burst,
                element: &CRYO_GAUGE1A,
                multiplier: 202.14,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            }, Attack {
                kind: AttackType::BurstDot,
                element: &CRYO_GAUGE1A,
                multiplier: 303.21,
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

impl SpecialAbility for Ayaka {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        if self.once {
            self.once = false;
        }
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, enemy: &mut Enemy) -> () {
        if self.once {
            let state = &mut modifiable_state[data.idx.0];
            // Alternate Sprint (Kamisato Art: Senho)
            state.infusion = true;
            state.cryo_dmg += 18.0;
        }
        if self.skill.timer.ping {
            let state = &mut modifiable_state[data.idx.0];
            match self.skill.timer.n {
                1 => {
                    state.na_dmg += 30.0;
                    state.ca_dmg += 30.0;
                },
                2 => {
                    state.na_dmg -= 30.0;
                    state.ca_dmg -= 30.0;
                },
                _ => (),
            }
        }
    }

    fn reset(&mut self) -> () {
        self.once = true;
    }
}

pub struct Yoimiya {
    skill_a1: DurationTimer,
    na: NaLoop,
    skill: SimpleSkill,
    burst: BurstDamage2Dot,
}

impl Yoimiya {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Yoimiya").vision(Pyro).weapon(Bow).release_date("2021-08-10").version(2.0)
            .base_hp(10164.0).base_atk(323.0).base_def(615.0)
            .cr(24.2)
            .energy_cost(60.0)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            skill_a1: DurationTimer::new(3.0, &[0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0]),
            na: NaLoop::new(
                // 5 attacks in 2.1 seconds
                &[0.42,0.42,0.42,0.42,0.42],
                vec![
                    Attack::na(63.59, 2, idx),
                    Attack::na(121.99, 1, idx),
                    Attack::na(158.59, 1, idx),
                    Attack::na(82.82, 2, idx),
                    Attack::na(188.87, 1, idx),
                ]
            ),
            skill: SimpleSkill::new(&[10.0, 8.0], Particle::new(Pyro, 4.0), Attack {
                kind: AttackType::PressSkill,
                element: &PYRO_GAUGE1A,
                multiplier: 0.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            }),
            burst: BurstDamage2Dot::new(&[2.0,2.0,2.0,2.0,2.0, 5.0], Attack {
                kind: AttackType::Burst,
                element: &PYRO_GAUGE2B,
                multiplier: 228.96,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            }, Attack {
                kind: AttackType::BurstDot,
                element: &PYRO_GAUGE1A,
                multiplier: 219.6,
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

impl SpecialAbility for Yoimiya {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.skill_a1.update(time, self.skill.timer.n == 1 && event.idx == data.idx && event.kind == Na);
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, enemy: &mut Enemy) -> () {
        if self.skill_a1.ping {
            let state = &mut modifiable_state[data.idx.0];
            if self.skill_a1.n > 0 {
                state.pyro_dmg += 2.0;
            } else {
                state.pyro_dmg -= 2.0 * self.skill_a1.previous_n as f32;
            }
        }
        if self.burst.timer.ping {
            match self.burst.timer.n {
                1 => for (i, s) in modifiable_state.iter_mut().enumerate() {
                    if i != data.idx.0 {
                        s.atk += 20.0; // TODO should use skill_a1
                    }
                },
                0 => for (i, s) in modifiable_state.iter_mut().enumerate() {
                    if i != data.idx.0 {
                        s.atk -= 20.0; // TODO should use skill_a1
                    }
                },
                _ => (),
            }
        }
        if self.skill.timer.ping {
            let state = &mut modifiable_state[data.idx.0];
            match self.skill.timer.n {
                1 => {
                    state.infusion = true;
                    state.na_talent += 61.74;
                },
                2 => {
                    state.infusion = false;
                    state.na_talent -= 61.74;
                },
                _ => (),
            }
        }
    }

    fn reset(&mut self) -> () {
        self.skill_a1.reset();
    }
}

pub struct Sayu {
    na: NaLoop,
    skill: SimpleSkill,
    burst: BurstDamage2Dot,
    skill_ea: ElementalAbsorption,
}

impl Sayu {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Sayu").vision(Anemo).weapon(Claymore).release_date("2021-08-10").version(2.0)
            .base_hp(11854.0).base_atk(244.0).base_def(745.0)
            .em(96.0)
            .energy_cost(80.0)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            na: NaLoop::new(
                // 4 attacks in 2.616 seconds
                &[0.654,0.654,0.654,0.654],
                vec![
                    Attack::na(142.8, 1, idx),
                    Attack::na(141.1, 1, idx),
                    Attack::na(85.85, 2, idx),
                    Attack::na(193.97, 1, idx),
                ]
            ),
            skill: SimpleSkill::new(&[6.0], Particle::new(Anemo, 3.5), Attack {
                kind: AttackType::PressSkill,
                element: &ANEMO_GAUGE1A,
                multiplier: 64.8 + 285.12,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            }),
            skill_ea: ElementalAbsorption::new(idx, SkillDot, 30.24 + 137.09, NTimer::new(&[6.0])),
            burst: BurstDamage2Dot::new(&[2.0,2.0,2.0,2.0,2.0,2.0, 8.0], Attack {
                kind: AttackType::Burst,
                element: &ANEMO_GAUGE1A,
                multiplier: 210.24,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            }, Attack {
                kind: AttackType::BurstDot,
                element: &ANEMO_GAUGE1A,
                multiplier: 93.6,
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

impl SpecialAbility for Sayu {
    fn init(&mut self, timers: &mut ICDTimers) -> () {
        *self.skill_ea.icd() = &mut timers.skill;
    }

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
