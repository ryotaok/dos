use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, CharacterAttack, AtkQueue};
use crate::sim2::types::{CharacterAction, DamageType, Vision, FieldCharacterIndex, WeaponType, FieldEnergy, Particle, VecFieldEnergy, ToNaAction};
use crate::sim2::element::{ElementalGauge, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::sim2::record::{CharacterRecord, CharacterData, Enemy};

use WeaponType::*;
use Vision::*;

// Other nearby party members can decrease the CD of Yae Miko's Yakan Evocation:
// Sesshou Sakura:
// * Hitting opponents with Elemental Skill DMG decreases it by 1s and can occur
//   once every 1.8s.
// * Hitting opponents with Elemental Burst DMG decreases it by 1s and can occur
//   once every 1.8s.

// Every point of Elemental Mastery Yae Miko possesses will increase Sesshou
// Sakura DMG by 0.15%.
#[derive(Debug)]
pub struct YaeMiko {
    constellation: u8,
    charge: u8,
}

impl YaeMiko {
    pub fn record(constellation: u8) -> CharacterRecord {
        let (name, energy_cost) = match constellation {
            0 => ("Yae Miko 0112", 90.),
            1 => ("Yae Miko C1", 66.),
            2 => ("Yae Miko C2", 66.),
            3 => ("Yae Miko C3", 66.),
            4 => ("Yae Miko C4", 66.),
            5 => ("Yae Miko C5", 66.),
            6 => ("Yae Miko C6", 66.),
            _ => unreachable!(),
        };
        CharacterRecord::default()
            .name(name).vision(Electro).weapon(Catalyst).version(2.5)
            .base_hp(11284.).base_atk(264.).base_def(682.)
            .cr(24.2)
            .energy_cost(energy_cost)
    }

    pub fn new(constellation: u8) -> Self {
        Self {
            constellation,
            charge: 3,
        }
    }
}

impl Timeline for YaeMiko {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        let energy_cost = if self.constellation >= 1 {
            66.
        } else {
            90.
        };
        // check if skill can be used
        if self.charge > 0 {
            if state.current_time == 0. {
                CharacterAction::Na1(0.)
            } else {
                self.charge -= 1;
                CharacterAction::PressSkill
            }
        // } else if state.rel_time.press >= 9. {
        } else if state.rel_time.press >= 4. {
            CharacterAction::PressSkill
        // is burst CD off and has enough energy
        } else if state.rel_time.burst >= 22. && state.energy >= energy_cost {
            CharacterAction::Burst
        // check if normal attacks can be used (both animations are ended)
        // } else if state.rel_time.na >= 3. {
        //     // 3 attacks in 1.5 seconds
        //     data.na_idx.to_na(3, state.na_carryover(3.))
        } else if state.rel_time.na >= 0.5 {
            // 3 attacks in 1.5 seconds
            data.na_idx.to_na(3, state.na_carryover(0.5))
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        match event {
            CharacterAction::PressSkill => field_energy.push_p(Particle::new(data.character.vision, 1.6666)),
            _ => (),
        }
    }

    fn reset_timeline(&mut self) -> () {
        self.charge = 3;
    }
}

impl CharacterAttack for YaeMiko {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_burst(468., &ELECTRO_GAUGE1A, time, event, data, state);
        atk_queue.add_burst(528.77, &ELECTRO_GAUGE1A, time, event, data, state);
        atk_queue.add_burst(528.77, &ELECTRO_GAUGE1A, time, event, data, state);
        atk_queue.add_burst(528.77, &ELECTRO_GAUGE1A, time, event, data, state);
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        let m = match self.constellation {
            2 => 213.3,
            3 | 4 | 5 | 6 => 251.81,
            _ => 170.64,
        };
        for i in 0..5 {
            atk_queue.add_skill(m, &ELECTRO_GAUGE1A, time * (i*3) as f32, event, data, state);
        }
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(71.39, &ELECTRO_GAUGE1A, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(69.33, &ELECTRO_GAUGE1A, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(102.4, &ELECTRO_GAUGE1A, time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx && attack.kind == DamageType::Skill {
            state.skill_dmg += 0.15 * state.em;
            // state.skill_dmg += 20.;
        }
    }

    fn reset_modify(&mut self) -> () {
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     use crate::sim2::simulate;
//     use crate::sim2::simulate::History;
//     use crate::sim2::testutil;
//     use crate::sim2::testutil::{NoopTimeline};
//     use crate::sim2::types::CharacterAction;
//     use crate::sim2::attack::{DamageResultUtil};
//     use crate::sim2::record::{TimelineMember, WeaponRecord, Artifact};

//     #[test]
//     fn miko_1() {
//         let mut history = History::<1>::new(12., 0.2);
//         let mut character = YaeMiko::new();
//         let mut weapon = NoopTimeline {};
//         let mut artifact = NoopTimeline {};
//         let mut states = [ActionState::new(); 1];
//         let mut members = [TimelineMember {
//             character: &mut character,
//             weapon: &mut weapon,
//             artifact: &mut artifact,
//         }; 1];
//         let cr = YaeMiko::record();
//         let wr = WeaponRecord::default();
//         let ar = Artifact::default();
//         let mut data = [CharacterData::new(0, &cr, &wr, &ar); 1];

//         states[0].energy += 90.0;
//         simulate::decide_action(&mut history, &mut members, &mut states, &mut data);
//         use CharacterAction::*;
//         println!("{:?}", history.action);
//         assert!(false);
//         // assert_eq!(history.action, vec![
// // [[PressSkill], [PressSkill], [PressSkill], [Burst], [Na1(0.0)], [StandStill], [StandStill], [Na2(0.100000024)], [StandStill], [Na3(0.0)], [StandStill], [StandStill], [Na1(0.100000024)], [StandStill], [Na2(0.0)], [StandStill], [StandStill], [Na3(0.100000024)], [StandStill], [Na1(0.0)], [StandStill], [StandStill], [Na2(0.100000024)], [StandStill], [Na3(0.0)], [StandStill], [StandStill], [Na1(0.100000024)], [StandStill], [Na2(0.0)], [StandStill], [StandStill], [Na3(0.100000024)], [StandStill], [Na1(0.0)], [StandStill], [StandStill], [Na2(0.100000024)], [StandStill], [Na3(0.0)], [StandStill], [StandStill], [Na1(0.100000024)], [StandStill], [Na2(0.0)], [StandStill], [StandStill], [Na3(0.100000024)], [PressSkill], [Na1(0.0)], [StandStill], [StandStill], [Na2(0.100000024)], [StandStill], [Na3(0.0)], [StandStill], [StandStill], [Na1(0.100000024)], [StandStill], [Na2(0.0)], [StandStill]]
//         // );
//     }
// }
