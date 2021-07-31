use std::ptr;

use crate::state::State;
use crate::types::{AttackType, WeaponType, Vision, FieldEnergy, VecFieldEnergy, Particle, GAUGE1A, GAUGE2B};
use crate::fc::{FieldCharacterIndex, SpecialAbility, CharacterAbility, CharacterData, CharacterRecord, Enemy};
use crate::action::{Attack, ElementalAttack, ElementalAttackVector, ElementalAbsorption, FullCharacterTimers, CharacterTimersBuilder, TimerGuard, EffectTimer, DurationTimer, HitsTimer, DotTimer, LoopTimer, StaminaTimer};
use crate::testutil;

use AttackType::*;
use WeaponType::*;
use Vision::*;

pub struct Kazuha {
    skill_aa: HitsTimer,
    burst_aa: DotTimer,
    swirl_a4: DurationTimer,
    na_1: Attack,
    na_2: Attack,
    na_3: Attack,
    na_4: Attack,
    na_5: Attack,
    midare_ranzan: Attack,
    soumon_swordsmanship: ElementalAbsorption,
    press: Attack, // hold
    burst: Attack,
    burst_dot: Attack,
    burst_ea: ElementalAbsorption,
}

impl Kazuha {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            skill_aa: HitsTimer::new(9.0, 1),
            burst_aa: DotTimer::new(20.0, 2.0, 4),
            swirl_a4: DurationTimer::new(0.0, 8.0),
            na_1: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 88.91,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_2: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 89.42,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_3: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: (51.0 + 61.2) / 2.0,
                hits: 2,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_4: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 120.02,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            na_5: Attack {
                kind: AttackType::Na,
                gauge: &GAUGE1A,
                multiplier: 50.15,
                hits: 3,
                icd_timer: ptr::null_mut(),
                idx,
            },
            midare_ranzan: Attack {
                kind: AttackType::Ca,
                gauge: &GAUGE1A,
                multiplier: 404.02,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            soumon_swordsmanship: ElementalAbsorption::new(idx, Ca, 200.0, DurationTimer::new(9.0, 1.0)),
            press: Attack {
                kind: AttackType::PressSkill,
                gauge: &GAUGE2B,
                multiplier: 469.44,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE2B,
                multiplier: 419.04,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_dot: Attack {
                kind: AttackType::Burst,
                gauge: &GAUGE1A,
                multiplier: 216.0,
                hits: 1,
                icd_timer: ptr::null_mut(),
                idx,
            },
            burst_ea: ElementalAbsorption::new(idx, BurstDot, 64.8, DurationTimer::new(15.0, 8.0)),
        }
    }
}

impl CharacterAbility for Kazuha {
    fn record(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Kazuha").vision(Anemo).weapon(Sword).release_date("2021-06-29").version(1.6)
            .base_hp(13348.0).base_atk(297.0).base_def(807.0)
            .em(115.2)
            .energy_cost(60.0)
    }

    fn timers(&self) -> FullCharacterTimers {
        CharacterTimersBuilder::new()
            .na(LoopTimer::new(2.166, 5))
            .press(DotTimer::single_hit(9.0))
            .burst(DotTimer::new(20.0, 2.0, 4))
            .build()
    }

    fn init_attack(&mut self, timers: &mut FullCharacterTimers) -> () {
        self.na_1.icd_timer = &mut timers.na_icd;
        self.na_2.icd_timer = &mut timers.na_icd;
        self.na_3.icd_timer = &mut timers.na_icd;
        self.na_4.icd_timer = &mut timers.na_icd;
        self.na_5.icd_timer = &mut timers.na_icd;
        self.midare_ranzan.icd_timer = &mut timers.ca_icd;
        *(self.soumon_swordsmanship.icd()) = &mut timers.ca_icd;
        self.press.icd_timer = &mut timers.skill_icd;
        self.burst.icd_timer = &mut timers.burst_icd;
        self.burst_dot.icd_timer = &mut timers.burst_icd;
        *(self.burst_ea.icd()) = &mut timers.burst_icd;
    }
}

impl SpecialAbility for Kazuha {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[FieldEnergy], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.soumon_swordsmanship.absorb(guard.check_second(PressSkill), enemy, time);
        self.burst_ea.absorb(guard.check_second(Burst), enemy, time);
        // TODO remember the specific element on the enemy
        self.swirl_a4.update(guard.second(attack.iter().any(|a| enemy.trigger_er(&a.element).is_swirl())), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<FieldEnergy>, timers: &FullCharacterTimers, data: &CharacterData, _enemy: &Enemy) -> () {
        let burst = timers.burst_timer();
        if burst.is_active() {
            if burst.n() == 1 {
                atk_queue.push(ElementalAttack::anemo(&self.burst));
                atk_queue.push(ElementalAttack::anemo(&self.burst_dot));
            } else {
                atk_queue.push(ElementalAttack::anemo(&self.burst_dot));
            }
            if let Some(a) = self.burst_ea.attack() {
                atk_queue.push(a);
            }
        }
        if timers.press_timer().is_active() {
            particles.push_p(Particle::new(Anemo, 4.0));
            atk_queue.push(ElementalAttack::anemo(&self.press));
            atk_queue.push(ElementalAttack::anemo(&self.midare_ranzan));
            if let Some(a) = self.soumon_swordsmanship.attack() {
                atk_queue.push(a);
            }
        }
        let na = timers.na_timer();
        if na.is_active() {
            match na.n() {
                1 => atk_queue.push_anemo(data, &self.na_1),
                2 => atk_queue.push_anemo(data, &self.na_2),
                3 => atk_queue.push_anemo(data, &self.na_3),
                4 => atk_queue.push_anemo(data, &self.na_4),
                5 => atk_queue.push_anemo(data, &self.na_5),
                _ => (),
            };
        }
    }

    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        if self.swirl_a4.is_active() {
            for s in modifiable_state.iter_mut() {
                s.elemental_dmg += data.state.em * 0.04;
            }
        }
    }

    fn reset(&mut self) -> () {
        self.swirl_a4.reset();
    }
}
