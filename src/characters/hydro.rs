use crate::state::State;
use crate::types::{AttackType, Vision, ElementalGaugeDecay};
use crate::fc::{SpecialAbility, FieldCharacter, FieldAction, CharacterRecord, Enemy};
use crate::action::{Attack, TimerGuard, EffectTimer, DurationTimer, DotTimer};

use AttackType::*;
use Vision::*;
use ElementalGaugeDecay::*;

// version 1.0

pub struct Barbara;

impl SpecialAbility for Barbara {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Barbara").vision("Hydro").weapon("Catalyst").release_date("2020-09-28").version(1.0)
            .base_hp(9787.0).base_atk(159.0).base_def(669.0)
            .hp(24.0)
            .na_1(68.11).na_2(63.94).na_3(73.87).na_4(99.36).na_5(0.0).na_6(0.0).na_time(1.5)
            // .na_0(0.0).ca_1(299.23).ca_2(0.0).ca_time(1.7)
            .press_cd(32.0).press_particle(0.0).press_dmg(0.0)
            .burst_cd(20.0).energy_cost(80.0).burst_dmg(0.0)
    }
}

pub struct Xingqiu {
    burst_aa: DotTimer,
}

impl Xingqiu {
    pub fn new() -> Self {
        Self {
            burst_aa: DotTimer::new(20.0, 1.233, 13),
        }
    }
}

impl SpecialAbility for Xingqiu {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Xingqiu").vision("Hydro").weapon("Sword").release_date("2020-09-28").version(1.0)
            .base_hp(10222.0).base_atk(202.0).base_def(758.0)
            .atk(24.0)
            // a4
            .dmg_hydro(20.0)
            .na_1(92.14).na_2(94.18).na_3(56.44*2.0).na_4(110.67).na_5(70.89*2.0).na_6(0.0).na_time(2.833)
            // .na_0(0.0).ca_1(0.0).ca_2(0.0).ca_time(0.0)
            .press_cd(21.0).press_particle(4.0).press_dmg(302.4 + 344.16)
            .burst_cd(20.0).energy_cost(80.0).burst_dmg(0.0)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.burst_aa.update(gaurd.second(attack.iter().any(|a| a.kind == Burst)), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.burst_aa.is_active() {
            atk_queue.push(Attack {
                kind: BurstDot,
                element: Hydro,
                multiplier: 103.12 * 3.0,
                particle: None,
                state: None,
                icd_cleared: fa.burst.icd.clear(),
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
    }

    fn reset(&mut self) -> () {
        self.burst_aa.reset();
    }
}

pub struct Mona {
    omen_timer: DurationTimer,
    skill_aa: DotTimer,
}

impl Mona {
    pub fn new() -> Self {
        Self {
            omen_timer: DurationTimer::new(15.0, 5.0),
            skill_aa: DotTimer::new(12.0, 1.0, 5),
        }
    }
}

impl SpecialAbility for Mona {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Mona").vision("Hydro").weapon("Catalyst").release_date("2020-09-28").version(1.0)
            .base_hp(10409.0).base_atk(287.0).base_def(653.0)
            .er(32.0)
            .na_1(67.68).na_2(64.8).na_3(80.64).na_4(101.09).na_5(0.0).na_6(0.0).na_time(1.6)
            // .na_0(0.0).ca_1(0.0).ca_2(0.0).ca_time(0.0)
            .press_cd(12.0).press_particle(3.0).press_dmg(239.04)
            .burst_cd(15.0).energy_cost(60.0).burst_dmg(796.32)
            .burst_unit(2.0).burst_decay(B)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.omen_timer.update(gaurd.second(attack.iter().any(|a| a.kind == Burst)), time);
        self.skill_aa.update(gaurd.second(attack.iter().any(|a| a.kind == Skill)), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.skill_aa.is_active() {
            atk_queue.push(Attack {
                kind: SkillDot,
                element: Hydro,
                multiplier: 57.6,
                particle: None,
                state: None,
                icd_cleared: fa.skill.icd.clear(),
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        // a4
        let er = 100.0 + owner_fc.state.er;
        modifiable_state[owner_fc.idx.0].hydro_dmg += er * 0.2;
        if self.omen_timer.is_active() {
            for s in modifiable_state.iter_mut() {
                s.all_dmg += 60.0;
            }
        }
    }

    fn reset(&mut self) -> () {
        self.omen_timer.reset();
    }
}
