use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, CharacterAttack, AtkQueue};
use crate::sim2::types::{CharacterAction, DamageType, Vision, FieldCharacterIndex, WeaponType, FieldEnergy, Particle, VecFieldEnergy, ToNaAction};
use crate::sim2::element::{ElementalGauge, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, ELECTRO_GAUGE4C, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::sim2::record::{CharacterRecord, CharacterData, Enemy};

use WeaponType::*;
use Vision::*;

// version 1.0

// When Ningguang is in possession of Star Jades, her Charged Attack does not
// consume Stamina.

// A character that passes through the Jade Screen will gain a 12% Geo DMG Bonus
// for 10s.
#[derive(Debug)]
pub struct Ningguang {
    star_jade: u8,
    skill_time: f32,
}

impl Ningguang {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Ningguang").vision(Geo).weapon(Catalyst).version(1.0)
            .base_hp(9787.0).base_atk(212.0).base_def(573.0)
            .geo_dmg(24.0)
            .energy_cost(40.)
    }

    pub fn new() -> Self {
        Self {
            star_jade: 0,
            skill_time: -99.,
        }
    }
}

impl Timeline for Ningguang {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // check if skill can be used
        if state.rel_time.press >= 12. {
            CharacterAction::PressSkill
        // is burst CD off and has enough energy
        } else if state.rel_time.burst >= 12. && state.energy >= 40. {
            CharacterAction::Burst
        } else if data.idx.is_on_field() && self.star_jade > 1 && state.rel_time.na >= 0.8 {
            CharacterAction::Ca(state.ca_carryover(1.5))
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.8 && state.rel_time.ca >= 1.5 {
            // 1 attacks in 0.8 seconds
            CharacterAction::Na1(state.na_carryover(0.8))
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        match event {
            CharacterAction::PressSkill => field_energy.push_p(Particle::new(data.character.vision, 3.5)),
            CharacterAction::Ca(_) => self.star_jade = 0,
            CharacterAction::Na1(_) => self.star_jade += 1,
            _ => (),
        }
    }

    fn reset_timeline(&mut self) -> () {
        self.star_jade = 0;
    }
}

impl CharacterAttack for Ningguang {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        let star_jade = if time - self.skill_time <= 12. {
            12
        } else {
            6
        };
        for i in 0..star_jade {
            atk_queue.add_burst(156.53, &GEO_GAUGE1A, time + 0.1111 * i as f32, event, data, state);
        }
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.skill_time = time;
        atk_queue.add_skill(414.72, &GEO_GAUGE1A, time, event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.star_jade += 1;
        // 2 times of 50.4
        atk_queue.add_na(100.8, &GEO_GAUGE1A, time, event, data, state);
    }

    fn ca(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_ca(313.34, &GEO_GAUGE1A, time, event, data, state);
        for i in 0..self.star_jade {
            atk_queue.add_ca(89.28, &GEO_GAUGE1A, time, event, data, state);
        }
        self.star_jade = 0;
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_skill() {
            self.skill_time = action_state.current_time;
        }
        if attack.time - self.skill_time <= 10. {
            state.geo_dmg += 12.;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.star_jade = 0;
        self.skill_time = -99.;
    }
}

// When Noelle is in the party but not on the field, this ability triggers
// automatically when your active character's HP falls below 30%: Creates a
// shield for your active character that lasts for 20s and absorbs DMG equal to
// 400% of Noelle's DEF. The shield has a 150% DMG Absorption effectiveness
// against all Elemental and Physical DMG. This effect can only occur once every
// 60s.

// Every 4 Normal or Charged Attack hits will decrease the CD of Breastplate by
// 1s. Hitting multiple opponents with a single attack is only counted as 1 hit.
#[derive(Debug)]
pub struct Noelle {
    burst_time: f32,
}

impl Noelle {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Noelle (C6)").vision(Geo).weapon(Claymore).version(1.0)
            .base_hp(12071.0).base_atk(191.0).base_def(799.0)
            .def(30.0)
            .energy_cost(60.)
    }

    pub fn new() -> Self {
        Self {
            burst_time: -99.
        }
    }

    fn infusion(&self, time: f32) -> &'static ElementalGauge {
        if time - self.burst_time <= 15. {
            &GEO_GAUGE1A
        } else {
            &PHYSICAL_GAUGE
        }
    }
}

impl Timeline for Noelle {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // check if skill can be used
        if state.rel_time.press >= 24. {
            CharacterAction::PressSkill
        // is burst CD off and has enough energy
        } else if state.rel_time.burst >= 15. && state.energy >= 60. {
            CharacterAction::Burst
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.654 {
            // 4 attacks in 2.616 seconds
            data.na_idx.to_na(4, state.na_carryover(0.654))
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        match event {
            CharacterAction::Na4(_) => state.reduce_skill += 1.,
            _ => (),
        }
    }
}

impl CharacterAttack for Noelle {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.burst_time = time;
        atk_queue.add_burst(120.96 + 167.76, &GEO_GAUGE1A, time, event, data, state);
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(216.0, &GEO_GAUGE1A, time, event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(156.4, self.infusion(time), time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(145.01, self.infusion(time), time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(170.51, self.infusion(time), time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(224.23, self.infusion(time), time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_burst() {
            self.burst_time = action_state.current_time;
        }
        if attack.idx == data.idx && attack.time - self.burst_time <= 15. {
            // state.flat_atk += 0.72 * state.DEF();
            state.flat_atk += 1.35 * state.DEF();
        }
    }

    fn reset_modify(&mut self) -> () {
        self.burst_time = -99.;
    }
}

// Reduces Starfell Sword's CD by 2s.

// The final hit of a Normal Attack combo triggers a collapse, dealing 60% of
// ATK as AoE Geo DMG.
#[derive(Debug)]
pub struct TravelerGeo {}

impl TravelerGeo {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Traveler (Geo)").vision(Geo).weapon(Sword).version(1.0)
            .base_hp(10875.0).base_atk(212.0).base_def(683.0)
            .atk(24.0)
            .energy_cost(60.)
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Timeline for TravelerGeo {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // check if skill can be used
        if state.rel_time.press >= 6. {
            CharacterAction::PressSkill
        // is burst CD off and has enough energy
        } else if state.rel_time.burst >= 15. && state.energy >= 60. {
            CharacterAction::Burst
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.51 {
            // 5 attacks in 2.55 seconds
            data.na_idx.to_na(5, state.na_carryover(0.51))
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        match event {
            CharacterAction::PressSkill => field_energy.push_p(Particle::new(data.character.vision, 3.5)),
            _ => (),
        }
    }
}

impl CharacterAttack for TravelerGeo {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        for i in 0..4 {
            atk_queue.add_burst(266.4, &GEO_GAUGE2B, time + 0.25 * i as f32, event, data, state);
        }
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(446.4, &GEO_GAUGE1A, time, event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(87.89, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(85.85, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(104.72, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(115.26, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na5(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(139.91, &PHYSICAL_GAUGE, time, event, data, state);
        atk_queue.add_na(60., &GEO_GAUGE1A, time, event, data, state);
    }

    // fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
    // }

    // fn reset_modify(&mut self) -> () {
    // }
}
