use crate::state::State;
use crate::types::{AttackType, Vision, ElementalGaugeDecay};
use crate::fc::{SpecialAbility, FieldCharacter, FieldAction, CharacterRecord, Enemy};
use crate::action::{Attack, TimerGuard, EffectTimer, CDTimer, DurationTimer, HitsTimer, NormalAttackAction, SkillAction, BurstAction};

use AttackType::*;
use Vision::*;
use ElementalGaugeDecay::*;

// version 1.0

pub struct Ningguang {
    skill_timer: DurationTimer,
}

impl Ningguang {
    pub fn new() -> Self {
        Self {
            skill_timer: DurationTimer::new(12.0, 10.0),
        }
    }
}

impl SpecialAbility for Ningguang {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Ningguang").vision("Geo").weapon("Catalyst").release_date("2020-09-28").version(1.0)
            .base_hp(9787.0).base_atk(212.0).base_def(573.0)
            .dmg_geo(24.0)
            // .na_1(0.0).na_2(0.0).na_3(0.0).na_4(0.0).na_5(0.0).na_6(0.0).na_time(1.67)
            .na_0(50.4).ca_1(313.34).ca_2(84.32).ca_time(1.9)
            .press_cd(12.0).press_particle(3.0).press_dmg(414.72)
            .burst_cd(12.0).energy_cost(40.0).burst_dmg(156.53 * 5.0)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.skill_timer.update(gaurd.second(attack.iter().any(|a| a.kind == Skill)), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        // a4
        if self.skill_timer.is_active() {
            for s in modifiable_state.iter_mut() {
                s.geo_dmg += 12.0;
            }
        }
    }

    fn reset(&mut self) -> () {
        self.skill_timer.reset();
    }
}

pub struct Noelle {
    burst_timer: CDTimer,
    na_a4: HitsTimer,
}

impl Noelle {
    pub fn new() -> Self {
        Self {
            burst_timer: CDTimer::new(15.0),
            na_a4: HitsTimer::new(2.55, 1),
        }
    }
}

impl SpecialAbility for Noelle {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Noelle").vision("Geo").weapon("Claymore").release_date("2020-09-28").version(1.0)
            .base_hp(12071.0).base_atk(191.0).base_def(799.0)
            .def(30.0)
            .na_1(156.4).na_2(145.01).na_3(170.51).na_4(224.23).na_time(2.616)
            // .na_0(0.0).ca_1(0.0).ca_2(0.0).ca_time(0.0)
            .press_cd(24.0).press_particle(0.0).press_dmg(216.0)
            .burst_cd(15.0).energy_cost(60.0).burst_dmg(120.96 + 167.76)
            .skill_unit(2.0).skill_decay(B)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.burst_timer.update(gaurd.second(attack.iter().any(|a| a.kind == Burst)), time);
        self.na_a4.update(gaurd.second(attack.iter().any(|a| a.kind == Na || a.kind == Ca)), time);
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.burst_timer.is_active() {
            modifiable_state[owner_fc.idx.0].flat_atk += owner_fc.state.DEF() * 0.72;
            modifiable_state[owner_fc.idx.0].infusion = true;
        }
    }

    fn accelerate(&self, _na: &mut NormalAttackAction, skill: &mut SkillAction, _burst: &mut BurstAction) -> () {
        if self.na_a4.is_active() {
            // a4
            skill.cd -= 1.0;
        }
    }

    fn reset(&mut self) -> () {
        self.burst_timer.reset();
        self.na_a4.reset();
    }
}

pub struct TravelerGeo {
    na_1: HitsTimer,
}

impl TravelerGeo {
    pub fn new() -> Self {
        Self {
            na_1: HitsTimer::new(2.55, 1),
        }
    }
}

impl SpecialAbility for TravelerGeo {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Traveler (Geo)").vision("Geo").weapon("Sword").release_date("2020-09-28").version(1.0)
            .base_hp(10875.0).base_atk(212.0).base_def(683.0)
            .atk(24.0)
            .na_1(87.89).na_2(85.85).na_3(104.72).na_4(115.26).na_5(139.91).na_6(0.0).na_time(2.55)
            .na_0(0.0).ca_1(0.0).ca_2(0.0).ca_time(0.0)
            // a1
            .press_cd(8.0 - 2.0).press_particle(3.5).press_dmg(446.4)
            .burst_cd(15.0).energy_cost(60.0).burst_dmg(266.4 * 4.0)
            .skill_unit(2.0).skill_decay(B).burst_unit(2.0).burst_decay(B)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], _owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.na_1.update(gaurd.second(attack.iter().any(|a| a.kind == Na)), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.na_1.is_active() {
            atk_queue.push(Attack {
                kind: Na,
                element: Geo,
                multiplier: 60.0,
                particle: None,
                state: None,
                icd_cleared: fa.na.icd.clear(),
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
    }

    fn reset(&mut self) -> () {
        self.na_1.reset();
    }
}
