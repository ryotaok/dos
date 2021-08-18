use std::rc::Rc;
use std::cell::RefCell;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Vision, FieldEnergy, VecFieldEnergy, Particle, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::fc::{FieldCharacterIndex, SpecialAbility, SkillAbility, CharacterAbility, NoopAbility, CharacterData, CharacterRecord, Enemy, Debuff};
use crate::action::{Attack, AttackEvent, ICDTimer, ElementalAbsorption, NaLoop, SimpleSkill, SimpleSkillDot, SkillDamage2Dot, SimpleBurst, SimpleBurstDot, BurstDamage2Dot, NTimer, DurationTimer, StaminaTimer, ICDTimers};

use AttackType::*;
use WeaponType::*;
use Vision::*;

pub struct Yanfei {
    na_noop: NoopAbility,
    ca_noop: NoopAbility,
    scarlet_seal: usize,
    na: NaLoop,
    ca_0: Attack,
    ca_1: Attack,
    ca_2: Attack,
    ca_3: Attack,
    ca_4: Attack,
    a4_blazing_eye: Attack,
    ca_timer: NTimer,
    // TODO // stamina: StaminaTimer,
    skill: SimpleSkill,
    burst: SimpleBurst,
}

impl Yanfei {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Yanfei").vision(Pyro).weapon(Catalyst).release_date("2020-12-23").version(1.5)
            .base_hp(9352.0).base_atk(240.0).base_def(587.0)
            // a1
            .pyro_dmg(24.0 + 15.0)
            .energy_cost(80.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            na_noop: NoopAbility,
            ca_noop: NoopAbility,
            scarlet_seal: 0,
            na: NaLoop::new(
                // 3 attacks in 1.5 seconds
                &[0.5,0.5,0.5],
                vec![
                    Attack::na(105.01, 1, idx, &icd_timer),
                    Attack::na(93.83, 1, idx, &icd_timer),
                    Attack::na(136.82, 1, idx, &icd_timer),
                ]
            ),
            ca_0: Attack {
                kind: AttackType::Ca,
                element: &PYRO_GAUGE1A,
                multiplier: 159.99,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.ca),
                idx,
            },
            ca_1: Attack {
                kind: AttackType::Ca,
                element: &PYRO_GAUGE1A,
                multiplier: 188.22,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.ca),
                idx,
            },
            ca_2: Attack {
                kind: AttackType::Ca,
                element: &PYRO_GAUGE1A,
                multiplier: 216.46,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.ca),
                idx,
            },
            ca_3: Attack {
                kind: AttackType::Ca,
                element: &PYRO_GAUGE1A,
                multiplier: 244.69,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.ca),
                idx,
            },
            ca_4: Attack {
                kind: AttackType::Ca,
                element: &PYRO_GAUGE1A,
                multiplier: 272.92,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.ca),
                idx,
            },
            a4_blazing_eye: Attack {
                kind: AttackType::Ca,
                element: &PYRO_GAUGE1A,
                multiplier: 80.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.ca),
                idx,
            },
            ca_timer: NTimer::new(&[1.0]),
            skill: SimpleSkill::new(&[9.0], Particle::new(Pyro, 3.0), Attack {
                kind: AttackType::PressSkill,
                element: &PYRO_GAUGE1A,
                multiplier: 305.28,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }),
            burst: SimpleBurst::new(&[1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0, 5.0], Attack {
                kind: AttackType::Burst,
                element: &PYRO_GAUGE2B,
                multiplier: 328.32,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
        }
    }
}

impl CharacterAbility for Yanfei {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na_noop }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca_noop }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na_noop }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca_noop }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

impl SpecialAbility for Yanfei {
    fn maybe_attack(&self, data: &CharacterData) -> Option<AttackEvent> {
        match (self.scarlet_seal >= 3, self.ca_timer.n) {
            (true, 0) => Some(AttackEvent {
                kind: self.ca_0.kind,
                idx: self.ca_0.idx,
            }),
            _ => self.na.maybe_attack(data),
        }
    }

    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.ca_timer.update(time, event == &self.ca_0);
        self.na.update(time, event, data, attack, particles, enemy);
        if event.idx == self.burst.attack.idx {
            match &event.kind {
                Na => self.scarlet_seal += 1,
                Ca => self.scarlet_seal = 0,
                PressSkill => self.scarlet_seal += 3,
                _ => (),
            }
        }
        if self.burst.timer.ping && 0 < self.burst.timer.n && self.burst.timer.n <= 15 {
            self.scarlet_seal += 1;
        }
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        self.na.additional_attack(atk_queue, particles, data);
        if self.ca_timer.ping && self.ca_timer.n == 1 {
            atk_queue.push(&self.a4_blazing_eye);
            match self.scarlet_seal {
                0 => atk_queue.push(&self.ca_0),
                1 => atk_queue.push(&self.ca_1),
                2 => atk_queue.push(&self.ca_2),
                3 => atk_queue.push(&self.ca_3),
                4 => atk_queue.push(&self.ca_4),
                _ => (),
            }
        }
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if self.burst.timer.ping {
            match self.burst.timer.n {
                1 => {
                    let state = &mut modifiable_data[self.burst.attack.idx.0].state;
                    state.ca_dmg += 54.4;
                },
                16 => {
                    let state = &mut modifiable_data[self.burst.attack.idx.0].state;
                    state.ca_dmg -= 54.4;
                },
                _ => (),
            }
        }
    }

    fn reset(&mut self) -> () {
        self.scarlet_seal = 0;
    }
}

#[derive(Debug)]
pub struct EulaSkill {
    grimheart: usize,
    press_timer: NTimer,
    hold_timer: NTimer,
    press: Attack,
    hold: Attack,
    icewhirl_brand_1: Attack,
    icewhirl_brand_2: Attack,
    hold_a1: Attack,
    press_particle: Particle,
    hold_particle: Particle,
}

impl EulaSkill {
    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            grimheart: 0,
            press_timer: NTimer::new(&[4.0]),
            hold_timer: NTimer::new(&[10.0]),
            press: Attack {
                kind: AttackType::PressSkill,
                element: &CRYO_GAUGE1A,
                multiplier: 263.52,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            },
            hold: Attack {
                kind: AttackType::HoldSkill,
                element: &CRYO_GAUGE1A,
                multiplier: 442.08,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            },
            icewhirl_brand_1: Attack {
                kind: AttackType::SkillDot,
                element: &CRYO_GAUGE1A,
                multiplier: 172.8,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            },
            icewhirl_brand_2: Attack {
                kind: AttackType::SkillDot,
                element: &CRYO_GAUGE1A,
                multiplier: 172.8,
                hits: 2,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            },
            hold_a1: Attack {
                kind: AttackType::SkillDot,
                element: &PHYSICAL_GAUGE,
                multiplier: 725.56 * 0.5,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            },
            press_particle: Particle::new(Cryo, 1.5),
            hold_particle: Particle::new(Cryo, 2.5),
        }
    }
}

impl SkillAbility for EulaSkill {
    fn accelerate(&mut self, f: fn(&mut NTimer)) -> () {
        f(&mut self.press_timer);
        f(&mut self.hold_timer);
    }
}

impl SpecialAbility for EulaSkill {
    fn maybe_attack(&self, _data: &CharacterData) -> Option<AttackEvent> {
        if self.grimheart == 2 {
            self.hold.to_event(&self.hold_timer)
        } else {
            self.press.to_event(&self.press_timer)
        }
    }

    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        if event.idx == self.press.idx {
            match &event.kind {
                PressSkill => {
                    self.grimheart += 1;
                    self.press_timer.update(time, true);
                    self.hold_timer.update(time, false);
                },
                HoldSkill => {
                    self.grimheart = 0;
                    self.press_timer.update(time, false);
                    self.hold_timer.update(time, true);
                },
                Burst => {
                    self.grimheart += 1;
                    self.press_timer.reset();
                    self.hold_timer.reset();
                },
                _ => {
                    self.press_timer.update(time, false);
                    self.hold_timer.update(time, false);
                },
            }
        } else {
            self.press_timer.update(time, false);
            self.hold_timer.update(time, false);
        }
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, _data: &CharacterData) -> () {
        if self.press_timer.ping && self.press_timer.n == 1 {
            atk_queue.push(&self.press);
            particles.push_p(self.press_particle);
        }
        if self.hold_timer.ping && self.hold_timer.n == 1 {
            atk_queue.push(&self.hold);
            particles.push_p(self.hold_particle);
            match self.grimheart {
                1 => atk_queue.push(&self.icewhirl_brand_1),
                2 => {
                    atk_queue.push(&self.icewhirl_brand_2);
                    atk_queue.push(&self.hold_a1);
                },
                _ => (),
            }
        }
    }

    fn reset(&mut self) -> () {
        self.grimheart = 0;
        self.press_timer.reset();
        self.hold_timer.reset();
    }
}

pub struct Eula {
    lightfall_sword_stack: usize,
    na: NaLoop,
    ca: NoopAbility,
    skill: EulaSkill,
    burst: SimpleBurst,
    burst_lightfall_sword: Attack,
    burst_stack_n: Attack,
}

impl Eula {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Eula").vision(Cryo).weapon(Claymore).release_date("2021-01-12").version(1.5)
            .base_hp(13226.0).base_atk(342.0).base_def(751.0)
            .cd(88.4)
            .energy_cost(80.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            lightfall_sword_stack: 0,
            na: NaLoop::new(
                // 5 attacks in 3.85 seconds
                &[0.77,0.77,0.77,0.77,0.77],
                vec![
                    Attack::na(177.38, 1, idx, &icd_timer),
                    Attack::na(184.93, 1, idx, &icd_timer),
                    Attack::na(112.28, 2, idx, &icd_timer),
                    Attack::na(222.67, 1, idx, &icd_timer),
                    Attack::na(142.0, 2, idx, &icd_timer),
                ]
            ),
            ca: NoopAbility,
            skill: EulaSkill::new(idx, icd_timer),
            burst: SimpleBurst::new(&[7.0, 13.0], Attack {
                kind: AttackType::Burst,
                element: &CRYO_GAUGE2B,
                multiplier: 617.44,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
            burst_lightfall_sword: Attack {
                kind: AttackType::BurstDot,
                element: &PHYSICAL_GAUGE,
                multiplier: 725.56,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            },
            burst_stack_n: Attack {
                kind: AttackType::BurstDot,
                element: &PHYSICAL_GAUGE,
                multiplier: 0.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            },
        }
    }
}

impl CharacterAbility for Eula {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

impl SpecialAbility for Eula {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        // accumulate stacks
        if self.burst.timer.n == 1 {
            unsafe {
                for &a in attack {
                    let atk = & *a;
                    if atk.idx != data.idx {
                        continue;
                    }
                    match &atk.kind {
                        Na | Ca | PressSkill | HoldSkill | SkillDot | Burst => self.lightfall_sword_stack += atk.hits,
                        _ => (),
                    };
                }
            }
        }
        if self.burst.timer.ping && self.burst.timer.n == 2 {
            self.burst_stack_n.multiplier = 148.24 * self.lightfall_sword_stack as f32;
            self.lightfall_sword_stack = 0;
        }
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        if self.burst.timer.ping && self.burst.timer.n == 2 {
            atk_queue.push(&self.burst_lightfall_sword);
            atk_queue.push(&self.burst_stack_n);
        }
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if self.skill.hold_timer.ping && self.skill.hold_timer.n == 1 {
            enemy.element_res_debuff.push(Debuff::eula_cryo());
            enemy.physical_res_debuff.push(Debuff::eula_physical());
        }
    }

    fn reset(&mut self) -> () {
        self.lightfall_sword_stack = 0;
    }
}
