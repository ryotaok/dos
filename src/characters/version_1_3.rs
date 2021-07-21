use crate::state::State;
use crate::types::{AttackType, Vision, ElementalGaugeDecay};
use crate::fc::{SpecialAbility, FieldCharacter, FieldAction, CharacterRecord, Enemy};
use crate::action::{Attack, TimerGuard, EffectTimer, DurationTimer, DotTimer, StackTimer};

use AttackType::*;
use Vision::*;
use ElementalGaugeDecay::*;

pub struct Xiao {
    burst_timer: DurationTimer,
    skill_a1: StackTimer,
}

impl Xiao {
    pub fn new() -> Self {
        Self {
            burst_timer: DurationTimer::new(18.0, 15.0),
            skill_a1: StackTimer::new(3.0, 4.0, 5),
        }
    }
}

impl SpecialAbility for Xiao {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Xiao").vision(Anemo).weapon(Polearm).release_date("2020-12-23").version(1.3)
            .base_hp(12736.0).base_atk(349.0).base_def(799.0)
            .cr(24.2)
            .na_1(49.14*2.0).na_2(101.58).na_3(122.3).na_4(67.2*2.0).na_5(127.64).na_6(170.97).na_time(4.1)
            .na_0(0.0).ca_1(404.02).ca_2(0.0).ca_time(1.333)
            .press_cd(10.0).press_particle(3.0).press_dmg(455.04)
            .burst_cd(18.0).energy_cost(70.0).burst_dmg(0.0)
            .skill_unit(2.0).skill_decay(B)
    }

    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.burst_timer.update(guard.second(attack.iter().any(|a| a.kind == Burst)), time);
        self.skill_a1.update(guard.second(true), time);
    }

    // TODO a4 is disabled for now
    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        if self.burst_timer.is_active() {
            modifiable_state[data.idx.0].infusion = true;
            modifiable_state[data.idx.0].na_dmg += 95.2;
            modifiable_state[data.idx.0].ca_dmg += 95.2;
        }
        if self.burst_timer.is_active() && self.skill_a1.is_active() {
            modifiable_state[data.idx.0].all_dmg += 5.0 * self.skill_a1.n as f32;
        }
    }

    fn reset(&mut self) -> () {
        self.burst_timer.reset();
        self.skill_a1.reset();
    }
}

pub struct HuTao {
    skill_timer: DurationTimer,
    skill_expire: DurationTimer,
    skill_aa: DotTimer,
}

impl HuTao {
    pub fn new() -> Self {
        Self {
            skill_timer: DurationTimer::new(16.0, 9.0),
            skill_expire: DurationTimer::new(0.0, 8.0),
            skill_aa: DotTimer::new(8.0, 4.0, 2),
        }
    }
}

impl SpecialAbility for HuTao {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Hu Tao").vision(Pyro).weapon(Polearm).release_date("2021-01-12").version(1.3)
            .base_hp(15552.0).base_atk(106.0).base_def(876.0)
            .cd(88.4)
            // a4
            .dmg_pyro(33.0)
            .na_1(0.0).na_2(0.0).na_3(0.0).na_4(0.0).na_5(0.0).na_6(0.0).na_time(2.733)
            .na_0(83.65).ca_1(242.57).ca_2(0.0).ca_time(0.915)
            .press_cd(16.0).press_particle(4.0).press_dmg(0.0)
            .burst_cd(15.0).energy_cost(60.0).burst_dmg(617.44)
            .burst_unit(2.0).burst_decay(B)
    }

    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        let before = self.skill_timer.is_active();
        self.skill_timer.update(guard.second(attack.iter().any(|a| a.kind == Skill)), time);
        let after = self.skill_timer.is_active();
        self.skill_aa.update(guard, time);
        self.skill_expire.update(guard.second(before && !after), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        if self.skill_aa.is_active() {
            atk_queue.push(Attack {
                kind: SkillDot,
                element: Pyro,
                multiplier: 115.2,
                particle: None,
                state: None,
                icd_cleared: fa.burst.icd.clear(),
                on_field_character_index: data.idx.0,
                fc_ptr: data,
            })
        }
    }

    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        if self.skill_timer.is_active() {
            modifiable_state[data.idx.0].infusion = true;
            modifiable_state[data.idx.0].flat_atk += data.state.HP() * 0.0626;
        }
        // a1
        if self.skill_expire.is_active() {
            for (i, s) in modifiable_state.iter_mut().enumerate() {
                if i != data.idx.0 {
                    s.cr += 12.0;
                }
            }
        }
    }

    fn reset(&mut self) -> () {
        self.skill_timer.reset();
        self.skill_expire.reset();
        self.skill_aa.reset();
    }
}
