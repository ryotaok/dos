use crate::state::State;
use crate::types::{AttackType, Vision, ElementalGaugeDecay};
use crate::fc::{SpecialAbility, FieldCharacter, FieldAction, CharacterRecord, Enemy};
use crate::action::{Attack, TimerGuard, EffectTimer, DurationTimer, HitsTimer, DotTimer};

use AttackType::*;
use Vision::*;
use ElementalGaugeDecay::*;

// version 1.0

pub struct Sucrose {
    burst_aa: DotTimer,
    skill_a1: bool,
    skill_a4: DurationTimer,
}

impl Sucrose {
    pub fn new() -> Self {
        Self {
            burst_aa: DotTimer::new(20.0, 2.0, 3),
            skill_a1: false,
            skill_a4: DurationTimer::new(15.0, 8.0),
        }
    }
}

impl SpecialAbility for Sucrose {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Sucrose").vision(Anemo).weapon(Catalyst).release_date("2020-09-28").version(1.0)
            .base_hp(9244.0).base_atk(170.0).base_def(703.0)
            .dmg_anemo(24.0)
            .na_1(60.24).na_2(55.11).na_3(69.21).na_4(86.25).na_time(1.5)
            // na_0(0.0).ca_1(0.0).ca_2(0.0).ca_time(1.9)
            .press_cd(15.0).press_particle(4.0).press_dmg(380.16)
            .burst_cd(20.0).energy_cost(80.0).burst_dmg(0.0)
    }

    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.burst_aa.update(guard.second(attack.iter().any(|a| a.kind == Burst)), time);
        self.skill_a1 = attack.iter().any(|a| enemy.trigger_er(&a.element).is_swirl());
        self.skill_a4.update(guard.second(attack.iter().any(|a| a.kind == Skill || a.kind == Burst || a.kind == BurstDot)), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        if self.burst_aa.is_active() {
            atk_queue.push(Attack {
                kind: BurstDot,
                element: Anemo,
                multiplier: 266.4 + 79.2,
                particle: None,
                state: None,
                icd_cleared: fa.burst.icd.clear(),
                on_field_character_index: data.idx.0,
                fc_ptr: data,
            })
        }
    }

    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        // TODO inaccurate
        if self.skill_a1 {
            for s in modifiable_state.iter_mut() {
                s.em += 50.0;
            }
        }
        if self.skill_a4.is_active() {
            for (i, s) in modifiable_state.iter_mut().enumerate() {
                if i != data.idx.0 {
                    s.em += data.state.em * 0.2;
                }
            }
        }
    }

    fn reset(&mut self) -> () {
        self.burst_aa.reset();
        self.skill_a1 = false;
        self.skill_a4.reset();
    }
}

pub struct TravelerAnemo {
    na_1: HitsTimer,
    burst_aa: DotTimer,
}

impl TravelerAnemo {
    pub fn new() -> Self {
        Self {
            na_1: HitsTimer::new(2.55, 1),
            burst_aa: DotTimer::new(15.0, 1.0, 10),
        }
    }
}

impl SpecialAbility for TravelerAnemo {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Traveler (Anemo)").vision(Anemo).weapon(Sword).release_date("2020-09-28").version(1.0)
            .base_hp(10875.0).base_atk(212.0).base_def(683.0)
            .atk(24.0)
            .na_1(87.89).na_2(85.85).na_3(104.72).na_4(115.26).na_5(139.91).na_time(2.55)
            // .na_0(0.0).ca_1(0.0).ca_2(0.0).ca_time(0.0)
            .press_cd(8.0).press_particle(3.5).press_dmg(345.6 + 30.24 * 7.0)
            .burst_cd(15.0).energy_cost(60.0).burst_dmg(0.0)
    }

    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.burst_aa.update(guard.second(attack.iter().any(|a| a.kind == Burst)), time);
        self.na_1.update(guard.second(attack.iter().any(|a| a.kind == Na)), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        if self.burst_aa.is_active() {
            atk_queue.push(Attack {
                kind: BurstDot,
                element: Anemo,
                multiplier: 145.44 + 44.64,
                particle: None,
                state: None,
                icd_cleared: fa.burst.icd.clear(),
                on_field_character_index: data.idx.0,
                fc_ptr: data,
            })
        }
        if self.na_1.is_active() {
            atk_queue.push(Attack {
                kind: Na,
                element: Anemo,
                multiplier: 60.0,
                particle: None,
                state: None,
                icd_cleared: fa.na.icd.clear(),
                on_field_character_index: data.idx.0,
                fc_ptr: data,
            })
        }
    }

    fn reset(&mut self) -> () {
        self.burst_aa.reset();
        self.na_1.reset();
    }
}

pub struct Jean {
    burst_aa: DotTimer,
}

impl Jean {
    pub fn new() -> Self {
        Self {
            burst_aa: DotTimer::new(20.0, 1.0, 3),
        }
    }
}

impl SpecialAbility for Jean {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Jean").vision(Anemo).weapon(Sword).release_date("2020-09-28").version(1.0)
            .base_hp(14695.0).base_atk(239.0).base_def(769.0)
            .na_1(95.54).na_2(90.1).na_3(119.17).na_4(130.22).na_5(156.57).na_time(1.67)
            // .na_0(0.0).ca_1(0.0).ca_2(0.0).ca_time(0.0)
            .press_cd(6.0).press_particle(3.0).press_dmg(525.6)
            // a4
            .burst_cd(20.0).energy_cost(80.0 * 0.8).burst_dmg(764.64)
            .skill_unit(2.0).skill_decay(B).burst_unit(2.0).burst_decay(B)
    }

    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.burst_aa.update(guard.second(attack.iter().any(|a| a.kind == Burst)), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        if self.burst_aa.is_active() {
            atk_queue.push(Attack {
                kind: BurstDot,
                element: Anemo,
                multiplier: 141.12,
                particle: None,
                state: None,
                icd_cleared: fa.burst.icd.clear(),
                on_field_character_index: data.idx.0,
                fc_ptr: data
            })
        }
    }

    fn reset(&mut self) -> () {
        self.burst_aa.reset();
    }
}

pub struct Venti {
    burst_aa: DotTimer,
    burst_a4: HitsTimer,
}

impl Venti {
    pub fn new() -> Self {
        Self {
            burst_aa: DotTimer::new(15.0, 1.0, 8),
            burst_a4: HitsTimer::new(15.0, 1),
        }
    }
}

impl SpecialAbility for Venti {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Venti").vision(Anemo).weapon(Bow).release_date("2020-09-28").version(1.0)
            .base_hp(10531.0).base_atk(263.0).base_def(669.0)
            .er(32.0)
            .na_1(40.29*2.0).na_2(87.72).na_3(103.53).na_4(51.51*2.0).na_5(100.13).na_6(140.25).na_time(2.4)
            // .na_0(0.0).ca_1(0.0).ca_2(0.0).ca_time(0.0)
            .press_cd(6.0).press_particle(3.0).press_dmg(496.8)
            // a4
            .burst_cd(15.0).energy_cost(60.0 - 15.0).burst_dmg(0.0)
            .skill_unit(2.0).skill_decay(B)
    }

    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.burst_aa.update(guard.second(attack.iter().any(|a| a.kind == Burst)), time);
        self.burst_a4.update(guard.second(attack.iter().any(|a| a.kind == BurstDot && enemy.trigger_er(&a.element).is_swirl())), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        if self.burst_aa.is_active() {
            atk_queue.push(Attack {
                kind: BurstDot,
                element: Anemo,
                multiplier: 67.68 + 33.84,
                particle: None,
                state: None,
                icd_cleared: fa.burst.icd.clear(),
                on_field_character_index: data.idx.0,
                fc_ptr: data,
            })
        }
        if self.burst_a4.is_active() {
            atk_queue.push(Attack {
                kind: StandStill,
                element: enemy.aura.aura,
                multiplier: 0.0,
                particle: Some(5.0),
                state: None,
                icd_cleared: false,
                on_field_character_index: data.idx.0,
                fc_ptr: data,
            })
        }
    }

    fn reset(&mut self) -> () {
        self.burst_aa.reset();
        self.burst_a4.reset();
    }
}
