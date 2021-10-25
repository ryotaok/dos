use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, CharacterAttack, AtkQueue};
use crate::sim2::types::{CharacterAction, DamageType, Vision, FieldCharacterIndex, WeaponType, FieldEnergy, Particle, VecFieldEnergy, ToNaAction};
use crate::sim2::element::{ElementalGauge, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, GEO_GAUGE4C, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::sim2::record::{CharacterRecord, CharacterData, Enemy};

use WeaponType::*;
use Vision::*;

// version 1.1

// Extends Riptide duration by 8s.

// When Tartaglia is in Foul Legacy: Raging Tide's Melee stance, on dealing a
// CRIT hit, Normal and Charged Attacks apply the Riptide status effects to
// opponents.
#[derive(Debug)]
pub struct Tartaglia {
    riptide: f32,
}

impl Tartaglia {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Tartaglia").vision(Hydro).weapon(Bow).version(1.1)
            .base_hp(13103.0).base_atk(301.0).base_def(815.0)
            .hydro_dmg(28.8)
            .energy_cost(60.)
    }

    pub fn new() -> Self {
        Self {
            riptide: -99.,
        }
    }

    fn riptide_attack(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        // riptide deals damage if on field
        if data.idx.0 == 0 && time - self.riptide >= 1.5 {
            atk_queue.add_skill(119.0, &HYDRO_GAUGE1A, time, event, data, state);
            self.riptide = time;
        }
    }
}

impl Timeline for Tartaglia {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // is burst CD off and has enough energy
        if state.rel_time.burst >= 15. && state.energy >= 60. {
            CharacterAction::Burst
        // check if skill can be used
        } else if state.rel_time.press >= 30. {
            CharacterAction::PressSkill
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.4025 {
            // 6 attacks in 2.415 seconds
            data.na_idx.to_na(6, state.na_carryover(0.4025))
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        match event {
            CharacterAction::PressSkill => field_energy.push_p(Particle::new(data.character.vision, 10.)),
            _ => (),
        }
    }
}

impl CharacterAttack for Tartaglia {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_burst(835.2, &HYDRO_GAUGE2B, time, event, data, state);
        atk_queue.add_burst(216.0, &HYDRO_GAUGE2B, time + 1., event, data, state);
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(122.4, &HYDRO_GAUGE2B, time, event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.riptide_attack(time, event, data, atk_queue, state, enemy);
        atk_queue.add_na(76.84, &HYDRO_GAUGE1A, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.riptide_attack(time, event, data, atk_queue, state, enemy);
        atk_queue.add_na(82.28, &HYDRO_GAUGE1A, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.riptide_attack(time, event, data, atk_queue, state, enemy);
        atk_queue.add_na(111.35, &HYDRO_GAUGE1A, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.riptide_attack(time, event, data, atk_queue, state, enemy);
        atk_queue.add_na(118.49, &HYDRO_GAUGE1A, time, event, data, state);
    }

    fn na5(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.riptide_attack(time, event, data, atk_queue, state, enemy);
        atk_queue.add_na(109.31, &HYDRO_GAUGE1A, time, event, data, state);
    }

    fn na6(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.riptide_attack(time, event, data, atk_queue, state, enemy);
        atk_queue.add_na(70.04, &HYDRO_GAUGE1A, time, event, data, state);
        atk_queue.add_na(74.46, &HYDRO_GAUGE1A, time + 0.1111, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        state.na_talent += 5.;
    }

    fn reset_modify(&mut self) -> () {
        self.riptide = -99.;
    }
}

// Characters shielded by Icy Paws have their Movement SPD increased by 10% and
// their Stamina Consumption decreased by 10%.

// Opponents who enter the AoE of Signature Mix have 10% decreased ATK for 15s.
#[derive(Debug)]
pub struct Diona {}

impl Diona {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Diona").vision(Cryo).weapon(Bow).version(1.1)
            .base_hp(9570.0).base_atk(212.0).base_def(601.0)
            .cryo_dmg(24.0)
            .energy_cost(80.)
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Timeline for Diona {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // is burst CD off and has enough energy
        if state.rel_time.burst >= 20. && state.energy >= 80. {
            CharacterAction::Burst
        // check if skill can be used
        } else if state.rel_time.hold >= 15. {
            CharacterAction::HoldSkill
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.4466 {
            // 5 attacks in 2.333 seconds
            data.na_idx.to_na(5, state.na_carryover(0.4466))
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        match event {
            CharacterAction::HoldSkill => field_energy.push_p(Particle::new(data.character.vision, 4.5)),
            _ => (),
        }
    }
}

impl CharacterAttack for Diona {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_burst(144.0, &CRYO_GAUGE1A, time, event, data, state);
        for i in 1..7 {
            atk_queue.add_burst(94.75, &CRYO_GAUGE1A, time + (2 * i) as f32, event, data, state);
        }
    }

    fn hold(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        for i in 0..5 {
            atk_queue.add_skill(75.46, &CRYO_GAUGE1A, time, event, data, state);
        }
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(71.4, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(66.3, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(90.1, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(85.0, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na5(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(106.25, &PHYSICAL_GAUGE, time, event, data, state);
    }

    // fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
    // }

    // fn reset_modify(&mut self) -> () {
    // }
}

// When the Jade Shield takes DMG, it will Fortify:
// - Fortified characters have 5% increased Shield Strength. Can stack up to 5
//   times, and lasts until the Jade Shield disappears.

// Zhongli deals bonus DMG based on his Max HP:
// - Normal Attack, Charged Attack, and Plunging Attack DMG is increased by
//   1.39% of Max HP.
// - Dominus Lapidis Stone Stele, resonance, and hold DMG is increased by 1.9% of Max HP.
// - Planet Befall deals additional DMG equal to 33% of Zhongli's Max HP.
#[derive(Debug)]
pub struct Zhongli {}

impl Zhongli {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Zhongli").vision(Geo).weapon(Polearm).version(1.1)
            .base_hp(14695.0).base_atk(251.0).base_def(738.0)
            .geo_dmg(28.8)
            .energy_cost(40.)
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Timeline for Zhongli {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // is burst CD off and has enough energy
        if state.rel_time.burst >= 12. && state.energy >= 40. {
            CharacterAction::Burst
        // check if skill can be used
        } else if state.rel_time.press >= 10. {
            CharacterAction::PressSkill
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.4875 {
            // 6 attacks in 2.925 seconds
            data.na_idx.to_na(6, state.na_carryover(0.4875))
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        match event {
            CharacterAction::PressSkill => field_energy.push_p(Particle::new(data.character.vision, 2.5)),
            _ => (),
        }
    }
}

impl CharacterAttack for Zhongli {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_burst(899.72, &GEO_GAUGE4C, time, event, data, state);
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(28.8, &GEO_GAUGE2B, time, event, data, state);
        for i in 0..5 {
            atk_queue.add_skill(57.6, &GEO_GAUGE1A, time + (2 * i) as f32, event, data, state);
        }
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(60.82, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(61.58, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(76.26, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(84.88, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na5(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        for i in 0..4 {
            atk_queue.add_na(21.25, &PHYSICAL_GAUGE, time, event, data, state);
        }
    }

    fn na6(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(107.73, &PHYSICAL_GAUGE, time, event, data, state);
    }

    // fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
    // }

    // fn reset_modify(&mut self) -> () {
    // }
}

// Decreases the number of opponents Sweeping Fervor must hit to trigger each
// level of shielding.
// - Shield Level 2: Lead-In requirement reduced to 1 opponent hit.
// - Shield Level 3: Rave requirement reduced to 2 opponents hit or more.

// Characters shielded by Sweeping Fervor deal 15% increased Physical DMG.
#[derive(Debug)]
pub struct Xinyan {
    skill_time: f32,
}

impl Xinyan {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Xinyan").vision(Pyro).weapon(Claymore).version(1.1)
            .base_hp(11201.0).base_atk(249.0).base_def(799.0)
            .atk(24.0)
            .energy_cost(60.)
    }

    pub fn new() -> Self {
        Self {
            skill_time: -99.,
        }
    }
}

impl Timeline for Xinyan {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // is burst CD off and has enough energy
        if state.rel_time.burst >= 15. && state.energy >= 60. {
            CharacterAction::Burst
        // check if skill can be used
        } else if state.rel_time.press >= 18. {
            CharacterAction::PressSkill
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.7 {
            // 4 attacks in 2.8 seconds
            data.na_idx.to_na(4, state.na_carryover(0.7))
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

impl CharacterAttack for Xinyan {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_burst(613.44, &PHYSICAL_GAUGE, time, event, data, state);
        for i in 0..5 {
            atk_queue.add_burst(72.0, &PYRO_GAUGE1A, time + 0.25 * i as f32, event, data, state);
        }
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(305.28, &PYRO_GAUGE1A, time, event, data, state);
        for i in 0..6 {
            atk_queue.add_skill(60.48, &PYRO_GAUGE1A, time + (2 * i) as f32, event, data, state);
        }
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(151.3, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(146.2, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(188.7, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(228.99, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_skill() {
            self.skill_time = action_state.current_time;
        }
        if attack.time - self.skill_time <= 12. {
            state.physical_dmg += 15.;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.skill_time = -99.;
    }
}
