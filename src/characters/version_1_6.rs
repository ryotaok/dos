use crate::state::State;
use crate::types::{AttackType, Vision, ElementalGaugeDecay};
use crate::fc::{SpecialAbility, FieldCharacter, FieldAction, CharacterRecord, Enemy};
use crate::action::{Attack, TimerGuard, EffectTimer, DurationTimer, DotTimer, HitsTimer};

use AttackType::*;
use Vision::*;
use ElementalGaugeDecay::*;

pub struct Kazuha {
    skill_aa: HitsTimer,
    burst_aa: DotTimer,
    swirl_a4: DurationTimer,
}

impl Kazuha {
    pub fn new() -> Self {
        Self {
            skill_aa: HitsTimer::new(9.0, 1),
            burst_aa: DotTimer::new(20.0, 2.0, 4),
            swirl_a4: DurationTimer::new(0.0, 8.0),
        }
    }
}

impl SpecialAbility for Kazuha {
    fn character(&self) -> CharacterRecord {
        CharacterRecord::default()
            .name("Kazuha").vision(Anemo).weapon(Sword).release_date("2021-06-29").version(1.6)
            .base_hp(13348.0).base_atk(297.0).base_def(807.0)
            .em(115.2)
            .na_1(88.91).na_2(89.42).na_3(51.0 + 61.2).na_4(120.02).na_5(50.15 * 3.0).na_6(0.0).na_time(2.066)
            // .na_0(0.0).ca_1(0.0).ca_2(0.0).ca_time(0.0)
            .press_cd(9.0).press_particle(4.0).press_dmg(469.44)
            .burst_cd(15.0).energy_cost(60.0).burst_dmg(472.32)
            .skill_unit(2.0).skill_decay(B).burst_unit(2.0).burst_decay(B)
    }

    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.skill_aa.update(guard.second(attack.iter().any(|a| a.kind == Skill)), time);
        self.burst_aa.update(guard.second(attack.iter().any(|a| a.kind == Burst)), time);
        // TODO remember the specific element on the enemy
        self.swirl_a4.update(guard.second(attack.iter().any(|a| enemy.trigger_er(&a.element).is_swirl())), time);
    }

    fn additional_attack(&self, atk_queue: &mut Vec<ElementalAttack>, particles: &mut Vec<Particle>, timers: &FullCharacterTimers, data: &CharacterData, enemy: &Enemy) -> () {
        if self.skill_aa.is_active() {
            // a1
            if enemy.aura.aura != Physical {
                atk_queue.push(Attack {
                    kind: Ca,
                    element: enemy.aura.aura,
                    multiplier: 200.0,
                    particle: None,
                    state: None,
                    icd_cleared: fa.na.icd.clear(),
                    on_field_character_index: data.idx.0,
                    fc_ptr: data,
                });
            }
            atk_queue.push(Attack {
                kind: Ca,
                element: Anemo,
                multiplier: 404.02,
                particle: None,
                state: None,
                icd_cleared: fa.na.icd.clear(),
                on_field_character_index: data.idx.0,
                fc_ptr: data,
            });
        }
        if self.burst_aa.is_active() {
            if enemy.aura.aura != Physical {
                atk_queue.push(Attack {
                    kind: BurstDot,
                    element: enemy.aura.aura,
                    multiplier: 64.8,
                    particle: None,
                    state: None,
                    icd_cleared: fa.burst.icd.clear(),
                    on_field_character_index: data.idx.0,
                    fc_ptr: data,
                });
            }
            atk_queue.push(Attack {
                kind: BurstDot,
                element: Anemo,
                multiplier: 216.0,
                particle: None,
                state: None,
                icd_cleared: fa.burst.icd.clear(),
                on_field_character_index: data.idx.0,
                fc_ptr: data,
            });
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
        self.skill_aa.reset();
        self.burst_aa.reset();
        self.swirl_a4.reset();
    }
}
