use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, CharacterAttack, AtkQueue};
use crate::sim2::types::{CharacterAction, DamageType, Vision, FieldCharacterIndex, WeaponType, FieldEnergy, Particle, VecFieldEnergy, ToNaAction};
use crate::sim2::element::{ElementalGauge, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::sim2::record::{CharacterRecord, CharacterData, Enemy};

use WeaponType::*;
use Vision::*;

// When Rosaria strikes an opponent from behind using Ravaging Confession,
// Rosaria's CRIT Rate increases by 12% for 5s.

// Casting Rites of Termination increases CRIT Rate of all nearby party members
// (except Rosaria herself) by 15% of Rosaria's CRIT Rate for 10s. CRIT Rate
// Bonus gained this way cannot exceed 15%.
#[derive(Debug)]
pub struct Rosaria {
    skill_time: f32,
    burst_time: f32,
}

impl Rosaria {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Rosaria").vision(Cryo).weapon(Polearm).version(1.4)
            .base_hp(12289.0).base_atk(240.0).base_def(710.0)
            .atk(24.0)
            .energy_cost(60.)
    }

    pub fn new() -> Self {
        Self {
            skill_time: -99.,
            burst_time: -99.,
        }
    }
}

impl Timeline for Rosaria {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // check if skill can be used
        if state.rel_time.press >= 6. {
            CharacterAction::PressSkill
        // is burst CD off and has enough energy
        } else if state.rel_time.burst >= 15. && state.energy >= 60. {
            CharacterAction::Burst
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.5466 {
            // 5 attacks in 2.733 seconds
            data.na_idx.to_na(5, state.na_carryover(0.5466))
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        match event {
            CharacterAction::PressSkill => field_energy.push_p(Particle::new(data.character.vision, 3.)),
            _ => (),
        }
    }
}

impl CharacterAttack for Rosaria {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.apply_burst(187.2, &CRYO_GAUGE1A, time, event, data, state);
        atk_queue.apply_burst(273.6, &CRYO_GAUGE1A, time+0.1111, event, data, state);
        for i in 1..5 {
            atk_queue.apply_burst(237.6, &CRYO_GAUGE1A, time + (2 * i) as f32, event, data, state);
        }
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(105.21, &CRYO_GAUGE1A, time, event, data, state);
        atk_queue.add_skill(244.80, &CRYO_GAUGE1A, time+0.1111, event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(103.7, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(102.0, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(62.9, &PHYSICAL_GAUGE, time, event, data, state);
        atk_queue.add_na(62.9, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(137.7, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na5(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(82.28, &PHYSICAL_GAUGE, time, event, data, state);
        atk_queue.add_na(85.00, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_burst() {
            self.burst_time = action_state.current_time;
        } else if action_state.did_skill() {
            self.skill_time = action_state.current_time;
        }
        if attack.idx == data.idx {
            if attack.time - self.skill_time <= 5. {
                state.cr += 12.;
            }
        } else {
            if attack.time - self.burst_time <= 10. {
                state.cr += 15.;
            }
        }
    }

    fn reset_modify(&mut self) -> () {
        self.skill_time = -99.;
        self.burst_time = -99.;
    }
}
