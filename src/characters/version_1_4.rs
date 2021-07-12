use crate::state::State;
use crate::types::{AttackType, Vision, ElementalGaugeDecay};
use crate::fc::{SpecialAbility, FieldCharacter, FieldAction, CharacterRecord, Enemy};
use crate::action::{Attack, TimerGuard, EffectTimer, DurationTimer, DotTimer};

use AttackType::*;
use Vision::*;
use ElementalGaugeDecay::*;

pub struct Rosaria {
    burst_aa: DotTimer,
    skill_a1: DurationTimer,
    burst_a4: DurationTimer,
}

impl Rosaria {
    pub fn new() -> Self {
        Self {
            burst_aa: DotTimer::new(15.0, 2.0, 4),
            skill_a1: DurationTimer::new(0.0, 5.0),
            burst_a4: DurationTimer::new(0.0, 10.0),
        }
    }
}

impl SpecialAbility for Rosaria {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Rosaria").vision("Cryo").weapon("Polearm").release_date("2020-12-23").version(1.4)
            .base_hp(12289.0).base_atk(240.0).base_def(710.0)
            .atk(24.0)
            .na_1(103.7).na_2(102.0).na_3(62.9*2.0).na_4(137.7).na_5(82.28 + 85.0).na_6(0.0).na_time(2.733)
            // .na_0(0.0).ca_1(0.0).ca_2(0.0).ca_time(0.0)
            .press_cd(6.0).press_particle(3.0).press_dmg(105.21 + 244.8)
            .burst_cd(15.0).energy_cost(60.0).burst_dmg(187.2 + 273.6)
    }

    fn update(&mut self, gaurd: &mut TimerGuard, attack: &[Attack], owner_fc: &FieldCharacter, _enemy: &Enemy, time: f32) -> () {
        self.burst_aa.update(gaurd.second(attack.iter().any(|a| a.owned(owner_fc) && a.kind == Burst)), time);
        self.skill_a1.update(gaurd.second(attack.iter().any(|a| a.owned(owner_fc) && a.kind == Skill)), time);
        self.burst_a4.update(gaurd.second(attack.iter().any(|a| a.owned(owner_fc) && a.kind == Burst)), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<Attack>, owner_fc: &FieldCharacter, fa: &FieldAction, _enemy: &Enemy) -> () {
        if self.burst_aa.is_active() {
            atk_queue.push(Attack {
                kind: BurstDot,
                element: Cryo,
                multiplier: 237.6,
                particle: None,
                state: None,
                icd_cleared: fa.burst.icd.clear(),
                on_field_character_index: owner_fc.idx.0,
                fc_ptr: owner_fc,
            })
        }
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        if self.skill_a1.is_active() {
            modifiable_state[owner_fc.idx.0].cr += 12.0;
        }
        if self.burst_a4.is_active() {
            for (i, s) in modifiable_state.iter_mut().enumerate() {
                if i != owner_fc.idx.0 {
                    s.cr += owner_fc.state.cr * 0.15;
                }
            }
        }
    }

    fn reset(&mut self) -> () {
        self.burst_aa.reset();
        self.skill_a1.reset();
        self.burst_a4.reset();
    }
}
