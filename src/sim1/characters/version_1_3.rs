use std::rc::Rc;
use std::cell::RefCell;

use crate::sim1::state::State;
use crate::sim1::types::{AttackType, WeaponType, Vision, FieldEnergy, VecFieldEnergy, Particle, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::sim1::fc::{FieldCharacterIndex, SpecialAbility, SkillAbility, CharacterAbility, NoopAbility, CharacterData, CharacterRecord, Enemy};
use crate::sim1::action::{Attack, AttackEvent, ICDTimer, ElementalAbsorption, NaLoop, SimpleSkill, SimpleSkillDot, SkillDamage2Dot, SimpleBurst, SimpleBurstDot, BurstDamage2Dot, NTimer, DurationTimer, StaminaTimer, ICDTimers};

use AttackType::*;
use WeaponType::*;
use Vision::*;

#[derive(Debug)]
pub struct XiaoSkill {
    pub timer1: NTimer,
    pub timer2: NTimer,
    pub attack1: Attack,
    pub attack2: Attack,
    pub particle: Particle,
}

impl XiaoSkill {
    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            timer1: NTimer::new(&[10.0]),
            timer2: NTimer::new(&[10.0]),
            attack1: Attack {
                kind: AttackType::PressSkill,
                element: &ANEMO_GAUGE2B,
                multiplier: 455.04,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            },
            attack2: Attack {
                kind: AttackType::PressSkill,
                element: &ANEMO_GAUGE2B,
                multiplier: 455.04,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            },
            particle: Particle::new(Anemo, 3.0),
        }
    }
}

impl SkillAbility for XiaoSkill {
    fn accelerate(&mut self, f: fn(&mut NTimer)) -> () {
        f(&mut self.timer1);
        f(&mut self.timer2);
    }
}

impl SpecialAbility for XiaoSkill {
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

// should not register `Builder.na()`
pub struct Xiao {
    na_noop: NoopAbility,
    ca_noop: NoopAbility,
    na: NaLoop,
    plunge: Attack,
    ca_timer: NTimer,
    skill: XiaoSkill,
    burst: SimpleBurst,
}

impl Xiao {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Xiao").vision(Anemo).weapon(Polearm).release_date("2020-12-23").version(1.3)
            .base_hp(12736.0).base_atk(349.0).base_def(799.0)
            .cr(24.2)
            .energy_cost(70.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            na_noop: NoopAbility,
            ca_noop: NoopAbility,
            na: NaLoop::new(
                // 6 attacks in 3.75 seconds
                &[0.625,0.625,0.625,0.625,0.625,0.625],
                vec![
                    Attack::na(49.14, 2, idx, &icd_timer),
                    Attack::na(101.58, 1, idx, &icd_timer),
                    Attack::na(122.3, 1, idx, &icd_timer),
                    Attack::na(67.2, 2, idx, &icd_timer),
                    Attack::na(127.64, 1, idx, &icd_timer),
                    Attack::na(170.97, 1, idx, &icd_timer),
                ]
            ),
            plunge: Attack {
                kind: AttackType::Ca,
                element: &ANEMO_GAUGE1A,
                multiplier: 404.02,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.ca),
                idx,
            },
            ca_timer: NTimer::new(&[1.7]),
            // a1
            skill: XiaoSkill::new(idx, icd_timer),
            burst: SimpleBurst::new(&[3.0,3.0,3.0,3.0,3.0, 3.0], Attack {
                kind: AttackType::Burst,
                element: &PHYSICAL_GAUGE,
                multiplier: 0.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
        }
    }

}

impl CharacterAbility for Xiao {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na_noop }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca_noop }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na_noop }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca_noop }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

impl SpecialAbility for Xiao {
    fn maybe_attack(&self, data: &CharacterData) -> Option<AttackEvent> {
        match (0 < self.burst.timer.n && self.burst.timer.n <= 5, self.ca_timer.n) {
            (true, 0) => Some(AttackEvent {
                kind: self.plunge.kind,
                idx: self.plunge.idx,
            }),
            _ => self.na.maybe_attack(data),
        }
    }

    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        let speedup_time = time * (1.0 + data.state.atk_spd / 100.0);
        self.ca_timer.update(speedup_time, event == &self.plunge);
        self.na.update(speedup_time, event, data, attack, particles, enemy);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        self.na.additional_attack(atk_queue, particles, data);
        match (self.ca_timer.ping, self.ca_timer.n) {
            (true, 1) => atk_queue.push(&self.plunge),
            _ => (),
        }
    }

    // TODO a4 is disabled for now
    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if 1 <= self.burst.timer.n && self.burst.timer.n <= 5 {
            let state = &mut modifiable_data[self.burst.attack.idx.0].state;
            state.infusion = true;
            state.na_dmg += 95.2;
            state.ca_dmg += 95.2;
            match (self.burst.timer.n) {
                1 => state.all_dmg += 5.0,
                2 => state.all_dmg += 10.0,
                3 => state.all_dmg += 15.0,
                4 => state.all_dmg += 20.0,
                5 => state.all_dmg += 25.0,
                _ => (),
            }
        }
    }
}

pub struct HuTao {
    na_noop: NoopAbility,
    ca_noop: NoopAbility,
    na: NaLoop,
    ca: Attack,
    stamina: StaminaTimer,
    skill: SimpleSkill,
    burst: SimpleBurst,
}

impl HuTao {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Hu Tao").vision(Pyro).weapon(Polearm).release_date("2021-01-12").version(1.3)
            .base_hp(15552.0).base_atk(106.0).base_def(876.0)
            .cd(88.4)
            // a4
            .pyro_dmg(33.0)
            .energy_cost(60.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            na_noop: NoopAbility,
            ca_noop: NoopAbility,
            na: NaLoop::new(
                // 6 attacks in 2.925 seconds
                &[0.4875,0.4875,0.4875,0.4875,0.4875,0.4875],
                vec![
                    Attack::na(83.65, 1, idx, &icd_timer),
                    Attack::na(86.09, 1, idx, &icd_timer),
                    Attack::na(108.92, 1, idx, &icd_timer),
                    Attack::na(117.11, 1, idx, &icd_timer),
                    Attack::na((59.36 + 62.8) / 2.0, 2, idx, &icd_timer),
                    Attack::na(153.36, 1, idx, &icd_timer),
                ]
            ),
            ca: Attack {
                kind: AttackType::Ca,
                element: &PYRO_GAUGE1A,
                multiplier: 242.57,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.ca),
                idx,
            },
            stamina: StaminaTimer::new(0.915),
            // a1
            skill: SimpleSkill::new(&[9.0, 7.0], Particle::new(Pyro, 3.0), Attack {
                kind: AttackType::PressSkill,
                element: &PYRO_GAUGE1A,
                multiplier: 115.2,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }),
            burst: SimpleBurst::new(&[15.0], Attack {
                kind: AttackType::Burst,
                element: &PYRO_GAUGE2B,
                multiplier: 617.44,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
        }
    }

}

impl CharacterAbility for HuTao {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na_noop }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca_noop }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na_noop }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca_noop }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

impl SpecialAbility for HuTao {
    fn maybe_attack(&self, data: &CharacterData) -> Option<AttackEvent> {
        match (self.skill.timer.n, self.stamina.n) {
            (1, 0) => Some(AttackEvent {
                kind: self.ca.kind,
                idx: self.ca.idx,
            }),
            _ => self.na.maybe_attack(data),
        }
    }

    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        let speedup_time = time * (1.0 + data.state.atk_spd / 100.0);
        self.stamina.update(speedup_time, 25.0 + 18.0, event == &self.ca);
        self.na.update(speedup_time, event, data, attack, particles, enemy);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        self.na.additional_attack(atk_queue, particles, data);
        if self.stamina.ping && self.stamina.n == 1 {
            atk_queue.push(&self.na.attack[0]);
            atk_queue.push(&self.ca);
        }
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        match self.skill.timer.n {
            1 => {
                let state = &mut modifiable_data[self.burst.attack.idx.0].state;
                state.infusion = true;
                state.flat_atk += state.HP() * 0.0626;
            },
            2 => {
                // a1
                for (i, data) in modifiable_data.iter_mut().enumerate() {
                    if i != self.burst.attack.idx.0 {
                        data.state.cr += 12.0;
                    }
                }
            },
            _ => (),
        }
    }
}
