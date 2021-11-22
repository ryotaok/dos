use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, CharacterAttack, AtkQueue};
use crate::sim2::types::{CharacterAction, DamageType, Vision, FieldCharacterIndex, WeaponType, FieldEnergy, Particle, VecFieldEnergy, ToNaAction};
use crate::sim2::element::{ElementalGauge, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::sim2::record::{CharacterRecord, CharacterData, Enemy};

use WeaponType::*;
use Vision::*;

// When your current active character obtains or refreshes a Blazing Barrier,
// this character's Shield Strength will increase by 5% for 6s. This effect can
// be triggered once every 0.3s seconds. Max 5 stacks.

// DMG dealt by Crimson Ooyoroi's Fiery Collapse is increased by 2.2% of Thoma's
// Max HP.
#[derive(Debug)]
pub struct Thoma {
    burst_time: f32,
}

impl Thoma {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Thoma").vision(Pyro).weapon(Polearm).version(2.2)
            .base_hp(10331.).base_atk(202.).base_def(751.)
            .atk(24.)
            .energy_cost(80.)
    }

    pub fn new() -> Self {
        Self {
            burst_time: -99.,
        }
    }
}

impl Timeline for Thoma {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // check if skill can be used
        if state.rel_time.press >= 15. {
            CharacterAction::PressSkill
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.4875 {
            // 4 attacks in 1.95 seconds
            data.na_idx.to_na(4, state.na_carryover(0.4875))
        // is burst CD off and has enough energy
        } else if state.rel_time.burst >= 20. && state.energy >= 80. {
            CharacterAction::Burst
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        match event {
            CharacterAction::PressSkill => field_energy.push_p(Particle::new(data.character.vision, 4.)),
            _ => (),
        }
    }
}

impl CharacterAttack for Thoma {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_burst(158.4, &PYRO_GAUGE2B, time, event, data, state);
        for i in 1..16 {
            atk_queue.add_burst(104.4, &PYRO_GAUGE1A, time + i as f32, event, data, state);
        }
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(263.52, &PYRO_GAUGE2B, time, event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(87.75, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(86.24, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(52.96, &PHYSICAL_GAUGE, time, event, data, state);
        atk_queue.add_na(52.96, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(133.14, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_burst() {
            self.burst_time = action_state.current_time;
        }
        if attack.idx == data.idx && attack.kind == DamageType::Burst && attack.time - self.burst_time <= 15. {
            state.flat_dmg += 0.022 * state.HP();
        }
    }

    fn reset_modify(&mut self) -> () {
        self.burst_time = -99.;
    }
}
