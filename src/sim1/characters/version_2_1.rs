use std::rc::Rc;
use std::cell::RefCell;

use crate::sim1::state::State;
use crate::sim1::types::{AttackType, WeaponType, Vision, FieldEnergy, VecFieldEnergy, Particle, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::sim1::fc::{FieldCharacterIndex, SpecialAbility, SkillAbility, CharacterAbility, NoopAbility, CharacterData, CharacterRecord, Enemy};
use crate::sim1::action::{Attack, AttackEvent, ICDTimer, ElementalAbsorption, NaLoop, SimpleSkill, SimpleSkillDot, SkillDamage2Dot, SkillDamage2DotParticle, SimpleBurst, SimpleBurstDot, BurstDamage2Dot, NTimer, DurationTimer, ICDTimers};

use AttackType::*;
use WeaponType::*;
use Vision::*;

pub struct RaidenShogun {
    once: bool,
    resolve_stack: f32,
    musou_isshin_energy: DurationTimer,
    a1_timer: NTimer,
    na: NaLoop,
    ca: NoopAbility,
    skill: SkillDamage2DotParticle,
    burst: SimpleBurst,
    na_burst: NaLoop,
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
        let na = NaLoop::new(
            // 5 attacks in 2.117 seconds
            &[0.4234,0.4234,0.4234,0.4234,0.4234],
            vec![
                Attack::na(78.37, 1, idx, &icd_timer),
                Attack::na(78.54, 1, idx, &icd_timer),
                Attack::na(98.6, 1, idx, &icd_timer),
                Attack::na(57.29, 2, idx, &icd_timer),
                Attack::na(129.37, 1, idx, &icd_timer),
            ]
        );
        let mut na_burst = na.clone();
        for a in na_burst.attack.iter_mut() {
            a.kind = BurstDot;
        }
        Self {
            once: true,
            resolve_stack: 40.0, // TODO starting 200 energy consumption
            musou_isshin_energy: DurationTimer::new(7.0, &[1.0,1.0,1.0,1.0,1.0]),
            a1_timer: NTimer::new(&[3.0]),
            na,
            ca: NoopAbility,
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
            na_burst,
        }
    }
}

impl CharacterAbility for RaidenShogun {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

impl SpecialAbility for RaidenShogun {
    fn maybe_attack(&self, data: &CharacterData) -> Option<AttackEvent> {
        if self.burst.timer.n == 1 {
            self.na_burst.maybe_attack(data)
        } else {
            self.na.maybe_attack(data)
        }
    }

    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        let speedup_time = time * (1.0 + data.state.atk_spd / 100.0);
        self.na.update(speedup_time, event, data, attack, particles, enemy);
        self.na_burst.update(speedup_time, event, data, attack, particles, enemy);
        self.a1_timer.update(time, particles.has_particles());
        if self.a1_timer.ping && self.a1_timer.n > 0 {
            self.resolve_stack += 2.0;
        }
        if self.burst.timer.n == 1 {
            self.musou_isshin_energy.update(time, event.idx == data.idx && (event.kind == Na || event.kind == BurstDot));
        }
        if self.burst.timer.ping {
            match self.burst.timer.n {
                1 => {
                    self.burst.attack.multiplier += 7.0 * self.resolve_stack;
                    let na = 1.31 * self.resolve_stack;
                    for a in self.na_burst.attack.iter_mut() {
                        a.multiplier += na;
                    }
                },
                2 => {
                    self.burst.attack.multiplier -= 7.0 * self.resolve_stack;
                    let na = 1.31 * self.resolve_stack;
                    for a in self.na_burst.attack.iter_mut() {
                        a.multiplier -= na;
                    }
                    self.resolve_stack = 30.0; // TODO gives 30 stacks regardless
                    self.musou_isshin_energy.reset();
                },
                _ => (),
            }
        }
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        self.na.additional_attack(atk_queue, particles, data);
        self.na_burst.additional_attack(atk_queue, particles, data);
        if self.musou_isshin_energy.ping && 0 < self.musou_isshin_energy.n && self.musou_isshin_energy.n <= 5 {
            let bonus = 1.0 + 0.6 * data.state.er / 100.0;
            particles.push_e(2.5 * bonus);
        }
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        // TODO Eye of Stormy Judgment
        for data in modifiable_data.iter_mut() {
            data.state.burst_dmg += 0.3 * data.character.energy_cost;
        }
        // a4
        let er = modifiable_data[self.skill.attack.idx.0].state.er;
        let state = &mut modifiable_data[self.skill.attack.idx.0].state;
        state.electro_dmg += 0.4 * er;
        if self.burst.timer.n == 1 {
            state.infusion = true;
        }
    }

    fn reset(&mut self) -> () {
        self.resolve_stack = 40.0;
        self.musou_isshin_energy.reset();
        self.a1_timer.reset();
        self.na_burst.attack[0].multiplier = self.na.attack[0].multiplier;
        self.na_burst.attack[1].multiplier = self.na.attack[1].multiplier;
        self.na_burst.attack[2].multiplier = self.na.attack[2].multiplier;
        self.na_burst.attack[3].multiplier = self.na.attack[3].multiplier;
        self.na_burst.attack[4].multiplier = self.na.attack[4].multiplier;
        self.burst.attack.multiplier = 721.44;
    }
}


pub struct KujouSara {
    bonus_timer: DurationTimer,
    na: NaLoop,
    ca: NoopAbility,
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
            ca: NoopAbility,
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
}

impl CharacterAbility for KujouSara {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
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
        if self.bonus_timer.n == 1 {
            let base_atk = modifiable_data[self.burst.attack.idx.0].state.base_atk;
            for data in modifiable_data.iter_mut() {
                data.state.flat_atk += base_atk * 0.7733;
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
    ca: NoopAbility,
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
    ca: NoopAbility,
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
}

impl CharacterAbility for Aloy {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

impl SpecialAbility for Aloy {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.skill_a4.update(time, self.skill.timer.ping && self.skill.timer.n == 5);
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if 0 < self.skill_a4.n && self.skill_a4.n <= 10 {
            let state = &mut modifiable_data[self.skill.attack.idx.0].state;
            state.cryo_dmg += 3.5 * self.skill_a4.n as f32;
        }
        if 1 <= self.skill.timer.n && self.skill.timer.n <= 5 {
            let state = &mut modifiable_data[self.skill.attack.idx.0].state;
            state.na_dmg += 9.52 * self.skill.timer.n as f32;
            if self.skill.timer.n == 5 {
                state.infusion = true;
                for (i, data) in modifiable_data.iter_mut().enumerate() {
                    if i == self.burst.attack.idx.0 {
                        data.state.atk += 16.0;
                    } else {
                        data.state.atk += 8.0;
                    }
                }
            }
        }
    }

    fn reset(&mut self) -> () {
        self.skill_a4.reset();
    }
}

pub struct SangonomiyaKokomi {
    once: bool,
    na: NaLoop,
    ca: NoopAbility,
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
            ca: NoopAbility,
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
}

impl CharacterAbility for SangonomiyaKokomi {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

impl SpecialAbility for SangonomiyaKokomi {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        if self.once {
            self.once = false;
            // TODO multiplier?
            self.burst.attack.multiplier = 0.01875 * data.state.HP();
        }
        if self.burst.timer.ping && self.burst.timer.n == 1 {
            self.skill.reset();
        }
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if self.burst.timer.n == 1 {
            let state = &mut modifiable_data[self.skill.attack.idx.0].state;
            let hp = state.HP();
            // passive 2
            state.flat_atk += hp * 0.0871 + 0.15 * 25.0;
        }
    }

    fn reset(&mut self) -> () {
        self.once = true;
    }
}
