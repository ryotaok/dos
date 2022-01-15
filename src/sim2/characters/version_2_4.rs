use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, CharacterAttack, AtkQueue};
use crate::sim2::types::{CharacterAction, DamageType, Vision, FieldCharacterIndex, WeaponType, FieldEnergy, Particle, VecFieldEnergy, ToNaAction, PeriodicStack};
use crate::sim2::element::{ElementalGauge, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::sim2::record::{CharacterRecord, CharacterData, Enemy};

use WeaponType::*;
use Vision::*;

// An active character within the field created by Divine Maiden's Deliverance
// gains 15% Cryo DMG Bonus.

// After Shenhe uses Spring Spirit Summoning, she will grant all nearby party
// members the following effects:
// * Press: Elemental Skill and Elemental Burst DMG increased by 15% for 10s.
// * Hold: Normal, Charged, and Plunging Attack DMG increased by 15% for 15s.
#[derive(Debug)]
pub struct Shenhe {
    skill_time: f32,
    burst_time: f32,
}

impl Shenhe {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Shenhe").vision(Cryo).weapon(Polearm).version(2.4)
            .base_hp(12993.).base_atk(304.).base_def(830.)
            .atk(28.8)
            .energy_cost(80.)
    }

    pub fn new() -> Self {
        Self {
            skill_time: -99.,
            burst_time: -99.,
        }
    }
}

impl Timeline for Shenhe {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // is burst CD off and has enough energy
        if state.rel_time.burst >= 20. && state.energy >= 80. {
            CharacterAction::Burst
        // check if skill can be used
        } else if state.rel_time.press >= 10. {
            CharacterAction::PressSkill
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.4234 {
            // 5 attacks in 2.117 seconds
            data.na_idx.to_na(5, state.na_carryover(0.4234))
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

impl CharacterAttack for Shenhe {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_burst(197.52, &CRYO_GAUGE1A, time, event, data, state);
        for i in 1..7 {
            atk_queue.add_burst(59.62, &CRYO_GAUGE1A, time + (i*2) as f32, event, data, state);
            atk_queue.add_burst(59.62, &CRYO_GAUGE1A, time + (i*2) as f32, event, data, state);
        }
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(250.56, &CRYO_GAUGE1A, time, event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(85.51, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(79.56, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(105.4, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(52.02, &PHYSICAL_GAUGE, time, event, data, state);
        atk_queue.add_na(52.02, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na5(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(129.71, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_burst() {
            self.burst_time = action_state.current_time;
        }
        if action_state.did_skill() {
            self.skill_time = action_state.current_time;
        }
        if attack.time - self.burst_time <= 12. {
            state.cryo_dmg += 15.;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.skill_time = -99.;
        self.burst_time = -99.;
    }
}

// Using Opening Flourish at the precise moment when Yun Jin is attacked will
// unleash its Level 2 Charged (Hold) form.

// The Normal Attack DMG Bonus granted by Flying Cloud Flag Formation is further
// increased by 2.5%/5%/7.5%/11.5% of Yun Jin's DEF when the party contains
// characters of 1/2/3/4 Elemental Types, respectively.
#[derive(Debug)]
pub struct YunJin {
    burst_time: f32,
    burst_stack: u8,
    def: f32,
}

impl YunJin {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Yun Jin").vision(Geo).weapon(Polearm).version(2.4)
            .base_hp(10657.).base_atk(191.).base_def(734.)
            .er(26.7)
            .energy_cost(60.)
    }

    pub fn new() -> Self {
        Self {
            burst_time: -99.,
            burst_stack: 0,
            def: 0.,
        }
    }
}

impl Timeline for YunJin {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        if state.rel_time.press >= 9. {
            CharacterAction::PressSkill
        // is burst CD off and has enough energy
        } else if state.rel_time.burst >= 15. && state.energy >= 60. {
            CharacterAction::Burst
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.4234 {
            // 5 attacks in 2.117 seconds
            data.na_idx.to_na(5, state.na_carryover(0.4234))
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

impl CharacterAttack for YunJin {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_burst(439.2, &GEO_GAUGE2B, time, event, data, state);
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        // atk_queue.add_skill(268.42, &GEO_GAUGE2B, time, event, data, state);
        // A1
        atk_queue.add_skill(708.32, &GEO_GAUGE2B, time, event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(80.07, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(79.56, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(45.39, &PHYSICAL_GAUGE, time, event, data, state);
        atk_queue.add_na(54.4, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(47.43, &PHYSICAL_GAUGE, time, event, data, state);
        atk_queue.add_na(52.93, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na5(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(133.11, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_burst() {
            self.burst_time = action_state.current_time;
            self.burst_stack = 30;
            self.def = state.DEF();
        }
        if attack.kind == DamageType::Na && attack.time - self.burst_time <= 15. && self.burst_stack > 0 {
            self.burst_stack -= 1;
            state.flat_dmg += 0.5789 * self.def * 1.115 /* A4 */;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.burst_time = -99.;
        self.burst_stack = 0;
        self.def = 0.;
    }
}
