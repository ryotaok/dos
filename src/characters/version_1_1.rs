use crate::state::State;
use crate::types::{AttackType, Vision, ElementalGaugeDecay};
use crate::fc::{SpecialAbility, FieldCharacter, FieldAction, CharacterRecord, Enemy};
use crate::action::{Attack, TimerGuard, EffectTimer, DurationTimer, HitsTimer, DotTimer};

use AttackType::*;
use Vision::*;
use ElementalGaugeDecay::*;

// version 1.1

pub struct Tartaglia {
    skill_aa: HitsTimer,
    skill_timer: DurationTimer,
}

impl Tartaglia {
    pub fn new() -> Self {
        Self {
            skill_aa: HitsTimer::new(1.5, 1),
            skill_timer: DurationTimer::new(45.0, 30.0),
        }
    }
}

impl SpecialAbility for Tartaglia {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Tartaglia").vision(Hydro).weapon(Bow).release_date("2020-11-11").version(1.1)
            .base_hp(13103.0).base_atk(301.0).base_def(815.0)
            .dmg_hydro(28.8)
            .na_1(76.84).na_2(82.28).na_3(111.35).na_4(118.49).na_5(109.31).na_6(70.04+74.46).na_time(2.316)
            // .na_0(0.0).ca_1(0.0).ca_2(0.0).ca_time(0.934)
            .press_cd(45.0).press_particle(15.0).press_dmg(122.4)
            .burst_cd(15.0).energy_cost(60.0).burst_dmg(835.2 + 216.0)
            .burst_unit(2.0).burst_decay(B)
    }

    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        let mut na = false;
        let mut skill = false;
        for a in attack {
            match a.kind {
                Na => na = true,
                Skill => skill = true,
                _ => (),
            }
        }
        self.skill_timer.update(guard.second(skill), time);
        self.skill_aa.update(guard.second(na), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        if self.skill_timer.is_active() && self.skill_aa.is_active() {
            atk_queue.push(Attack {
                kind: SkillDot,
                element: Hydro,
                multiplier: 119.0,
                particle: None,
                state: None,
                icd_cleared: fa.skill.icd.clear(),
                on_field_character_index: data.idx.0,
                fc_ptr: data,
            })
        }
    }

    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        let state = &mut modifiable_state[data.idx.0];
        if self.skill_timer.is_active() {
            state.infusion = true;
        }
    }

    fn reset(&mut self) -> () {
        self.skill_aa.reset();
    }
}

pub struct Diona {
    burst_aa: DotTimer,
}

impl Diona {
    pub fn new() -> Self {
        Self {
            burst_aa: DotTimer::new(20.0, 2.0, 6),
        }
    }
}

impl SpecialAbility for Diona {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Diona").vision(Cryo).weapon(Bow).release_date("2020-11-11").version(1.1)
            .base_hp(9570.0).base_atk(212.0).base_def(601.0)
            .dmg_cryo(24.0)
            .na_1(71.4).na_2(66.3).na_3(90.1).na_4(85.0).na_5(106.25).na_6(0.0).na_time(2.233)
            // TODO
            .na_0(0.0).ca_1(0.0).ca_2(0.0).ca_time(2.0)
            .press_cd(15.0).press_particle(4.5).press_dmg(75.46*5.0)
            .burst_cd(20.0).energy_cost(80.0).burst_dmg(144.0)
    }

    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.burst_aa.update(guard.second(attack.iter().any(|a| a.owned(data) && a.kind == Burst)), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        if self.burst_aa.is_active() {
            atk_queue.push(Attack {
                kind: BurstDot,
                element: Cryo,
                multiplier: 94.75,
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

pub struct Zhongli {
    skill_aa: DotTimer,
}

impl Zhongli {
    pub fn new() -> Self {
        Self {
            skill_aa: DotTimer::new(30.0, 2.0, 15),
        }
    }
}

impl SpecialAbility for Zhongli {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Zhongli").vision(Geo).weapon(Polearm).release_date("2020-12-01").version(1.1)
            .base_hp(14695.0).base_atk(251.0).base_def(738.0)
            .dmg_geo(28.8)
            .na_1(60.82).na_2(61.58).na_3(76.26).na_4(84.88).na_5(21.25*4.0).na_6(107.73).na_time(3.117)
            // .na_0(0.0).ca_1(0.0).ca_2(0.0).ca_time(0.0)
            .press_cd(4.0).press_particle(0.0).press_dmg(28.8)
            .burst_cd(12.0).energy_cost(40.0).burst_dmg(899.72)
            .skill_unit(2.0).skill_decay(B).burst_unit(4.0).burst_decay(C)
    }

    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.skill_aa.update(guard.second(attack.iter().any(|a| a.owned(data) && a.kind == Skill)), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        if self.skill_aa.is_active() {
            atk_queue.push(Attack {
                kind: SkillDot,
                element: Geo,
                multiplier: 57.6,
                particle: Some(0.5),
                state: None,
                icd_cleared: fa.skill.icd.clear(),
                on_field_character_index: data.idx.0,
                fc_ptr: data,
            })
        }
    }

    fn reset(&mut self) -> () {
        self.skill_aa.reset();
    }
}

pub struct Xinyan {
    burst_aa: DotTimer,
    burst_phy: HitsTimer,
    skill_aa: DotTimer,
    skill_a4: DurationTimer,
}

impl Xinyan {
    pub fn new() -> Self {
        Self {
            burst_aa:  DotTimer::new(15.0, 0.5, 4),
            burst_phy: HitsTimer::new(15.0, 1),
            skill_aa: DotTimer::new(18.0, 2.0, 6),
            skill_a4: DurationTimer::new(0.0, 12.0),
        }
    }
}

impl SpecialAbility for Xinyan {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Xinyan").vision(Pyro).weapon(Claymore).release_date("2020-12-01").version(1.1)
            .base_hp(11201.0).base_atk(249.0).base_def(799.0)
            .atk(24.0)
            .na_1(151.3).na_2(146.2).na_3(188.7).na_4(228.99).na_5(0.0).na_6(0.0).na_time(2.8)
            // .na_0(0.0).ca_1(0.0).ca_2(0.0).ca_time(0.0)
            .press_cd(18.0).press_particle(4.0).press_dmg(305.28)
            .burst_cd(15.0).energy_cost(60.0).burst_dmg(0.0)
    }

    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.burst_aa.update(guard.second(attack.iter().any(|a| a.owned(data) && a.kind == Burst)), time);
        self.burst_phy.update(guard, time);
        self.skill_aa.update(guard.second(attack.iter().any(|a| a.owned(data) && a.kind == Skill)), time);
        self.skill_a4.update(guard, time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        if self.skill_aa.is_active() {
            atk_queue.push(Attack {
                kind: SkillDot,
                element: Pyro,
                multiplier: 60.48,
                particle: None,
                state: None,
                icd_cleared: fa.skill.icd.clear(),
                on_field_character_index: data.idx.0,
                fc_ptr: data,
            })
        }
        if self.burst_aa.is_active() {
            atk_queue.push(Attack {
                kind: BurstDot,
                element: Pyro,
                multiplier: 72.0,
                particle: None,
                state: None,
                icd_cleared: fa.burst.icd.clear(),
                on_field_character_index: data.idx.0,
                fc_ptr: data,
            })
        }
        if self.burst_phy.is_active() {
            atk_queue.push(Attack {
                kind: BurstDot,
                element: Physical,
                multiplier: 613.44,
                particle: None,
                state: None,
                icd_cleared: fa.burst.icd.clear(),
                on_field_character_index: data.idx.0,
                fc_ptr: data,
            })
        }
    }

    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        if self.skill_a4.is_active() {
            for s in modifiable_state.iter_mut() {
                s.physical_dmg += 15.0;
            }
        }
    }

    fn reset(&mut self) -> () {
        self.burst_aa.reset();
        self.burst_phy.reset();
        self.skill_aa.reset();
        self.skill_a4.reset();
    }
}
