use std::rc::Rc;
use std::cell::RefCell;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Vision, FieldEnergy, VecFieldEnergy, Particle, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::fc::{FieldCharacterIndex, SpecialAbility, SkillAbility, CharacterAbility, NoopAbility, CharacterData, CharacterRecord, Enemy};
use crate::action::{Attack, AttackEvent, ICDTimer, ElementalAbsorption, NaLoop, SimpleSkill, SimpleSkillDot, SkillDamage2Dot, SimpleBurst, SimpleBurstDot, BurstDamage2Dot, NTimer, DurationTimer, ICDTimers};
use crate::testutil;

use AttackType::*;
use WeaponType::*;
use Vision::*;

// version 1.0

pub struct Chongyun {
    a4_timer: DurationTimer,
    na: NaLoop,
    ca: NoopAbility,
    skill: SimpleSkill,
    burst: SimpleBurst,
}

impl Chongyun {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Chongyun").vision(Cryo).weapon(Claymore).release_date("2020-09-28").version(1.0)
            .base_hp(10984.0).base_atk(223.0).base_def(648.0)
            .atk(24.0)
            .energy_cost(40.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            a4_timer: DurationTimer::new(8.0, &[0.0]),
            na: NaLoop::new(
                // 4 attacks in 2.834 seconds
                &[0.7085,0.7085,0.7085,0.7085],
                vec![
                    Attack::na(138.38, 1, idx, &icd_timer),
                    Attack::na(124.78, 1, idx, &icd_timer),
                    Attack::na(158.78, 1, idx, &icd_timer),
                    Attack::na(200.09, 1, idx, &icd_timer),
                ]
            ),
            ca: NoopAbility,
            skill: SimpleSkill::new(&[3.0, 7.0, 5.0], Particle::new(Cryo, 4.0), Attack {
                kind: AttackType::PressSkill,
                element: &CRYO_GAUGE2B,
                multiplier: 261.44,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }),
            burst: SimpleBurst::new(&[12.0], Attack {
                kind: AttackType::Burst,
                element: &CRYO_GAUGE1A,
                multiplier: 256.32,
                hits: 3,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
        }
    }
}

impl CharacterAbility for Chongyun {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

impl SpecialAbility for Chongyun {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.a4_timer.update(time, self.skill.timer.ping && self.skill.timer.n == 3);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        match (self.skill.timer.ping, self.skill.timer.n) {
            (true, 3) => atk_queue.push(&self.skill.attack),
            _ => (),
        }
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        // a1
        match (self.skill.timer.ping, self.skill.timer.n) {
            (true, 1) => for data in modifiable_data.iter_mut() {
                data.state.atk_spd += 8.0; // TODO only melee characters
            },
            (true, 3) => for data in modifiable_data.iter_mut() {
                data.state.atk_spd -= 8.0; // TODO only melee characters
            },
            _ => (),
        }
        // TODO Chongyun infusion
        let state = &mut modifiable_data[self.skill.attack.idx.0].state;
        match (self.skill.timer.ping, self.skill.timer.n) {
            (true, 1) => state.infusion = true,
            (true, 2) => state.infusion = false,
            _ => (),
        }
        if self.a4_timer.ping {
            match self.a4_timer.n {
                1 => enemy.debuff.cryo += 10.0,
                0 => enemy.debuff.cryo -= 10.0,
                _ => (),
            }
        }
    }
}

pub struct Kaeya {
    skill_a4: bool,
    na: NaLoop,
    ca: NoopAbility,
    skill: SimpleSkill,
    burst: SimpleBurstDot,
}

impl Kaeya {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Kaeya").vision(Cryo).weapon(Sword).release_date("2020-09-28").version(1.0)
            .base_hp(11636.0).base_atk(223.0).base_def(792.0)
            .er(26.7)
            .energy_cost(60.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            skill_a4: false,
            na: NaLoop::new(
                // 5 attacks in 2.734 seconds
                &[0.5468,0.5468,0.5468,0.5468,0.5468],
                vec![
                    Attack::na(106.25, 1, idx, &icd_timer),
                    Attack::na(102.17, 1, idx, &icd_timer),
                    Attack::na(129.03, 1, idx, &icd_timer),
                    Attack::na(140.08, 1, idx, &icd_timer),
                    Attack::na(174.42, 1, idx, &icd_timer),
                ]
            ),
            ca: NoopAbility,
            skill: SimpleSkill::new(&[6.0], Particle::new(Cryo, 2.5), Attack {
                kind: AttackType::PressSkill,
                element: &CRYO_GAUGE2B,
                multiplier: 344.16,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }),
            burst: SimpleBurstDot::new(&[0.6666,0.6666,0.6666,0.6666,0.6666,0.6666,0.6666,0.6666,0.6666,0.6666,0.6666,0.6666, 7.0008], Attack {
                kind: AttackType::BurstDot,
                element: &CRYO_GAUGE1A,
                multiplier: 139.92,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
        }
    }
}

impl CharacterAbility for Kaeya {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

impl SpecialAbility for Kaeya {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.skill_a4 = event == &self.skill.attack && enemy.aura.aura == Hydro;
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        if self.skill_a4 {
            particles.push_p(Particle::new(Cryo, 2.0));
        }
    }
}

pub struct Qiqi {
    na: NaLoop,
    ca: NoopAbility,
    skill: SkillDamage2Dot,
    burst: SimpleBurst,
}

impl Qiqi {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Qiqi").vision(Cryo).weapon(Sword).release_date("2020-09-28").version(1.0)
            .base_hp(12368.0).base_atk(287.0).base_def(922.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            na: NaLoop::new(
                // 5 attacks in 2.25 seconds
                &[0.45,0.45,0.45,0.45,0.45],
                vec![
                    Attack::na(74.63, 1, idx, &icd_timer),
                    Attack::na(76.84, 1, idx, &icd_timer),
                    Attack::na(47.77, 2, idx, &icd_timer),
                    Attack::na(48.79, 2, idx, &icd_timer),
                    Attack::na(124.61, 1, idx, &icd_timer),
                ]
            ),
            ca: NoopAbility,
            skill: SkillDamage2Dot::new(&[3.0,3.0,3.0,3.0, 18.0], Particle::new(Cryo, 0.0), Attack {
                kind: AttackType::PressSkill,
                element: &CRYO_GAUGE1A,
                multiplier: 172.8,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }, Attack {
                kind: AttackType::SkillDot,
                element: &CRYO_GAUGE1A,
                multiplier: 64.8,
                hits: 2,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }),
            burst: SimpleBurst::new(&[20.0], Attack {
                kind: AttackType::Burst,
                element: &CRYO_GAUGE2B,
                multiplier: 512.64,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
        }
    }
}

impl CharacterAbility for Qiqi {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

impl SpecialAbility for Qiqi {}
