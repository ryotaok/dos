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

pub struct Kazuha {
    swirl_a4: DurationTimer,
    na: NaLoop,
    ca: NoopAbility,
    skill: SimpleSkill, // hold
    burst: BurstDamage2Dot,
    midare_ranzan: Attack,
    soumon_swordsmanship: ElementalAbsorption,
    burst_ea: ElementalAbsorption,
}

impl Kazuha {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Kazuha").vision(Anemo).weapon(Sword).release_date("2021-06-29").version(1.6)
            .base_hp(13348.0).base_atk(297.0).base_def(807.0)
            .em(115.2)
            .energy_cost(60.0)
    }

    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            swirl_a4: DurationTimer::new(8.0, &[0.0]),
            na: NaLoop::new(
                // 5 attacks in 2.166 seconds
                &[0.4332,0.4332,0.4332,0.4332,0.4332],
                vec![
                    Attack::na(88.91, 1, idx, &icd_timer),
                    Attack::na(89.42, 1, idx, &icd_timer),
                    Attack::na((51.0 + 61.2) / 2.0, 2, idx, &icd_timer),
                    Attack::na(120.02, 1, idx, &icd_timer),
                    Attack::na(50.15, 3, idx, &icd_timer),
                ]
            ),
            midare_ranzan: Attack {
                kind: AttackType::Ca,
                element: &ANEMO_GAUGE1A,
                multiplier: 404.02,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.ca),
                idx,
            },
            soumon_swordsmanship: ElementalAbsorption::new(idx, Ca, 200.0, NTimer::new(&[9.0]), icd_timer),
            ca: NoopAbility,
            skill: SimpleSkill::new(&[9.0], Particle::new(Anemo, 4.0), Attack {
                kind: AttackType::PressSkill,
                element: &ANEMO_GAUGE2B,
                multiplier: 469.44,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.skill),
                idx,
            }),
            burst: BurstDamage2Dot::new(&[2.0,2.0,2.0,2.0, 7.0], Attack {
                kind: AttackType::Burst,
                element: &ANEMO_GAUGE2B,
                multiplier: 419.04,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }, Attack {
                kind: AttackType::BurstDot,
                element: &ANEMO_GAUGE1A,
                multiplier: 216.0,
                hits: 1,
                icd_timer: Rc::clone(&icd_timer.burst),
                idx,
            }),
            burst_ea: ElementalAbsorption::new(idx, BurstDot, 64.8, NTimer::new(&[8.0, 7.0]), icd_timer),
        }
    }
}

impl CharacterAbility for Kazuha {
    fn na_ref(&self) -> &dyn SpecialAbility { &self.na }
    fn ca_ref(&self) -> &dyn SpecialAbility { &self.ca }
    fn skill_ref(&self) -> &dyn SkillAbility { &self.skill }
    fn burst_ref(&self) -> &dyn SpecialAbility { &self.burst }
    fn na_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.na }
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.ca }
    fn skill_mut(&mut self) -> &mut dyn SkillAbility { &mut self.skill }
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility { &mut self.burst }
}

impl SpecialAbility for Kazuha {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.soumon_swordsmanship.absorb(time, event == &self.skill.attack, enemy);
        self.burst_ea.absorb(time, event == &self.burst.attack, enemy);
        // TODO remember the specific element on the enemy
        let is_swirl = unsafe {
            attack.iter().any(|&a| {
                let atk = & *a;
                atk.idx == data.idx && enemy.trigger_er(&atk.element.aura).is_swirl()
            })
        };
        self.swirl_a4.update(time, is_swirl);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        if self.burst.timer.ping && 0 < self.burst.timer.n && self.burst.timer.n <= 4 {
            if let Some(a) = self.burst_ea.attack() {
                atk_queue.push(a);
            }
        }
        if self.skill.timer.ping && self.skill.timer.n == 1 {
            atk_queue.push(&self.midare_ranzan);
            if let Some(a) = self.soumon_swordsmanship.attack() {
                atk_queue.push(a);
            }
        }
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if self.swirl_a4.ping {
            let em = modifiable_data[self.burst.attack.idx.0].state.em;
            match self.swirl_a4.n {
                1 => for data in modifiable_data.iter_mut() {
                    let bonus = em * 0.04;
                    data.state.pyro_dmg += bonus;
                    data.state.hydro_dmg += bonus;
                    data.state.electro_dmg += bonus;
                    data.state.cryo_dmg += bonus;
                },
                0 => for data in modifiable_data.iter_mut() {
                    let bonus = em * 0.04;
                    data.state.pyro_dmg -= bonus;
                    data.state.hydro_dmg -= bonus;
                    data.state.electro_dmg -= bonus;
                    data.state.cryo_dmg -= bonus;
                },
                _ => (),
            }
        }
    }

    fn reset(&mut self) -> () {
        self.swirl_a4.reset();
    }
}
