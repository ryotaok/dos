use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, CharacterAttack, AtkQueue};
use crate::sim2::types::{CharacterAction, DamageType, Vision, FieldCharacterIndex, WeaponType, FieldEnergy, Particle, VecFieldEnergy, ToNaAction};
use crate::sim2::element::{PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::sim2::record::{CharacterRecord, CharacterData, Enemy};

use WeaponType::*;
use Vision::*;

// version 1.0

// The Stamina Consumption of characters within Let the Show Begin's Melody Loop
// is reduced by 12%.

// When your active character gains an Elemental Orb/Particle, the duration of
// Let the Show Begin's Melody Loop is extended by 1s. The maximum extension is
// 5s.
#[derive(Debug)]
pub struct Barbara {}

impl Barbara {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Barbara").vision(Hydro).weapon(Catalyst).version(1.0)
            .base_hp(9787.0).base_atk(159.0).base_def(669.0)
            .hp(24.0)
            .energy_cost(80.)
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Timeline for Barbara {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // is burst CD off and has enough energy
        if state.rel_time.burst >= 20. && state.energy >= 80. {
            CharacterAction::Burst
        // check if skill can be used
        } else if state.rel_time.press >= 32. {
            CharacterAction::PressSkill
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.375 {
            // 4 attacks in 1.5 seconds
            data.na_idx.to_na(4, state.na_carryover(0.375))
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    // fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
    // }
}

impl CharacterAttack for Barbara {
    //  fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
    // }

    // fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
    // }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(68.11, &HYDRO_GAUGE1A, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(63.94, &HYDRO_GAUGE1A, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(73.87, &HYDRO_GAUGE1A, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(99.36, &HYDRO_GAUGE1A, time, event, data, state);
    }
}

// When a Rain Sword is shattered or when its duration expires, it regenerates
// the current character's HP based on 6% of Xingqiu's Max HP.

// Xingqiu gains a 20% Hydro DMG Bonus.
#[derive(Debug)]
pub struct Xingqiu {}

impl Xingqiu {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Xingqiu").vision(Hydro).weapon(Sword).version(1.0)
            .base_hp(10222.0).base_atk(202.0).base_def(758.0)
            .atk(24.0)
            // a4
            .hydro_dmg(20.0)
            .energy_cost(80.)
    }

    pub fn new() -> Self {
        Self {}
    }

}

impl Timeline for Xingqiu {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // is burst CD off and has enough energy
        if state.rel_time.burst >= 20. && state.energy >= 80. {
            CharacterAction::Burst
        // check if skill can be used
        } else if state.rel_time.press >= 21. {
            CharacterAction::PressSkill
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.5666 {
            // 5 attacks in 2.833 seconds
            data.na_idx.to_na(5, state.na_carryover(0.5666))
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

impl CharacterAttack for Xingqiu {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        for i in 0..13 {
            let t = time + 1.233 * i as f32;
            atk_queue.add_burst(103.12, &HYDRO_GAUGE1A, t, event, data, state);
            atk_queue.add_burst(103.12, &HYDRO_GAUGE1A, t, event, data, state);
            atk_queue.add_burst(103.12, &HYDRO_GAUGE1A, t, event, data, state);
        }
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(302.40, &HYDRO_GAUGE1A, time, event, data, state);
        atk_queue.add_skill(344.16, &HYDRO_GAUGE1A, time+0.2222, event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(92.14, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(94.18, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(56.44, &PHYSICAL_GAUGE, time, event, data, state);
        atk_queue.add_na(56.44, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(110.67, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na5(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(70.89, &PHYSICAL_GAUGE, time, event, data, state);
        atk_queue.add_na(70.89, &PHYSICAL_GAUGE, time, event, data, state);
    }

    // fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
    // }

    // fn reset_modify(&mut self) -> () {
    // }
}

// After she has used Illusory Torrent for 2s, if there are any opponents
// nearby, Mona will automatically create a Phantom. A Phantom created in this
// manner lasts for 2s, and its explosion DMG is equal to 50% of Mirror
// Reflection of Doom.

// Increases Mona's Hydro DMG Bonus by a degree equivalent to 20% of her Energy
// Recharge rate.
#[derive(Debug)]
pub struct Mona {
    burst_time: f32,
}

impl Mona {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Mona").vision(Hydro).weapon(Catalyst).version(1.0)
            .base_hp(10409.0).base_atk(287.0).base_def(653.0)
            .er(32.0)
            .energy_cost(60.)
    }

    pub fn new() -> Self {
        Self {
            burst_time: -99.
        }
    }
}

impl Timeline for Mona {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // is burst CD off and has enough energy
        if state.rel_time.burst >= 15. && state.energy >= 60. {
            CharacterAction::Burst
        // check if skill can be used
        } else if state.rel_time.press >= 12. {
            CharacterAction::PressSkill
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.375 {
            // 4 attacks in 1.5 seconds
            data.na_idx.to_na(4, state.na_carryover(0.375))
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

impl CharacterAttack for Mona {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_burst(796.32, &HYDRO_GAUGE2B, time, event, data, state);
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        for i in 0..4 {
            atk_queue.add_skill(57.6, &HYDRO_GAUGE1A, time + i as f32, event, data, state);
        }
        atk_queue.add_skill(239.04, &HYDRO_GAUGE1A, time+5., event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(67.68, &HYDRO_GAUGE1A, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(64.8, &HYDRO_GAUGE1A, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(80.64, &HYDRO_GAUGE1A, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(101.09, &HYDRO_GAUGE1A, time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx {
            // a4
            let er = 100.0 + action_state.er;
            state.hydro_dmg += er * 0.2;
        }
        if action_state.did_burst() {
            self.burst_time = action_state.current_time;
        }
        if attack.time - self.burst_time <= 5. {
            state.all_dmg += 60.;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.burst_time = -99.;
    }
}
