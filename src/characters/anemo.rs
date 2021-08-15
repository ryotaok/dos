use std::rc::Rc;
use std::cell::RefCell;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Vision, FieldEnergy, VecFieldEnergy, Particle, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::fc::{FieldCharacterIndex, FieldAbilityBuilder, SpecialAbility, SkillAbility, CharacterData, CharacterRecord, Enemy};
use crate::action::{Attack, AttackEvent, ICDTimer, ElementalAbsorption, NaLoop, SimpleSkill, SimpleSkillDot, SkillDamage2Dot, SimpleBurst, SimpleBurstDot, BurstDamage2Dot, NTimer, DurationTimer, ICDTimers};
use crate::testutil;

use AttackType::*;
use WeaponType::*;
use Vision::*;

// version 1.0

#[derive(Debug)]
pub struct SucroseSkill {
    pub timer1: NTimer,
    pub timer2: NTimer,
    pub attack1: Attack,
    pub attack2: Attack,
    pub particle: Particle,
}

impl SucroseSkill {
    pub fn new(idx: FieldCharacterIndex, icd_timer: &Rc<RefCell<ICDTimer>>) -> Self {
        Self {
            timer1: NTimer::new(&[15.0]),
            timer2: NTimer::new(&[15.0]),
            attack1: Attack {
                kind: AttackType::PressSkill,
                element: &ANEMO_GAUGE1A,
                multiplier: 380.16,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            },
            attack2: Attack {
                kind: AttackType::PressSkill,
                element: &ANEMO_GAUGE1A,
                multiplier: 380.16,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            },
            particle: Particle::new(Anemo, 4.0),
        }
    }
}

impl SkillAbility for SucroseSkill {
    fn accelerate(&mut self, f: fn(&mut NTimer)) -> () {
        f(&mut self.timer1);
        f(&mut self.timer2);
    }
}

impl SpecialAbility for SucroseSkill {
    fn init(&mut self, timers: &mut ICDTimers) -> () {
        self.attack1.icd_timer = &mut timers.skill;
        self.attack2.icd_timer = &mut timers.skill;
    }

    fn maybe_attack(&self, _data: &CharacterData) -> Option<AttackEvent> {
        self.attack1.to_event(&self.timer1)
            .or(self.attack2.to_event(&self.timer2))
    }

    fn update(&mut self, time: f32, event: &AttackEvent, _data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        let guard = event == &self.attack1;
        self.timer1.update(time, guard);
        self.timer2.update(time, guard);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, _data: &CharacterData) -> () {
        match (self.timer1.ping, self.timer1.n, self.timer2.ping, self.timer2.n) {
            (true, 1, _, _) => {
                atk_queue.push(&self.attack1);
                particles.push_p(self.particle);
            },
            (_, _, true, 1) => {
                atk_queue.push(&self.attack2);
                particles.push_p(self.particle);
            },
            _ => (),
        }
    }
}

pub struct Sucrose {
    skill_a1: DurationTimer,
    skill_a4: DurationTimer,
    na: NaLoop,
    skill: SucroseSkill,
    burst: SimpleBurstDot,
    burst_ea: ElementalAbsorption,
}

impl Sucrose {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Sucrose").vision(Anemo).weapon(Catalyst).release_date("2020-09-28").version(1.0)
            .base_hp(9244.0).base_atk(170.0).base_def(703.0)
            .anemo_dmg(24.0)
            .energy_cost(80.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &Rc<RefCell<ICDTimer>>) -> Self {
        Self {
            skill_a1: DurationTimer::new(8.0, &[0.0]),
            skill_a4: DurationTimer::new(8.0, &[0.0]),
            na: NaLoop::new(
                // 4 attacks in 1.5 seconds
                &[0.375,0.375,0.375,0.375],
                vec![
                    Attack::na(60.24, 1, idx),
                    Attack::na(55.11, 1, idx),
                    Attack::na(69.21, 1, idx),
                    Attack::na(86.25, 1, idx),
                ]
            ),
            skill: SucroseSkill::new(idx),
            burst: SimpleBurstDot::new(&[2.0,2.0,2.0, 14.0], Attack {
                kind: AttackType::BurstDot,
                element: &ANEMO_GAUGE1A,
                multiplier: 266.4,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }),
            burst_ea: ElementalAbsorption::new(idx, BurstDot, 79.2, NTimer::new(&[6.0, 14.0])),
        }
    }

    pub fn build(&mut self, builder: &mut FieldAbilityBuilder) -> () {
        builder.na(&mut self.na).skill(&mut self.skill).burst(&mut self.burst).passive(self);
    }
}

impl SpecialAbility for Sucrose {
    fn init(&mut self, timers: &mut ICDTimers) -> () {
        *self.burst_ea.icd() = &mut timers.burst;
    }

    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        let check_idx = event.idx == self.burst.attack.idx;
        let is_swirl = unsafe {
            attack.iter().any(|&a| {
                let atk = & *a;
                atk.idx == data.idx && enemy.trigger_er(&atk.element.aura).is_swirl()
            })
        };
        self.skill_a1.update(time, is_swirl);
        self.skill_a4.update(time, check_idx && (event.kind == Burst || event.kind == PressSkill));
        self.burst_ea.absorb(time, check_idx && event.kind == Burst, enemy);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        match (self.burst.timer.ping, 0 < self.burst.timer.n && self.burst.timer.n <= 3) {
            (true, true) => if let Some(a) = self.burst_ea.attack() {
                atk_queue.push(a);
            },
            _ => (),
        }
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, enemy: &mut Enemy) -> () {
        // TODO inaccurate
        match (self.skill_a1.ping, self.skill_a1.n) {
            (true, 1) => for s in modifiable_state.iter_mut() {
                s.em += 50.0;
            },
            (true, 0) => for s in modifiable_state.iter_mut() {
                s.em -= 50.0;
            },
            _ => (),
        }
        match (self.skill_a4.ping, self.skill_a4.n) {
            (true, 1) => {
                let state = data.state();
                for (i, s) in modifiable_state.iter_mut().enumerate() {
                    if i != data.idx.0 {
                        s.em += state.em * 0.2;
                    }
                }
            },
            (true, 0) => {
                let state = data.state();
                for (i, s) in modifiable_state.iter_mut().enumerate() {
                    if i != data.idx.0 {
                        // TODO inaccurate
                        s.em -= state.em * 0.2;
                    }
                }
            },
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.skill_a1.reset();
        self.skill_a4.reset();
    }
}

pub struct TravelerAnemo {
    na: NaLoop,
    na_last: Attack,
    skill: SkillDamage2Dot,
    // TODO what is multiplier?
    // press_ea: ElementalAbsorption,
    burst: SimpleBurstDot,
    burst_ea: ElementalAbsorption,
}

impl TravelerAnemo {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Traveler (Anemo)").vision(Anemo).weapon(Sword).release_date("2020-09-28").version(1.0)
            .base_hp(10875.0).base_atk(212.0).base_def(683.0)
            .atk(24.0)
            .energy_cost(60.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &Rc<RefCell<ICDTimer>>) -> Self {
        Self {
            na: NaLoop::new(
                // 5 attacks in 2.55 seconds
                &[0.51,0.51,0.51,0.51,0.51],
                vec![
                    Attack::na(87.89, 1, idx),
                    Attack::na(85.85, 1, idx),
                    Attack::na(104.72, 1, idx),
                    Attack::na(115.26, 1, idx),
                    Attack::na(139.91, 1, idx),
                ]
            ),
            na_last: Attack {
                kind: AttackType::Na,
                element: &ANEMO_GAUGE1A,
                multiplier: 60.0,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            },
            skill: SkillDamage2Dot::new(&[0.5,0.5,0.5,0.5,0.5,0.5,0.5, 4.5], Particle::new(Anemo, 4.0), Attack {
                kind: AttackType::PressSkill,
                element: &ANEMO_GAUGE1A,
                multiplier: 345.6,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }, Attack {
                kind: AttackType::SkillDot,
                element: &ANEMO_GAUGE1A,
                multiplier: 30.24,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }),
            burst: SimpleBurstDot::new(&[1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0, 5.0], Attack {
                kind: AttackType::BurstDot,
                element: &ANEMO_GAUGE1A,
                multiplier: 145.44,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }),
            burst_ea: ElementalAbsorption::new(idx, BurstDot, 44.64, NTimer::new(&[10.0, 5.0])),
        }
    }

    pub fn build(&mut self, builder: &mut FieldAbilityBuilder) -> () {
        builder.na(&mut self.na).skill(&mut self.skill).burst(&mut self.burst).passive(self);
    }
}

impl SpecialAbility for TravelerAnemo {
    fn init(&mut self, timers: &mut ICDTimers) -> () {
        self.na_last.icd_timer = &mut timers.na;
        *self.burst_ea.icd() = &mut timers.burst;
    }

    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.burst_ea.absorb(time, event.idx == self.burst.attack.idx && event.kind == Burst, enemy);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        match (self.burst.timer.ping, 0 < self.burst.timer.n && self.burst.timer.n <= 10) {
            (true, true) => if let Some(a) = self.burst_ea.attack() {
                atk_queue.push(a);
            },
            _ => (),
        }
        match (self.na.timer.ping, self.na.timer.n) {
            (true, 5) => atk_queue.push(&self.na_last),
            _ => (),
        }
    }
}

pub struct Jean {
    na: NaLoop,
    skill: SimpleSkill,
    burst: BurstDamage2Dot,
}

impl Jean {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Jean").vision(Anemo).weapon(Sword).release_date("2020-09-28").version(1.0)
            .base_hp(14695.0).base_atk(239.0).base_def(769.0)
            // a4
            .energy_cost(80.0 * 0.8)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &Rc<RefCell<ICDTimer>>) -> Self {
        Self {
            na: NaLoop::new(
                // 5 attacks in 2.55 seconds
                &[0.51,0.51,0.51,0.51,0.51],
                vec![
                    Attack::na(95.54, 1, idx),
                    Attack::na(90.1, 1, idx),
                    Attack::na(119.17, 1, idx),
                    Attack::na(130.22, 1, idx),
                    Attack::na(156.57, 1, idx),
                ]
            ),
            skill: SimpleSkill::new(&[6.0], Particle::new(Anemo, 3.0), Attack {
                kind: AttackType::PressSkill,
                element: &ANEMO_GAUGE2B,
                multiplier: 525.6,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }),
            burst: BurstDamage2Dot::new(&[1.0,1.0,1.0, 17.0], Attack {
                kind: AttackType::Burst,
                element: &ANEMO_GAUGE2B,
                multiplier: 764.64,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }, Attack {
                kind: AttackType::BurstDot,
                element: &ANEMO_GAUGE1A,
                multiplier: 141.12,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }),
        }
    }

    pub fn build(&mut self, builder: &mut FieldAbilityBuilder) -> () {
        builder.na(&mut self.na).skill(&mut self.skill).burst(&mut self.burst).passive(self);
    }
}

impl SpecialAbility for Jean {}

pub struct Venti {
    na: NaLoop,
    skill: SimpleSkill,
    burst: SimpleBurstDot,
    burst_ea: ElementalAbsorption,
}

impl Venti {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Venti").vision(Anemo).weapon(Bow).release_date("2020-09-28").version(1.0)
            .base_hp(10531.0).base_atk(263.0).base_def(669.0)
            .er(32.0)
            // a4
            .energy_cost(60.0 - 15.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &Rc<RefCell<ICDTimer>>) -> Self {
        Self {
            na: NaLoop::new(
                // 6 attacks in 2.85 seconds
                &[0.475,0.475,0.475,0.475,0.475,0.475],
                vec![
                    Attack::na(40.29, 2, idx),
                    Attack::na(87.72, 1, idx),
                    Attack::na(103.53, 1, idx),
                    Attack::na(51.51, 2, idx),
                    Attack::na(100.13, 1, idx),
                    Attack::na(140.25, 1, idx),
                ]
            ),
            skill: SimpleSkill::new(&[6.0], Particle::new(Anemo, 3.0), Attack {
                kind: AttackType::PressSkill,
                element: &ANEMO_GAUGE2B,
                multiplier: 496.8,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }),
            burst: SimpleBurstDot::new(&[1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0, 7.0], Attack {
                kind: AttackType::BurstDot,
                element: &ANEMO_GAUGE1A,
                multiplier: 67.68,
                hits: 1,
                icd_timer: Rc::clone(icd_timer),
                idx,
            }),
            burst_ea: ElementalAbsorption::new(idx, BurstDot, 33.84, NTimer::new(&[8.0, 7.0])),
        }
    }

    pub fn build(&mut self, builder: &mut FieldAbilityBuilder) -> () {
        builder.na(&mut self.na).skill(&mut self.skill).burst(&mut self.burst).passive(self);
    }
}

impl SpecialAbility for Venti {
    fn init(&mut self, timers: &mut ICDTimers) -> () {
        *self.burst_ea.icd() = &mut timers.burst;
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        match (self.burst_ea.timer.ping, self.burst_ea.timer.n) {
            // TODO should be limited to the corresponding vision
            (true, 1) => particles.push_e(15.0),
            _ => (),
        }
        match (self.burst.timer.ping, 0 < self.burst.timer.n && self.burst.timer.n <= 8) {
            (true, true) => if let Some(a) = self.burst_ea.attack() {
                atk_queue.push(a);
            },
            _ => (),
        }
    }
}
