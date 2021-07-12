use std::mem;
use crate::state::State;
use crate::types::{AttackType, Vision, ElementalGaugeDecay};
use crate::fc::{SpecialAbility, FieldCharacter, FieldAction, CharacterRecord, Enemy};
use crate::action::{Attack, TimerGuard, EffectTimer, DurationTimer, DotTimer};

use AttackType::*;
use Vision::*;
// use ElementalGaugeDecay::*;

// version 1.2

pub struct Albedo {
    burst_timer: DurationTimer,
    skill_aa: DotTimer,
}

impl Albedo {
    pub fn new() -> Self {
        Self {
            burst_timer: DurationTimer::new(12.0, 10.0),
            skill_aa: DotTimer::new(30.0, 2.0, 15),
        }
    }
}

impl SpecialAbility for Albedo {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Albedo").vision("Geo").weapon("Sword").release_date("2020-12-23").version(1.2)
            .base_hp(13226.0).base_atk(251.0).base_def(876.0)
            .dmg_geo(28.8)
            .na_1(72.62).na_2(72.62).na_3(93.81).na_4(98.35).na_5(122.7).na_6(0.0).na_time(2.667)
            // .na_0(0.0).ca_1(0.0).ca_2(0.0).ca_time(0.0)
            .press_cd(4.0).press_particle(0.0).press_dmg(234.73)
            .burst_cd(12.0).energy_cost(40.0).burst_dmg(660.96 + 129.6 * 3.0)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.burst_timer.update(gaurd.second(attack.iter().any(|a| a.kind == Burst)), time);
        self.skill_aa.update(gaurd.second(attack.iter().any(|a| a.kind == Skill)), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.skill_aa.is_active() {
            atk_queue.push(Attack {
                kind: SkillDot,
                element: Geo,
                multiplier: 234.72,
                particle: Some(0.8),
                state: None,
                icd_cleared: fa.burst.icd.clear(),
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
    }

    fn modify(&self, modifiable_state: &mut [State], _owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        // a4
        if self.burst_timer.is_active() {
            for s in modifiable_state.iter_mut() {
                s.em += 125.0;
            }
        }
    }

    fn reset(&mut self) -> () {
        self.burst_timer.reset();
        self.skill_aa.reset();
    }
}

pub struct Ganyu {
    burst_aa: DotTimer,
}

impl Ganyu {
    pub fn new() -> Self {
        Self {
            burst_aa: DotTimer::new(15.0, 1.0, 18),
        }
    }
}

impl SpecialAbility for Ganyu {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Ganyu").vision("Cryo").weapon("Bow").release_date("2021-01-12").version(1.2)
            .base_hp(9797.0).base_atk(335.0).base_def(630.0)
            .cd(88.4)
            .na_1(0.0).na_2(0.0).na_3(0.0).na_4(0.0).na_5(0.0).na_6(0.0).na_time(2.1)
            .na_0(0.0).ca_1(230.4).ca_2(391.68).ca_time(2.466)
            .press_cd(10.0).press_particle(4.0).press_dmg(237.6*2.0)
            .burst_cd(15.0).energy_cost(60.0).burst_dmg(0.0)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.burst_aa.update(gaurd.second(attack.iter().any(|a| a.kind == Burst)), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.burst_aa.is_active() {
            atk_queue.push(Attack {
                kind: BurstDot,
                element: Cryo,
                multiplier: 126.49,
                particle: None,
                state: None,
                icd_cleared: fa.burst.icd.clear(),
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
    }

    fn modify(&self, modifiable_state: &mut [State], _owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        // a4
        if self.burst_aa.is_active() {
            for s in modifiable_state.iter_mut() {
                s.cryo_dmg += 20.0;
            }
        }
    }

    // a1
    fn intensify(&self, attack: &mut Attack, _owner_fc: &FieldCharacter, _enemy: &Enemy) -> () {
        if attack.kind == Ca {
            let mut state: Option<State> = None;
            mem::swap(&mut state, &mut attack.state);
            attack.state = if let Some(mut state) = state {
                state.cr += 20.0;
                Some(state)
            } else {
                Some(State::new().cr(20.0))
            }
        }
    }

    fn reset(&mut self) -> () {
        self.burst_aa.reset();
    }
}
