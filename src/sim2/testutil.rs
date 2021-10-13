use crate::sim2::element::PHYSICAL_GAUGE;
use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, CharacterAttack};
use crate::sim2::types::{CharacterAction, DamageType, Vision, FieldCharacterIndex, FieldEnergy, VecFieldEnergy, Particle, ToNaAction};
use crate::sim2::record::{TimelineMember, FieldMember, CharacterData, Enemy, CharacterRecord, WeaponRecord, Artifact};

use Vision::*;

#[derive(Debug)]
pub struct Sim2TestCharacter {
    na_idx: usize,
}

impl Sim2TestCharacter {
    pub fn new() -> Self {
        Self {
            na_idx: 1
        }
    }

    pub fn record(vision: Vision) -> CharacterRecord {
        CharacterRecord::default()
            .vision(vision)
            .base_atk(200.0)
            .cr(0.0).cd(0.0)
            .energy_cost(40.0)
    }
}

impl Timeline for Sim2TestCharacter {
    // perform an action
    fn decide_action(&mut self, state: &ActionState) -> CharacterAction {
        // is burst CD off and has enough energy
        if state.rel_time.burst >= 12.0 && state.energy >= 40.0 {
            CharacterAction::Burst
        // check if skill can be used
        } else if state.rel_time.press >= 6.0 {
            CharacterAction::PressSkill
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.4 {
            self.na_idx.to_na(4)
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState) -> () {
        match event {
            CharacterAction::PressSkill => field_energy.push_p(Particle::new(Pyro, 2.0)),
            _ => (),
        }
    }
}

impl CharacterAttack for Sim2TestCharacter {
    fn burst(&self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State) -> () {
        atk_queue.push(Attack {
            kind: DamageType::Burst,
            multiplier: 300.0,
            element: data.character.vision.to_gauge(),
            aura_application: state.apply_aura(time, event),
            time,
            idx: data.idx,
        });
    }

    fn press(&self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State) -> () {
        atk_queue.push(Attack {
            kind: DamageType::Skill,
            multiplier: 200.0,
            element: data.character.vision.to_gauge(),
            aura_application: state.apply_aura(time, event),
            time,
            idx: data.idx,
        });
    }

    fn na1(&self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State) -> () {
        atk_queue.push(Attack {
            kind: DamageType::Na,
            multiplier: 100.0,
            element: if state.infusion {
                data.character.vision.to_gauge()
            } else {
                &PHYSICAL_GAUGE
            },
            aura_application: state.apply_aura(time, event),
            time,
            idx: data.idx,
        });
    }

    fn na2(&self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State) -> () {
        self.na1(time, event, data, atk_queue, state);
    }

    fn na3(&self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State) -> () {
        self.na1(time, event, data, atk_queue, state);
    }

    fn na4(&self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State) -> () {
        self.na1(time, event, data, atk_queue, state);
    }

    fn modify(&self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {}
}

#[derive(Debug)]
pub struct NoopTimeline {}

impl Timeline for NoopTimeline {}
