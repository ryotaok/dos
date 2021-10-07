use std::rc::Rc;
use std::cell::RefCell;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Vision, FieldEnergy, VecFieldEnergy, Particle, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, ELECTRO_GAUGE4C, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::fc::{FieldCharacterIndex, SpecialAbility, SkillAbility, CharacterAbility, NoopAbility, CharacterData, CharacterRecord, Enemy};
use crate::action::{Attack, AttackEvent, ICDTimer, ElementalAbsorption, NaLoop, SimpleSkill, SimpleSkillDot, SkillDamage2Dot, SkillDamage2DotParticle, SimpleBurst, SimpleBurstDot, BurstDamage2Dot, NTimer, DurationTimer, ICDTimers};
// StaminaTimer

use AttackType::*;
use WeaponType::*;
use Vision::*;

// version 1.0

pub struct Beidou {
    na: NaLoop,
    ca: NoopAbility,
    skill: SimpleSkill,
    burst: BurstDamage2Dot,
}

impl Beidou {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Beidou").vision(Electro).weapon(Claymore).release_date("2020-09-28").version(1.0)
            .base_hp(13050.0).base_atk(225.0).base_def(648.0)
            .electro_dmg(24.0)
            .energy_cost(80.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            na: NaLoop::new(
                // 5 attacks in 3.75 seconds
                &[0.75,0.75,0.75,0.75,0.75],
                vec![
                    Attack::na(140.59, 1, idx, &icd_timer),
                    Attack::na(140.08, 1, idx, &icd_timer),
                    Attack::na(174.59, 1, idx, &icd_timer),
                    Attack::na(171.02, 1, idx, &icd_timer),
                    Attack::na(221.68, 1, idx, &icd_timer),
                ]
            ),
            ca: NoopAbility,
            skill: SimpleSkill::new(&[7.5], Particle::new(Electro, 2.5), Attack {
                kind: AttackType::PressSkill,
                element: &ELECTRO_GAUGE2B,
                multiplier: (218.88 + 288.0) / 2.0,
                hits: 2,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }),
            burst: BurstDamage2Dot::new(&[1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0, 10.0], Attack {
                kind: AttackType::Burst,
                element: &ELECTRO_GAUGE4C,
                multiplier: 218.88,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }, Attack {
                kind: AttackType::BurstDot,
                element: &ELECTRO_GAUGE1A,
                multiplier: 172.8,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
        }
    }
}

impl CharacterAbility for Beidou {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

impl SpecialAbility for Beidou {
    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        // a4
        if self.skill.timer.n == 1 {
            let state = &mut modifiable_data[self.skill.attack.idx.0].state;
            state.na_dmg += 15.0;
            state.ca_dmg += 15.0;
            state.atk_spd += 15.0;
        }
    }

    // // TODO inaccurate
    // fn intensify(&self, attack: &Attack) -> Option<State> {
    //     if attack.kind == PressSkill {
    //         Some(State::new().skill_dmg(20.0))
    //     } else {
    //         None
    //     }
    // }
}

pub struct Fischl {
    electro_er: bool,
    na: NaLoop,
    ca: NoopAbility,
    skill: SkillDamage2DotParticle,
    burst: SimpleBurst,
    aa_a4: Attack,
}

impl Fischl {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Fischl").vision(Electro).weapon(Bow).release_date("2020-09-28").version(1.0)
            .base_hp(9189.0).base_atk(244.0).base_def(594.0)
            .atk(24.0)
            .energy_cost(60.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            electro_er: false,
            na: NaLoop::new(
                // 5 attacks in 2.1 seconds
                &[0.42,0.42,0.42,0.42,0.42],
                vec![
                    Attack::na(87.21, 1, idx, &icd_timer),
                    Attack::na(92.48, 1, idx, &icd_timer),
                    Attack::na(114.92, 1, idx, &icd_timer),
                    Attack::na(114.07, 1, idx, &icd_timer),
                    Attack::na(142.46, 1, idx, &icd_timer),
                ]
            ),
            ca: NoopAbility,
            skill: SkillDamage2DotParticle::new(&[1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0, 13.0], Particle::new(Electro, 1.0), Attack {
                kind: AttackType::PressSkill,
                element: &ELECTRO_GAUGE1A,
                multiplier: 207.79,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }, Attack {
                kind: AttackType::SkillDot,
                element: &ELECTRO_GAUGE1A,
                multiplier: 159.84,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }),
            burst: SimpleBurst::new(&[15.0], Attack {
                kind: AttackType::Burst,
                element: &ELECTRO_GAUGE1A,
                multiplier: 374.4,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
            aa_a4: Attack {
                kind: AttackType::SkillDot, // TODO inaccurate
                element: &ELECTRO_GAUGE1A,
                multiplier: 80.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }
        }
    }
}

impl CharacterAbility for Fischl {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

impl SpecialAbility for Fischl {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.electro_er = unsafe {
            attack.iter().any(|&a| enemy.trigger_er(&(*a).element.aura).is_electro())
        };
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        if self.electro_er && 0 < self.skill.timer.n && self.skill.timer.n <= 12 {
            atk_queue.push(&self.aa_a4);
        }
    }
}

#[derive(Debug)]
pub struct LisaSkill {
    conductive_status: usize,
    press_timer: NTimer,
    hold_timer: NTimer,
    press: Attack,
    hold_3: Attack,
}

impl LisaSkill {
    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            conductive_status: 0,
            press_timer: NTimer::new(&[1.0]),
            hold_timer: NTimer::new(&[16.0]),
            press: Attack {
                kind: AttackType::PressSkill,
                element: &ELECTRO_GAUGE2B,
                multiplier: 144.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            },
            hold_3: Attack {
                kind: AttackType::HoldSkill,
                element: &ELECTRO_GAUGE2B,
                multiplier: 876.96,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            },
        }
    }
}

impl SkillAbility for LisaSkill {
    fn accelerate(&mut self, f: fn(&mut NTimer)) -> () {
        f(&mut self.press_timer);
        f(&mut self.hold_timer);
    }
}

impl SpecialAbility for LisaSkill {
    fn maybe_attack(&self, _data: &CharacterData) -> Option<AttackEvent> {
        if self.conductive_status == 3 {
            self.hold_3.to_event(&self.hold_timer)
        } else if self.hold_timer.n == 0 {
            self.press.to_event(&self.press_timer)
        } else {
            None
        }
    }

    fn update(&mut self, time: f32, event: &AttackEvent, _data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.hold_timer.update(time, event == &self.hold_3);
        self.press_timer.update(time, event == &self.press);
        if self.conductive_status == 3 {
            self.conductive_status = 0;
        }
        if self.press_timer.ping && self.press_timer.n == 1 {
            self.conductive_status += 1;
        }
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, _data: &CharacterData) -> () {
        match (self.hold_timer.ping, self.hold_timer.n, self.press_timer.ping, self.press_timer.n) {
            (true, 1, _, _) => {
                atk_queue.push(&self.hold_3);
                particles.push_p(Particle::new(Electro, 5.0));
            },
            (_, _, true, 1) => {
                atk_queue.push(&self.press);
                particles.push_p(Particle::new(Electro, 0.3));
            },
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.conductive_status = 0;
        self.press_timer.reset();
        self.hold_timer.reset();
    }
}

pub struct Lisa {
    a4_timer: DurationTimer,
    na: NaLoop,
    ca: NoopAbility,
    skill: LisaSkill,
    burst: SimpleBurstDot,
}

impl Lisa {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Lisa").vision(Electro).weapon(Catalyst).release_date("2020-09-28").version(1.0)
            .infusion(true)
            .base_hp(9570.0).base_atk(232.0).base_def(573.0)
            .em(96.0)
            .energy_cost(80.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            a4_timer: DurationTimer::new(10.0, &[0.0]),
            na: NaLoop::new(
                // 4 attacks in 1.5 seconds
                &[0.375,0.375,0.375,0.375],
                vec![
                    Attack::na(71.28, 1, idx, &icd_timer),
                    Attack::na(64.66, 1, idx, &icd_timer),
                    Attack::na(77.04, 1, idx, &icd_timer),
                    Attack::na(98.93, 1, idx, &icd_timer),
                ]
            ),
            ca: NoopAbility,
            skill: LisaSkill::new(idx, icd_timer),
            burst: SimpleBurstDot::new(&[0.5555,0.5555,0.5555,0.5555,0.5555,0.5555,0.5555,0.5555,0.5555,0.5555,0.5555,0.5555,0.5555,0.5555,0.5555,0.5555,0.5555,0.5555,0.5555,0.5555,0.5555,0.5555,0.5555,0.5555,0.5555,0.5555,0.5555,0.5555, 4.446], Attack {
                kind: AttackType::BurstDot,
                element: &ELECTRO_GAUGE1A,
                multiplier: 65.81,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
        }
    }
}

impl CharacterAbility for Lisa {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

impl SpecialAbility for Lisa {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.a4_timer.update(time, self.burst.timer.ping && 0 < self.burst.timer.n && self.burst.timer.n <= 28);
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if self.a4_timer.ping {
            match self.a4_timer.n {
                1 => enemy.def_down += 15.0,
                0 => enemy.def_down -= 15.0,
                _ => (),
            }
        }
    }

    fn reset(&mut self) -> () {
        self.skill.reset();
    }
}

#[derive(Debug)]
pub struct RazorSkill {
    electro_sigil: usize,
    press_timer: NTimer,
    hold_timer: NTimer,
    press: Attack,
    hold: Attack,
}

impl RazorSkill {
    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            electro_sigil: 0,
            // a1
            press_timer: NTimer::new(&[6.0 * 0.82]),
            hold_timer: NTimer::new(&[10.0 * 0.82]),
            press: Attack {
                kind: AttackType::PressSkill,
                element: &ELECTRO_GAUGE2B,
                multiplier: 358.56,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            },
            hold: Attack {
                kind: AttackType::HoldSkill,
                element: &ELECTRO_GAUGE2B,
                multiplier: 531.36,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            },
        }
    }
}

impl SkillAbility for RazorSkill {
    fn accelerate(&mut self, f: fn(&mut NTimer)) -> () {
        f(&mut self.press_timer);
        f(&mut self.hold_timer);
    }
}

impl SpecialAbility for RazorSkill {
    fn maybe_attack(&self, _data: &CharacterData) -> Option<AttackEvent> {
        if self.electro_sigil == 3 {
            self.hold.to_event(&self.hold_timer)
        } else if self.hold_timer.n == 0 {
            self.press.to_event(&self.press_timer)
        } else {
            None
        }
    }

    fn update(&mut self, time: f32, event: &AttackEvent, _data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.hold_timer.update(time, event == &self.hold);
        self.press_timer.update(time, event == &self.press);
        if self.press_timer.ping && self.press_timer.n == 1 {
            self.electro_sigil += 1;
        } else if self.electro_sigil == 3 {
            self.electro_sigil = 0;
        }
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, _data: &CharacterData) -> () {
        match (self.hold_timer.ping, self.hold_timer.n, self.press_timer.ping, self.press_timer.n) {
            (true, 1, _, _) => {
                atk_queue.push(&self.hold);
                particles.push_p(Particle::new(Electro, 5.0));
            },
            (_, _, true, 1) => {
                atk_queue.push(&self.press);
                particles.push_p(Particle::new(Electro, 4.0));
            },
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.press_timer.reset();
        self.hold_timer.reset();
    }
}

pub struct Razor {
    na: NaLoop,
    ca: NoopAbility,
    skill: RazorSkill,
    burst: BurstDamage2Dot,
}

impl Razor {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Razor").vision(Electro).weapon(Claymore).release_date("2020-09-28").version(1.0)
            .base_hp(11962.0).base_atk(234.0).base_def(751.0)
            .physical_dmg(30.0)
            .energy_cost(80.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            na: NaLoop::new(
                // 4 attacks in 2.734 seconds
                &[0.6835,0.6835,0.6835,0.6835],
                vec![
                    Attack::na(171.13, 1, idx, &icd_timer),
                    Attack::na(147.42, 1, idx, &icd_timer),
                    Attack::na(184.32, 1, idx, &icd_timer),
                    Attack::na(242.72, 1, idx, &icd_timer),
                ]
            ),
            ca: NoopAbility,
            skill: RazorSkill::new(idx, icd_timer),
            burst: BurstDamage2Dot::new(&[0.75,0.75,0.75,0.75,0.75,0.75,0.75,0.75,0.75,0.75,0.75,0.75,0.75,0.75,0.75,0.75,0.75,0.75,0.75,0.75, 5.0,], Attack {
                kind: AttackType::Burst,
                element: &ELECTRO_GAUGE2B,
                multiplier: 288.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }, Attack {
                kind: AttackType::BurstDot,
                element: &ELECTRO_GAUGE1A,
                multiplier: 43.2,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
        }
    }
}

impl CharacterAbility for Razor {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

impl SpecialAbility for Razor {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        // a1
        if event == &self.burst.attack {
            self.skill.reset();
        }
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        let energy_cost = modifiable_data[self.burst.attack.idx.0].character.energy_cost;
        let mut state = &mut modifiable_data[self.burst.attack.idx.0].state;
        // a4
        if state.energy / energy_cost <= 0.5 {
            state.er += 30.0;
        }
        if 0 < self.skill.electro_sigil {
            state.er += 20.0 * self.skill.electro_sigil as f32;
        }
        if self.skill.hold_timer.ping && self.skill.hold_timer.n == 1 {
            state.energy += 5.0 * self.skill.electro_sigil as f32;
        }
        if 1 <= self.burst.timer.n && self.burst.timer.n < 21 {
            state.atk_spd += 40.0;
        }
    }
}

#[derive(Debug)]
pub struct KeqingBurst {
    timer: NTimer,
    attack_1: Attack,
    attack_2: Attack,
    attack_3: Attack,
}

impl KeqingBurst {
    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            timer: NTimer::new(&[0.5,0.5,0.5, 6.5, 4.0]),
            attack_1: Attack {
                kind: AttackType::Burst,
                element: &ELECTRO_GAUGE1A,
                multiplier: 158.4,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            },
            attack_2: Attack {
                kind: AttackType::BurstDot,
                element: &ELECTRO_GAUGE1A,
                multiplier: 43.2,
                hits: 8,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            },
            attack_3: Attack {
                kind: AttackType::BurstDot,
                element: &ELECTRO_GAUGE1A,
                multiplier: 339.84,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            },
        }
    }
}

impl SpecialAbility for KeqingBurst {
    fn maybe_attack(&self, data: &CharacterData) -> Option<AttackEvent> {
        if data.can_burst() {
            self.attack_1.to_event(&self.timer)
        } else {
            None
        }
    }

    fn update(&mut self, time: f32, event: &AttackEvent, _data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update(time, event == &self.attack_1);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, _particles: &mut Vec<FieldEnergy>, _data: &CharacterData) -> () {
        match (self.timer.ping, self.timer.n) {
            (true, 1) => atk_queue.push(&self.attack_1),
            (true, 2) => atk_queue.push(&self.attack_2),
            (true, 3) => atk_queue.push(&self.attack_3),
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct Keqing {
    na: NaLoop,
    ca: NoopAbility,
    skill: SimpleSkill,
    burst: KeqingBurst,
}

impl Keqing {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Keqing").vision(Electro).weapon(Sword).release_date("2020-09-28").version(1.0)
            .base_hp(13103.0).base_atk(323.0).base_def(799.0)
            .cd(88.4)
            .energy_cost(40.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            na: NaLoop::new(
                // 5 attacks in 2.017 seconds
                &[0.4034,0.4034,0.4034,0.4034,0.4034],
                vec![
                    Attack::na(81.09, 1, idx, &icd_timer),
                    Attack::na(81.09, 1, idx, &icd_timer),
                    Attack::na(107.61, 1, idx, &icd_timer),
                    Attack::na((62.22+68.0) / 2.0, 2, idx, &icd_timer),
                    Attack::na(132.43, 1, idx, &icd_timer),
                ]
            ),
            ca: NoopAbility,
            skill: SimpleSkill::new(&[5.0, 2.5], Particle::new(Electro, 2.5), Attack {
                kind: AttackType::PressSkill,
                element: &ELECTRO_GAUGE1A,
                multiplier: (90.72 + 302.4) / 2.0,
                hits: 2,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }),
            burst: KeqingBurst::new(idx, icd_timer),
        }
    }
}

impl CharacterAbility for Keqing {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

impl SpecialAbility for Keqing {
    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        let state = &mut modifiable_data[self.skill.attack.idx.0].state;
        if self.skill.timer.n == 1 {
            state.infusion = true;
        }
        if 1 <= self.burst.timer.n && self.burst.timer.n < 5 {
            state.cr += 15.0;
            state.er += 15.0;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use crate::types::Vision;
    use crate::simulate::simulate;
    use crate::testutil::{TestEnvironment2, TestCharacter};

    #[test]
    fn lisa() {
        let mut timers = ICDTimers::new();
        let cr = TestCharacter::record(Vision::Electro);
        let mut character = Lisa::new(FieldCharacterIndex(0), &timers);
        character.na = NaLoop::noop(FieldCharacterIndex(0));

        let mut env = TestEnvironment2::character(0, &mut timers, State::new(), &cr, &mut character);

        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(0.5, &mut env.data, &mut env.ability, &mut env.atk_queue, &mut env.field_energy, &mut env.enemy);
        }

        let expect = 3.0 * 144.0 + 876.96;
        assert_eq!(total_dmg, expect);
    }
}
