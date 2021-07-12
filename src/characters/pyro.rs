use std::mem;
use crate::state::State;
use crate::types::{AttackType, Vision, ElementalGaugeDecay};
use crate::fc::{SpecialAbility, FieldCharacter, FieldAction, CharacterRecord, Enemy};
use crate::action::{Attack, TimerGuard, EffectTimer, DurationTimer, HitsTimer, DotTimer, NormalAttackAction, SkillAction, BurstAction};
use crate::testutil;

use AttackType::*;
use Vision::*;
use ElementalGaugeDecay::*;

// version 1.0

pub struct Amber {
    ca_timer: DurationTimer,
}

impl Amber {
    pub fn new() -> Self {
        Self {
            ca_timer: DurationTimer::new(0.0, 10.0)
        }
    }
}

impl SpecialAbility for Amber {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Amber").vision("Pyro").weapon("Bow").release_date("2020-09-28").version(1.0)
            .base_hp(9461.0).base_atk(223.0).base_def(601.0)
            .atk(24.0)
            // TODO
            .ca_1(223.2).ca_time(2.0)
            .press_cd(15.0).press_particle(4.0).press_dmg(221.76)
            .burst_cd(12.0).energy_cost(40.0).burst_dmg(909.79)
            .ca_unit(2.0).ca_decay(B).skill_unit(2.0).skill_decay(B)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.ca_timer.update(gaurd.second(attack.iter().any(|a| a.owned(owner_fc) && a.kind == Ca)), time);
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.ca_timer.is_active() {
            // a4
            modifiable_state[owner_fc.idx.0].atk += 15.0;
        }
    }

    // a1
    fn intensify(&self, attack: &mut Attack, _owner_fc: &FieldCharacter, _enemy: &Enemy) -> () {
        if attack.kind == Burst {
            let mut state: Option<State> = None;
            mem::swap(&mut state, &mut attack.state);
            attack.state = if let Some(mut state) = state {
                state.cr += 10.0;
                Some(state)
            } else {
                Some(State::new().cr(10.0))
            }
        }
    }

    fn reset(&mut self) -> () {
        self.ca_timer.reset();
    }
}

pub struct Bennett {
    burst_timer: DurationTimer,
    bonus: f32,
}

impl Bennett {
    pub fn new() -> Self {
        Self {
            burst_timer: DurationTimer::new(0.0, 12.0),
            bonus: 1.008
        }
    }

    // pub fn c6() -> Self {
    //     Self { burst_timer: DurationTimer(0.0, 12.0), bonus: 1.19 }
    // }
}

impl SpecialAbility for Bennett {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Bennett").vision("Pyro").weapon("Sword").release_date("2020-09-28").version(1.0)
            .base_hp(12397.0).base_atk(191.0).base_def(771.0)
            .er(26.7)
            .na_1(88.06).na_2(84.49).na_3(107.95).na_4(117.98).na_5(142.12).na_time(2.567)
            // a1
            .press_cd(5.0 * 0.8).press_particle(2.0).press_dmg(261.44)
            .burst_cd(15.0).energy_cost(60.0).burst_dmg(419.04)
            .skill_unit(2.0).skill_decay(B).burst_unit(2.0).burst_decay(B)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.burst_timer.update(gaurd.second(attack.iter().any(|a| a.owned(owner_fc) && a.kind == Burst)), time);
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.burst_timer.is_active() {
            for s in modifiable_state.iter_mut() {
                s.flat_atk += owner_fc.state.base_atk * self.bonus;
            }
        }
    }

    fn accelerate(&self, _na: &mut NormalAttackAction, skill: &mut SkillAction, _burst: &mut BurstAction) -> () {
        if self.burst_timer.is_active() {
            // a4
            skill.spd += 100.0;
        }
    }

    fn reset(&mut self) -> () {
        self.burst_timer.reset();
    }
}

pub struct Xiangling {
    burst_aa: DotTimer,
    skill_aa: DotTimer,
    skill_a4: DurationTimer,
}

impl Xiangling {
    pub fn new() -> Self {
        Self {
            burst_aa: DotTimer::new(20.0, 1.0, 10),
            skill_aa: DotTimer::new(12.0, 2.0, 4),
            skill_a4: DurationTimer::new(0.0, 10.0)
        }
    }
}

impl SpecialAbility for Xiangling {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Xiangling").vision("Pyro").weapon("Polearm").release_date("2020-09-28").version(1.0)
            .base_hp(10875.0).base_atk(225.0).base_def(669.0)
            .em(96.0)
            .na_1(83.13).na_2(83.3).na_3(103.02).na_4(111.52).na_5(140.42).na_time(2.4)
            .press_cd(12.0).press_particle(0.0).press_dmg(0.0)
            .burst_cd(20.0).energy_cost(80.0).burst_dmg(129.6 + 158.4 + 197.28)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.burst_aa.update(gaurd.second(attack.iter().any(|a| a.owned(owner_fc) && a.kind == Burst)), time);
        self.skill_aa.update(gaurd.second(attack.iter().any(|a| a.owned(owner_fc) && a.kind == Skill)), time);
        self.skill_a4.update(gaurd.second(attack.iter().any(|a| a.owned(owner_fc) && a.kind == Skill)), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.skill_aa.is_active() {
            atk_queue.push(Attack {
                kind: SkillDot,
                element: Pyro,
                multiplier: 200.3,
                particle: Some(1.0),
                state: None,
                icd_cleared: fa.skill.icd.clear(),
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
        if self.burst_aa.is_active() {
            atk_queue.push(Attack {
                kind: BurstDot,
                element: Pyro,
                multiplier: 201.6,
                particle: None,
                state: None,
                icd_cleared: fa.burst.icd.clear(),
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
    }

    fn modify(&self, modifiable_state: &mut [State], _owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.skill_a4.is_active() {
            for s in modifiable_state.iter_mut() {
                s.atk += 10.0;
            }
        }
    }

    fn reset(&mut self) -> () {
        self.burst_aa.reset();
        self.skill_aa.reset();
        self.skill_a4.reset();
    }
}

pub struct Diluc {
    burst_timer: DotTimer,
    burst_a4: DurationTimer,
}

impl Diluc {
    pub fn new() -> Self {
        Self {
            burst_timer: DotTimer::new(12.0, 0.5, 3),
            burst_a4: DurationTimer::new(0.0, 12.0)
        }
    }
}

impl SpecialAbility for Diluc {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Diluc").vision("Pyro").weapon("Claymore").release_date("2020-09-28").version(1.0)
            .base_hp(12981.0).base_atk(335.0).base_def(784.0)
            .cr(24.2)
            .na_1(177.31).na_2(173.23).na_3(195.33).na_4(264.86).na_time(2.834)
            .press_cd(10.0).press_particle(4.0).press_dmg(169.92 + 175.68 + 231.84)
            .burst_cd(12.0).energy_cost(40.0).burst_dmg(367.2)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.burst_timer.update(gaurd.second(attack.iter().any(|a| a.owned(owner_fc) && a.kind == Burst)), time);
        self.burst_a4.update(gaurd, time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.burst_timer.is_active() {
            atk_queue.push(Attack {
                kind: BurstDot,
                element: Pyro,
                multiplier: 108.0,
                particle: None,
                state: None,
                icd_cleared: fa.burst.icd.clear(),
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.burst_a4.is_active() {
            modifiable_state[owner_fc.idx.0].pyro_dmg += 20.0;
            modifiable_state[owner_fc.idx.0].infusion = true;
        }
    }
}

pub struct Klee {
    burst_aa: DotTimer,
    skill_aa: DotTimer,
    ca_a4: HitsTimer,
    na_a1: HitsTimer,
}

impl Klee {
    pub fn new() -> Self {
        Self {
            burst_aa: DotTimer::new(15.0, 1.0, 6),
            skill_aa: DotTimer::new(20.0, 1.0, 4),
            ca_a4: HitsTimer::new(1.0, 1),
            na_a1: HitsTimer::new(1.0, 1),
        }
    }
}

impl SpecialAbility for Klee {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Klee").vision("Pyro").weapon("Catalyst").release_date("2020-09-28").version(1.0)
            .base_hp(10287.0).base_atk(311.0).base_def(615.0)
            .dmg_pyro(28.8)
            .na_1(129.89).na_2(112.32).na_3(161.86).na_time(1.467)
            .na_0(0.0).ca_1(0.0).ca_2(0.0).ca_time(1.334)
            .press_cd(20.0).press_particle(3.5).press_dmg(171.36 * 3.0)
            .burst_cd(15.0).energy_cost(60.0).burst_dmg(0.0)
            .skill_unit(2.0).skill_decay(B)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        let mut ca = false;
        let mut na_a1 = false;
        let mut skill = false;
        let mut burst = false;
        for a in attack {
            match a.kind {
                Burst => burst = true,
                Skill => {
                    skill = true;
                    na_a1 = if testutil::chance() < 0.5 { true } else { false };
                },
                Ca    => {
                    ca = true;
                    na_a1 = if testutil::chance() < 0.5 { true } else { false };
                },
                _ => (),
            }
        }
        self.burst_aa.update(gaurd.second(burst), time);
        self.skill_aa.update(gaurd.second(skill), time);
        self.ca_a4.update(gaurd.second(ca), time);
        self.na_a1.update(gaurd.second(na_a1), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.skill_aa.is_active() {
            atk_queue.push(Attack {
                kind: SkillDot,
                element: Pyro,
                multiplier: 59.04,
                particle: None,
                state: None,
                icd_cleared: fa.skill.icd.clear(),
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
        if self.burst_aa.is_active() {
            atk_queue.push(Attack {
                kind: BurstDot,
                element: Pyro,
                multiplier: 76.76 * 4.0,
                particle: None,
                state: None,
                icd_cleared: fa.burst.icd.clear(),
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
        if self.ca_a4.is_active() {
            atk_queue.push(Attack {
                kind: StandStill,
                element: Physical,
                multiplier: 0.0,
                particle: Some(1.0 * owner_fc.state.cr / 100.0),
                state: None,
                icd_cleared: false,
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
    }

    // a1
    fn intensify(&self, attack: &mut Attack, _owner_fc: &FieldCharacter, _enemy: &Enemy) -> () {
        if attack.kind == Ca && self.na_a1.is_active() {
            let mut state: Option<State> = None;
            mem::swap(&mut state, &mut attack.state);
            attack.state = if let Some(mut state) = state {
                state.ca_dmg += 50.0;
                Some(state)
            } else {
                Some(State::new().ca_dmg(50.0))
            }
        }
    }

    fn reset(&mut self) -> () {
        self.burst_aa.reset();
        self.skill_aa.reset();
        self.ca_a4.reset();
        self.na_a1.reset();
    }
}
