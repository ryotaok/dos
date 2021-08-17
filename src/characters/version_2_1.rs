use std::rc::Rc;
use std::cell::RefCell;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Vision, FieldEnergy, VecFieldEnergy, Particle, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::fc::{FieldCharacterIndex, FieldAbilityBuilder, SpecialAbility, SkillAbility, CharacterData, CharacterRecord, Enemy};
use crate::action::{Attack, AttackEvent, ICDTimer, ElementalAbsorption, NaLoop, SimpleSkill, SimpleSkillDot, SkillDamage2Dot, SkillDamage2DotParticle, SimpleBurst, SimpleBurstDot, BurstDamage2Dot, NTimer, DurationTimer, ICDTimers};

use AttackType::*;
use WeaponType::*;
use Vision::*;

pub struct RaidenShogun {
    once: bool,
    resolve_stack: f32,
    musou_isshin_energy: DurationTimer,
    a1_timer: NTimer,
    na: NaLoop,
    skill: SkillDamage2DotParticle,
    burst: SimpleBurst,

}

impl RaidenShogun {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Raiden Shogun").vision(Electro).weapon(Polearm).release_date("2021-07-20").version(2.1)
            .base_hp(12907.0).base_atk(337.0).base_def(789.0)
            .er(32.0)
            .energy_cost(90.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            once: true,
            resolve_stack: 60.0, // TODO starting 200 energy consumption
            musou_isshin_energy: DurationTimer::new(7.0, &[1.0,1.0,1.0,1.0,1.0]),
            a1_timer: NTimer::new(&[3.0]),
            na: NaLoop::new(
                // 5 attacks in 2.117 seconds
                &[0.4234,0.4234,0.4234,0.4234,0.4234],
                vec![
                    Attack::na(78.37, 1, idx, &icd_timer),
                    Attack::na(78.54, 1, idx, &icd_timer),
                    Attack::na(98.6, 1, idx, &icd_timer),
                    Attack::na(57.29, 2, idx, &icd_timer),
                    Attack::na(129.37, 1, idx, &icd_timer),
                ]
            ),
            skill: SkillDamage2DotParticle::new(&[0.9,0.9,0.9,0.9,0.9,0.9,0.9,0.9,0.9,0.9,0.9,0.9], Particle::new(Electro, 0.5), Attack {
                kind: AttackType::PressSkill,
                element: &ELECTRO_GAUGE1A,
                multiplier: 210.96,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }, Attack {
                kind: AttackType::SkillDot,
                element: &ELECTRO_GAUGE1A,
                multiplier: 75.6,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }),
            burst: SimpleBurst::new(&[7.0, 11.0], Attack {
                kind: AttackType::Burst,
                element: &ELECTRO_GAUGE1A,
                multiplier: 721.44,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
        }
    }

    pub fn build(&mut self, builder: &mut FieldAbilityBuilder) -> () {
        builder.na(&mut self.na).skill(&mut self.skill).burst(&mut self.burst).passive(self);
    }
}

impl SpecialAbility for RaidenShogun {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        if self.once {
            self.once = false;
        }
        // self.a1_timer.update(time, particles.has_particles());
        // if self.a1_timer.ping && self.a1_timer.n > 0 {
        //     self.resolve_stack += 2.0;
        // }
        if self.burst.timer.n == 1 {
            self.musou_isshin_energy.update(time, event.idx == self.skill.attack.idx && event.kind == Na);
        }
        // if self.burst.timer.ping && self.burst.timer.n == 2 {
        //     self.resolve_stack = 0.0;
        //     self.musou_isshin_energy.reset();
        // }
        if self.burst.timer.ping {
            match self.burst.timer.n {
                1 => {
                    self.burst.attack.multiplier += 7.0 * self.resolve_stack;
                    let na = 1.31 * self.resolve_stack;
                    for a in self.na.attack.iter_mut() {
                        a.multiplier += na;
                    }
                },
                2 => {
                    self.burst.attack.multiplier -= 7.0 * self.resolve_stack;
                    let na = 1.31 * self.resolve_stack;
                    for a in self.na.attack.iter_mut() {
                        a.multiplier -= na;
                    }
                    self.resolve_stack = 0.0;
                    self.musou_isshin_energy.reset();
                },
                _ => (),
            }
        }
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        if self.musou_isshin_energy.ping && 0 < self.musou_isshin_energy.n && self.musou_isshin_energy.n <= 5 {
            let bonus = 1.0 + 0.6 * data.state.er / 100.0;
            particles.push_e(2.5 * bonus);
        }
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if self.once {
            // TODO Eye of Stormy Judgment
            let er = modifiable_data[self.skill.attack.idx.0].state.er;
            for data in modifiable_data.iter_mut() {
                data.state.burst_dmg += 0.3 * er;
            }
            // a4
            modifiable_data[self.skill.attack.idx.0].state.elemental_dmg += 0.4 * er;
        }
        if self.burst.timer.ping {
            let state = &mut modifiable_data[self.skill.attack.idx.0].state;
            match self.burst.timer.n {
                1 => {
                    state.infusion = true;
                },
                2 => {
                    state.infusion = false;
                },
                _ => (),
            }
        }
    }

    fn reset(&mut self) -> () {
        self.once = true;
        self.resolve_stack = 0.0;
        self.musou_isshin_energy.reset();
        self.a1_timer.reset();
    }
}

pub struct SangonomiyaKokomi {
    once: bool,
    na: NaLoop,
    skill: SimpleSkillDot,
    burst: SimpleBurst,
}

impl SangonomiyaKokomi {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Sangonomiya Kokomi").vision(Hydro).weapon(Catalyst).release_date("2021-08-10").version(2.1)
            .base_hp(12262.0).base_atk(226.0).base_def(628.0)
            // passive 2?
            .cr(-100.0)
            .hydro_dmg(28.8)
            .energy_cost(70.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            once: true,
            na: NaLoop::new(
                // 3 attacks in 1.5 seconds
                &[0.5,0.5,0.5],
                vec![
                    Attack::na(123.08, 1, idx, &icd_timer),
                    Attack::na(110.77, 1, idx, &icd_timer),
                    Attack::na(169.75, 1, idx, &icd_timer),
                ]
            ),
            skill: SimpleSkillDot::new(&[2.0,2.0,2.0,2.0,2.0,2.0, 8.0], Particle::new(Hydro, 0.75), Attack {
                kind: AttackType::SkillDot,
                element: &HYDRO_GAUGE1A,
                multiplier: 196.54,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }),
            burst: SimpleBurst::new(&[10.0, 8.0], Attack {
                kind: AttackType::Burst,
                element: &HYDRO_GAUGE1A,
                multiplier: 0.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
        }
    }

    pub fn build(&mut self, builder: &mut FieldAbilityBuilder) -> () {
        builder.na(&mut self.na).skill(&mut self.skill).burst(&mut self.burst).passive(self);
    }
}

impl SpecialAbility for SangonomiyaKokomi {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        if self.once {
            self.once = false;
            // TODO multiplier?
            self.burst.attack.multiplier = 0.1875 * data.state.HP();
        }
        if self.burst.timer.ping && self.burst.timer.n == 1 {
            self.skill.reset();
        }
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if self.burst.timer.ping {
            let state = &mut modifiable_data[self.skill.attack.idx.0].state;
            match self.burst.timer.n {
                1 => {
                    let hp = state.HP();
                    // passive 2
                    state.na_dmg += hp * 0.00871 + 0.15 * 25.0;
                    state.skill_dmg += hp * 0.0122 + 0.15 * 25.0;
                },
                2 => {
                    let hp = state.HP();
                    state.na_dmg -= hp * 0.00871 + 0.15 * 25.0;
                    state.skill_dmg -= hp * 0.0122 + 0.15 * 25.0;
                },
                _ => (),
            }
        }
    }

    fn reset(&mut self) -> () {
        self.once = true;
    }
}

pub struct KujouSara {
    bonus_timer: DurationTimer,
    na: NaLoop,
    skill: SimpleSkill,
    burst: BurstDamage2Dot,
}

impl KujouSara {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Kujou Sara").vision(Electro).weapon(Bow).release_date("2021-08-10").version(2.1)
            .base_hp(9570.0).base_atk(195.0).base_def(628.0)
            .atk(24.0)
            .energy_cost(80.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            bonus_timer: DurationTimer::new(6.0, &[0.0]),
            na: NaLoop::new(
                // 5 attacks in 2.1 seconds
                &[0.42,0.42,0.42,0.42,0.42],
                vec![
                    Attack::na(78.08, 1, idx, &icd_timer),
                    Attack::na(81.9, 1, idx, &icd_timer),
                    Attack::na(95.88, 1, idx, &icd_timer),
                    Attack::na(99.62, 1, idx, &icd_timer),
                    Attack::na(114.75, 1, idx, &icd_timer),
                ]
            ),
            skill: SimpleSkill::new(&[6.0, 4.0], Particle::new(Electro, 2.5), Attack {
                kind: AttackType::PressSkill,
                element: &ELECTRO_GAUGE1A,
                multiplier: 226.37,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }),
            burst: BurstDamage2Dot::new(&[0.5,0.5,0.5,0.5, 6.0, 12.0], Attack {
                kind: AttackType::Burst,
                element: &ELECTRO_GAUGE1A,
                multiplier: 737.28,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }, Attack {
                kind: AttackType::BurstDot,
                element: &ELECTRO_GAUGE1A,
                multiplier: 61.42,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
        }
    }

    pub fn build(&mut self, builder: &mut FieldAbilityBuilder) -> () {
        builder.na(&mut self.na).skill(&mut self.skill).burst(&mut self.burst).passive(self);
    }
}

impl SpecialAbility for KujouSara {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        let mut should_update = false;
        if self.burst.timer.ping && 0 < self.burst.timer.n && self.burst.timer.n <= 5 {
            should_update = true;
        }
        if self.skill.timer.ping && self.skill.timer.n == 1 {
            should_update = true;
        }
        self.bonus_timer.update(time, should_update);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        if self.skill.timer.ping && self.skill.timer.n == 1 {
            // a4
            let er = 100.0 + data.state.er;
            particles.push_e(0.012 * er);
        }
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if self.bonus_timer.ping {
            let base_atk = modifiable_data[self.burst.attack.idx.0].state.base_atk;
            match self.bonus_timer.n {
                1 => for data in modifiable_data.iter_mut() {
                    data.state.flat_atk += base_atk * 0.7733;
                },
                0 => for data in modifiable_data.iter_mut() {
                    data.state.flat_atk -= base_atk * 0.7733;
                },
                _ => (),
            }
        }
    }

    fn reset(&mut self) -> () {
        self.bonus_timer.reset();
    }
}

pub struct Aloy {
    skill_a4: DurationTimer,
    na: NaLoop,
    skill: SkillDamage2Dot,
    burst: SimpleBurst,
}

impl Aloy {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Aloy").vision(Cryo).weapon(Bow).release_date("2021-08-10").version(2.1)
            .base_hp(10899.0).base_atk(234.0).base_def(676.0)
            .cryo_dmg(28.8)
            .energy_cost(40.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            skill_a4: DurationTimer::new(1.5, &[1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0]),
            na: NaLoop::new(
                // 4 attacks in 1.6 seconds
                &[0.4,0.4,0.4,0.4],
                vec![
                    Attack::na((37.68 + 42.39) / 2.0, 2, idx, &icd_timer),
                    Attack::na(76.93, 1, idx, &icd_timer),
                    Attack::na(94.2, 1, idx, &icd_timer),
                    Attack::na(117.12, 1, idx, &icd_timer),
                ]
            ),
            skill: SkillDamage2Dot::new(&[1.0,1.0,1.0,1.0, 10.0, 6.0], Particle::new(Cryo, 4.0), Attack {
                kind: AttackType::PressSkill,
                element: &CRYO_GAUGE1A,
                multiplier: 319.68,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }, Attack {
                kind: AttackType::SkillDot,
                element: &CRYO_GAUGE1A,
                multiplier: 72.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }),
            burst: SimpleBurst::new(&[12.0], Attack {
                kind: AttackType::Burst,
                element: &CRYO_GAUGE1A,
                multiplier: 646.56,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
        }
    }

    pub fn build(&mut self, builder: &mut FieldAbilityBuilder) -> () {
        builder.na(&mut self.na).skill(&mut self.skill).burst(&mut self.burst).passive(self);
    }
}

impl SpecialAbility for Aloy {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.skill_a4.update(time, self.skill.timer.ping && self.skill.timer.n == 5);
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if self.skill_a4.ping {
            let state = &mut modifiable_data[self.skill.attack.idx.0].state;
            if 0 < self.skill_a4.n && self.skill_a4.n <= 10 {
                state.cryo_dmg += 3.5;
            } else {
                state.cryo_dmg -= 35.0;
            }
        }
        if self.skill.timer.ping {
            let state = &mut modifiable_data[self.skill.attack.idx.0].state;
            match self.skill.timer.n {
                1 | 2 | 3 | 4 => state.na_dmg += 9.52,
                5 => {
                    state.infusion = true;
                    for (i, data) in modifiable_data.iter_mut().enumerate() {
                        if i == self.burst.attack.idx.0 {
                            data.state.atk += 16.0;
                        } else {
                            data.state.atk += 8.0;
                        }
                    }
                },
                6 => {
                    state.infusion = false;
                    state.na_dmg -= 38.08;
                    for (i, data) in modifiable_data.iter_mut().enumerate() {
                        if i == self.burst.attack.idx.0 {
                            data.state.atk -= 16.0;
                        } else {
                            data.state.atk -= 8.0;
                        }
                    }
                },
                _ => (),
            }
        }
    }

    fn reset(&mut self) -> () {
        self.skill_a4.reset();
    }
}
