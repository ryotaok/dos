use std::rc::Rc;
use std::cell::RefCell;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Vision, FieldEnergy, VecFieldEnergy, Particle, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::fc::{FieldCharacterIndex, FieldAbilityBuilder, SpecialAbility, SkillAbility, CharacterData, CharacterRecord, Enemy};
use crate::action::{Attack, AttackEvent, ICDTimer, ElementalAbsorption, NaLoop, SimpleCa, SimpleSkill, SimpleSkillDot, SkillDamage2Dot, SimpleBurst, SimpleBurstDot, BurstDamage2Dot, Time, NTimer, DurationTimer, ICDTimers};
use crate::testutil;

use AttackType::*;
use WeaponType::*;
use Vision::*;

// version 1.0

pub struct Amber {
    ca_timer: DurationTimer,
    ca: SimpleCa,
    skill: SimpleSkill,
    burst: SimpleBurst,
}

impl Amber {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Amber").vision(Pyro).weapon(Bow).release_date("2020-09-28").version(1.0)
            .base_hp(9461.0).base_atk(223.0).base_def(601.0)
            .atk(24.0)
            .energy_cost(40.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            ca_timer: DurationTimer::new(10.0, &[0.0]),
            ca: SimpleCa::new(0.0, 2.0, Attack {
                kind: AttackType::Ca,
                element: &PYRO_GAUGE2B,
                multiplier: 223.2,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.ca),
                idx,
            }),
            skill: SimpleSkill::new(&[15.0], Particle::new(Pyro, 4.0), Attack {
                kind: AttackType::PressSkill,
                element: &PYRO_GAUGE2B,
                multiplier: 221.76,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }),
            burst: SimpleBurst::new(&[12.0], Attack {
                kind: AttackType::Burst,
                element: &PYRO_GAUGE1A,
                multiplier: 50.54,
                hits: 18,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
        }
    }

    pub fn build(&mut self, builder: &mut FieldAbilityBuilder) -> () {
        builder.ca(&mut self.ca).skill(&mut self.skill).burst(&mut self.burst).passive(self);
    }
}

impl SpecialAbility for Amber {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.ca_timer.update(time, event.idx == self.skill.attack.idx && event.kind == Ca);
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        let state = &mut modifiable_data[self.skill.attack.idx.0].state;
        // a4
        match (self.ca_timer.ping, self.ca_timer.n) {
            (true, 1) => state.atk += 15.0,
            (true, 0) => state.atk -= 15.0,
            _ => (),
        }
    }

    // a1
    fn intensify(&self, attack: &Attack) -> Option<State> {
        if self.burst.attack.most_eq(attack) {
            Some(State::new().cr(10.0))
        } else {
            None
        }
    }

    fn reset(&mut self) -> () {
        self.ca_timer.reset();
    }
}

pub struct Bennett {
    bonus: f32,
    na: NaLoop,
    skill: SimpleSkill,
    burst: SimpleBurst,
}

impl Bennett {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Bennett").vision(Pyro).weapon(Sword).release_date("2020-09-28").version(1.0)
            .base_hp(12397.0).base_atk(191.0).base_def(771.0)
            .er(26.7)
            .energy_cost(60.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            bonus: 1.008,
            na: NaLoop::new(
                // 5 attacks in 2.567 seconds
                &[0.5134,0.5134,0.5134,0.5134,0.5134],
                vec![
                    Attack::na(88.06, 1, idx, &icd_timer),
                    Attack::na(84.49, 1, idx, &icd_timer),
                    Attack::na(107.95, 1, idx, &icd_timer),
                    Attack::na(117.98, 1, idx, &icd_timer),
                    Attack::na(142.12, 1, idx, &icd_timer),
                ]
            ),
            // a1
            skill: SimpleSkill::new(&[5.0 * 0.8], Particle::new(Pyro, 2.0), Attack {
                kind: AttackType::PressSkill,
                element: &PYRO_GAUGE2B,
                multiplier: 261.44,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }),
            burst: SimpleBurst::new(&[12.0, 3.0], Attack {
                kind: AttackType::Burst,
                element: &PYRO_GAUGE2B,
                multiplier: 419.04,
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

impl SpecialAbility for Bennett {
    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        match (self.burst.timer.ping, self.burst.timer.n) {
            (true, 1) => for data in modifiable_data.iter_mut() {
                data.state.flat_atk += data.state.base_atk * self.bonus;
            },
            (true, 2) => for data in modifiable_data.iter_mut() {
                data.state.flat_atk -= data.state.base_atk * self.bonus;
            },
            _ => (),
        }
    }

    // TODO pass closure instead?
    // fn accelerate(&self, _na: &mut NormalAttackAction, skill: &mut SkillAction, _burst: &mut BurstAction) -> () {
    //     if self.burst_timer.is_active() {
    //         // a4
    //         skill.spd += 100.0;
    //     }
    // }
}

pub struct Xiangling {
    na: NaLoop,
    skill: SimpleSkillDot,
    burst: BurstDamage2Dot,
    skill_a4: DurationTimer,
}

impl Xiangling {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Xiangling").vision(Pyro).weapon(Polearm).release_date("2020-09-28").version(1.0)
            .base_hp(10875.0).base_atk(225.0).base_def(669.0)
            .em(96.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            na: NaLoop::new(
                // 5 attacks in 2.4 seconds
                &[0.48,0.48,0.48,0.48,0.48],
                vec![
                    Attack::na(83.13, 1, idx, &icd_timer),
                    Attack::na(83.3, 1, idx, &icd_timer),
                    Attack::na(103.02, 1, idx, &icd_timer),
                    Attack::na(111.52, 1, idx, &icd_timer),
                    Attack::na(140.42, 1, idx, &icd_timer),
                ]
            ),
            skill: SimpleSkillDot::new(&[2.0,2.0,2.0,2.0,4.0], Particle::new(Pyro, 1.0), Attack {
                kind: AttackType::SkillDot,
                element: &PYRO_GAUGE1A,
                multiplier: 200.3,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }),
            burst: BurstDamage2Dot::new(&[1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0, 10.0], Attack {
                kind: AttackType::Burst,
                element: &PYRO_GAUGE1A,
                multiplier: (129.6 + 158.4 + 197.28) / 3.0,
                hits: 3,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }, Attack {
                kind: AttackType::BurstDot,
                element: &PYRO_GAUGE1A,
                multiplier: 201.6,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
            skill_a4: DurationTimer::new(10.0, &[0.0]),
        }
    }

    pub fn build(&mut self, builder: &mut FieldAbilityBuilder) -> () {
        builder.na(&mut self.na).skill(&mut self.skill).burst(&mut self.burst).passive(self);
    }
}

impl SpecialAbility for Xiangling {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.skill_a4.update(time, self.skill.timer.ping && self.skill.timer.n == 5);
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        // a4
        let state = &mut modifiable_data[self.skill.attack.idx.0].state;
        match (self.skill_a4.ping, self.skill_a4.n) {
            (true, 1) => state.atk += 10.0,
            (true, 0) => state.atk -= 10.0,
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.skill_a4.reset();
    }
}

#[derive(Debug)]
pub struct DilucSkill {
    pub timer1: NTimer,
    pub timer2: NTimer,
    pub timer3: NTimer,
    pub attack1: Attack,
    pub attack2: Attack,
    pub attack3: Attack,
    pub particle: Particle,
}

impl DilucSkill {
    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            timer1: NTimer::new(&[10.0]),
            timer2: NTimer::new(&[10.0]),
            timer3: NTimer::new(&[10.0]),
            attack1: Attack {
                kind: AttackType::PressSkill,
                element: &PYRO_GAUGE1A,
                multiplier: 169.92,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            },
            attack2: Attack {
                kind: AttackType::PressSkill,
                element: &PYRO_GAUGE1A,
                multiplier: 175.68,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            },
            attack3: Attack {
                kind: AttackType::PressSkill,
                element: &PYRO_GAUGE1A,
                multiplier: 231.84,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            },
            particle: Particle::new(Pyro, 1.3333),
        }
    }
}

impl SkillAbility for DilucSkill {
    fn accelerate(&mut self, f: fn(&mut NTimer)) -> () {
        f(&mut self.timer1);
        f(&mut self.timer2);
        f(&mut self.timer3);
    }
}

impl SpecialAbility for DilucSkill {
    fn maybe_attack(&self, _data: &CharacterData) -> Option<AttackEvent> {
        self.attack1.to_event(&self.timer1)
            .or(self.attack2.to_event(&self.timer2))
            .or(self.attack3.to_event(&self.timer3))
    }

    fn update(&mut self, time: f32, event: &AttackEvent, _data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        let guard = event == &self.attack1;
        self.timer1.update(time, guard);
        self.timer2.update(time, guard);
        self.timer3.update(time, guard);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, _data: &CharacterData) -> () {
        match (self.timer1.ping, self.timer1.n, self.timer2.ping, self.timer2.n, self.timer3.ping, self.timer3.n, ) {
            (true, 1, _, _, _, _) => {
                atk_queue.push(&self.attack1);
                particles.push_p(self.particle);
            },
            (_, _, true, 1, _, _) => {
                atk_queue.push(&self.attack2);
                particles.push_p(self.particle);
            },
            (_, _, _, _, true, 1) => {
                atk_queue.push(&self.attack3);
                particles.push_p(self.particle);
            },
            _ => (),
        }
    }
}

pub struct Diluc {
    na: NaLoop,
    skill: DilucSkill,
    burst: BurstDamage2Dot,
}

impl Diluc {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Diluc").vision(Pyro).weapon(Claymore).release_date("2020-09-28").version(1.0)
            .base_hp(12981.0).base_atk(335.0).base_def(784.0)
            .cr(24.2)
            .energy_cost(40.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            na: NaLoop::new(
                // 4 attacks in 2.834 seconds
                &[0.7085,0.7085,0.7085,0.7085],
                vec![
                    Attack::na(177.31, 1, idx, &icd_timer),
                    Attack::na(173.23, 1, idx, &icd_timer),
                    Attack::na(195.33, 1, idx, &icd_timer),
                    Attack::na(264.86, 1, idx, &icd_timer),
                ]
            ),
            skill: DilucSkill::new(idx, icd_timer),
            burst: BurstDamage2Dot::new(&[0.5,0.5,0.5, 10.5], Attack {
                kind: AttackType::Burst,
                element: &PYRO_GAUGE1A,
                multiplier: 367.2,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }, Attack {
                kind: AttackType::BurstDot,
                element: &PYRO_GAUGE1A,
                multiplier: 114.0,
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

impl SpecialAbility for Diluc {
    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        let state = &mut modifiable_data[self.skill.attack1.idx.0].state;
        // a4
        match (self.burst.timer.ping, self.burst.timer.n) {
            (true, 1) => {
                state.pyro_dmg += 20.0;
                state.infusion = true;
            },
            (true, 0) => {
                state.pyro_dmg -= 20.0;
                state.infusion = false;
            },
            _ => (),
        }
    }
}

#[derive(Debug)]
pub struct KleeCa {
    timer: NTimer,
    attack: Attack,
    na_count: usize,
}

impl KleeCa {
    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            timer: NTimer::with_condition(&[1.5]),
            attack: Attack {
                kind: AttackType::Ca,
                element: &PYRO_GAUGE1A,
                multiplier: 283.25,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.ca),
                idx,
            },
            na_count: 0,
        }
    }
}

impl SpecialAbility for KleeCa {
    fn maybe_attack(&self, _data: &CharacterData) -> Option<AttackEvent> {
        self.attack.to_event(&self.timer)
    }

    fn update(&mut self, time: f32, event: &AttackEvent, _data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        if event.idx == self.attack.idx {
            if event.kind == Na {
                self.na_count += 1;
            } else if event.kind == PressSkill {
                self.na_count += 3;
            }
        }
        let should_update = self.na_count >= 3;
        self.timer.update(time, should_update);
        if should_update {
            self.na_count = 0;
        }
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, _particles: &mut Vec<FieldEnergy>, _data: &CharacterData) -> () {
        // TODO
        // match (self.timer.ping, self.timer.n, &self.timer.recovery) {
        match (self.timer.ping, self.timer.n) {
            (true, 1) => {
                atk_queue.push(&self.attack);
            },
            _ => (),
        }
    }
}

#[derive(Debug)]
pub struct KleeSkill {
    pub timer1: NTimer,
    pub timer2: NTimer,
    pub attack: Attack,
    pub dot: Attack,
    pub particle: Particle,
}

impl KleeSkill {
    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            timer1: NTimer::new(&[1.0,1.0, 18.0]),
            timer2: NTimer::new(&[1.0,1.0, 18.0]),
            attack: Attack {
                kind: AttackType::PressSkill,
                element: &PYRO_GAUGE2B,
                multiplier: 171.36,
                hits: 3,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            },
            dot: Attack {
                kind: AttackType::SkillDot,
                element: &PYRO_GAUGE1A,
                multiplier: 59.04,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            },
            particle: Particle::new(Pyro, 4.0),
        }
    }
}

impl SkillAbility for KleeSkill {
    fn accelerate(&mut self, f: fn(&mut NTimer)) -> () {
        f(&mut self.timer1);
        f(&mut self.timer2);
    }
}

impl SpecialAbility for KleeSkill {
    fn maybe_attack(&self, _data: &CharacterData) -> Option<AttackEvent> {
        self.attack.to_event(&self.timer1)
            .or(self.attack.to_event(&self.timer2))
    }

    fn update(&mut self, time: f32, event: &AttackEvent, _data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        let guard = event == &self.attack;
        self.timer1.update(time, guard);
        self.timer2.update(time, guard);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, _data: &CharacterData) -> () {
        match (self.timer1.ping, self.timer1.n) {
            (true, 1) => {
                atk_queue.push(&self.attack);
                atk_queue.push(&self.dot);
                particles.push_p(self.particle);
            },
            (true, 2) => atk_queue.push(&self.dot),
            _ => (),
        }
        match (self.timer2.ping, self.timer2.n) {
            (true, 1) => {
                atk_queue.push(&self.attack);
                atk_queue.push(&self.dot);
                particles.push_p(self.particle);
            },
            (true, 2) => atk_queue.push(&self.dot),
            _ => (),
        }
    }
}

pub struct Klee {
    na: NaLoop,
    ca: KleeCa,
    skill: KleeSkill,
    burst: SimpleBurstDot,
}

impl Klee {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Klee").vision(Pyro).weapon(Catalyst).release_date("2020-09-28").version(1.0)
            .infusion(true)
            .base_hp(10287.0).base_atk(311.0).base_def(615.0)
            .pyro_dmg(28.8)
            .energy_cost(60.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            na: NaLoop::new(
                // 3 attacks in 1.467 seconds
                &[0.489,0.489,0.489,],
                vec![
                    Attack::na(129.89, 1, idx, &icd_timer),
                    Attack::na(112.32, 1, idx, &icd_timer),
                    Attack::na(161.86, 1, idx, &icd_timer),
                ]
            ),
            ca: KleeCa::new(idx, icd_timer),
            skill: KleeSkill::new(idx, icd_timer),
            burst: SimpleBurstDot::new(&[1.0,1.0,1.0,1.0,1.0,1.0, 9.0], Attack {
                kind: AttackType::BurstDot,
                element: &PYRO_GAUGE1A,
                multiplier: 76.76,
                hits: 4,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
        }
    }

    pub fn build(&mut self, builder: &mut FieldAbilityBuilder) -> () {
        builder.na(&mut self.na).ca(&mut self.ca).skill(&mut self.skill).burst(&mut self.burst).passive(self);
    }
}

impl SpecialAbility for Klee {
    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        // a4
        if self.ca.timer.ping && self.ca.timer.n == 1 {
            particles.push_e(2.0 * data.state.cr / 100.0);
        }
    }

    // a1
    fn intensify(&self, attack: &Attack) -> Option<State> {
        if self.ca.attack.most_eq(attack) {
            Some(State::new().ca_dmg(50.0))
        } else {
            None
        }
    }
}
