use crate::state::State;
use crate::types::{AttackType, Vision, ElementalGaugeDecay};
use crate::fc::{SpecialAbility, FieldCharacter, FieldAction, CharacterRecord, Enemy};
use crate::action::{Attack, TimerGuard, EffectTimer, DurationTimer, DotTimer, StackTimer};

use AttackType::*;
use Vision::*;
use ElementalGaugeDecay::*;

pub struct Ayaka {
    skill_a1: DurationTimer,
    burst_timer: DotTimer,
}

impl Ayaka {
    pub fn new() -> Self {
        Self {
            skill_a1: DurationTimer::new(0.0, 6.0),
            burst_timer: DotTimer::new(20.0, 0.5, 10),
        }
    }
}

impl SpecialAbility for Ayaka {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Ayaka").vision(Cryo).weapon(Sword).release_date("2021-07-20").version(2.0)
            .base_hp(12858.0).base_atk(342.0).base_def(784.0)
            .cd(88.4)
            .na_1(90.39).na_2(96.24).na_3(123.79).na_4(44.77 * 3.0).na_5(154.55).na_6(0.0).na_time(2.017)
            // .na_0(0.0).ca_1(0.0).ca_2(0.0).ca_time(1.9)
            .press_cd(10.0).press_particle(3.5).press_dmg(406.64)
            .burst_cd(20.0).energy_cost(80.0).burst_dmg(202.14)
            .skill_unit(2.0).skill_decay(B)
    }

    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        let mut skill = false;
        let mut burst = false;
        for a in attack {
            match a.kind {
                Skill => skill = true,
                Burst => burst = true,
                _ => (),
            }
        }
        self.skill_a1.update(guard.second(skill), time);
        self.burst_timer.update(guard.second(burst), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        if self.burst_timer.is_active() {
            atk_queue.push(Attack {
                kind: BurstDot,
                element: Cryo,
                multiplier: 303.21,
                particle: None,
                state: None,
                icd_cleared: fa.burst.icd.clear(),
                on_field_character_index: data.idx.0,
                fc_ptr: data,
            })
        }
    }

    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        let s = &mut modifiable_state[data.idx.0];
        // Alternate Sprint (Kamisato Art: Senho)
        s.infusion = true;
        if self.skill_a1.is_active() {
            s.na_dmg += 30.0;
            s.ca_dmg += 30.0;
        }
    }

    fn reset(&mut self) -> () {
        self.skill_a1.reset();
        self.burst_timer.reset();
    }
}

pub struct Yoimiya {
    skill_timer: DurationTimer,
    // skill_aa: HitsTimer,
    skill_a1: StackTimer,
    burst_aa: DotTimer,
    burst_a4: DurationTimer,
}

impl Yoimiya {
    pub fn new() -> Self {
        Self {
            skill_timer: DurationTimer::new(15.0, 10.0),
            // skill_aa: HitsTimer::new(0.0, 1),
            skill_a1: StackTimer::new(0.0, 3.0, 10),
            burst_aa: DotTimer::new(15.0, 2.0, 5),
            burst_a4: DurationTimer::new(0.0, 15.0),
        }
    }
}

impl SpecialAbility for Yoimiya {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Yoimiya").vision(Pyro).weapon(Bow).release_date("2021-08-10").version(2.0)
            .base_hp(10164.0).base_atk(323.0).base_def(615.0)
            .cr(24.2)
            .na_1(63.59*2.0).na_2(121.99).na_3(158.59).na_4(82.82*2.0).na_5(188.87).na_6(0.0).na_time(2.0)
            // .na_0(0.0).ca_1(0.0).ca_2(0.0).ca_time(0.0)
            .press_cd(18.0).press_particle(3.0).press_dmg(161.74)
            .burst_cd(15.0).energy_cost(60.0).burst_dmg(228.96)
    }

    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        let mut skill_and_na = false;
        let mut skill = false;
        let mut burst = false;
        for a in attack {
            match a.kind {
                Na    => skill_and_na = self.skill_timer.is_active(),
                Skill => skill = true,
                Burst => burst = true,
                _ => (),
            }
        }
        self.skill_timer.update(guard.second(skill), time);
        // self.skill_aa.update(guard.second(skill_and_na), time);
        self.skill_a1.update(guard.second(skill_and_na), time);
        self.burst_aa.update(guard.second(burst), time);
        self.burst_a4.update(guard.second(burst), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        // if self.skill_aa.is_active() {
        //     atk_queue.push(Attack {
        //         kind: Na,
        //         element: Pyro,
        //         multiplier: 161.74,
        //         particle: None,
        //         state: None,
        //         icd_cleared: fa.na.icd.clear(),
        //         on_field_character_index: data.idx.0,
        //         fc_ptr: data,
        //     })
        // }
        if self.burst_aa.is_active() {
            atk_queue.push(Attack {
                kind: BurstDot,
                element: Pyro,
                multiplier: 219.6,
                particle: None,
                state: None,
                icd_cleared: fa.burst.icd.clear(),
                on_field_character_index: data.idx.0,
                fc_ptr: data,
            })
        }
    }

    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        if self.burst_a4.is_active() {
            for (i, s) in modifiable_state.iter_mut().enumerate() {
                if i != data.idx.0 {
                    s.atk += 10.0 + self.skill_a1.n as f32;
                }
            }
        }
        let state = &mut modifiable_state[data.idx.0];
        if self.skill_a1.is_active() {
            state.pyro_dmg += 2.0 * self.skill_a1.n as f32;
        }
        if self.skill_timer.is_active() {
            state.infusion = true;
        }
        // if self.skill_timer.is_active() {
        //     state.na_dmg += 161.74;
        // }
    }

    fn reset(&mut self) -> () {
        self.skill_timer.reset();
        // self.skill_aa.reset();
        self.skill_a1.reset();
        self.burst_aa.reset();
        self.burst_a4.reset();
    }
}

pub struct Sayu {
    burst_aa: DotTimer,
}

impl Sayu {
    pub fn new() -> Self {
        Self {
            burst_aa: DotTimer::new(20.0, 2.0, 5),
        }
    }
}

impl SpecialAbility for Sayu {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Sayu").vision(Anemo).weapon(Claymore).release_date("2021-08-10").version(2.0)
            .base_hp(11854.0).base_atk(244.0).base_def(745.0)
            .em(96.0)
            .na_1(142.8).na_2(141.1).na_3(85.85*2.0).na_4(193.97).na_5(0.0).na_6(0.0).na_time(2.616)
            // .na_0(0.0).ca_1(0.0).ca_2(0.0).ca_time(0.0)
            .press_cd(6.0).press_particle(3.0).press_dmg(64.8 + 285.12 + 30.24 + 137.09)
            .burst_cd(20.0).energy_cost(80.0).burst_dmg(210.24)
            // .skill_unit(2.0).skill_decay(B).burst_unit(4.0).burst_decay(C)
    }

    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        let mut burst = false;
        for a in attack {
            match a.kind {
                Burst => burst = true,
                _ => (),
            }
        }
        self.burst_aa.update(guard.second(burst), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        if self.burst_aa.is_active() {
            atk_queue.push(Attack {
                kind: BurstDot,
                element: Anemo,
                multiplier: 93.6,
                particle: None,
                state: None,
                icd_cleared: fa.burst.icd.clear(),
                on_field_character_index: data.idx.0,
                fc_ptr: data,
            })
        }
    }

    fn reset(&mut self) -> () {
        self.burst_aa.reset();
    }
}
