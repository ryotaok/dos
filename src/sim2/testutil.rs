use crate::sim2::element::PHYSICAL_GAUGE;
use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline, ActionColumn};
use crate::sim2::attack::{Attack, CharacterAttack};
use crate::sim2::types::{CharacterAction, DamageType, Vision, FieldCharacterIndex, FieldEnergy, VecFieldEnergy, Particle, ToNaAction};
use crate::sim2::record::{TimelineMember, FieldMember, CharacterData, Enemy, CharacterRecord, WeaponRecord, Artifact};
use crate::sim2::simulate::History;

use Vision::*;
use CharacterAction::*;

pub fn chance() -> f32 {
    // rand::random::<f32>()
    0.0
}

#[derive(Debug)]
pub struct Sim2TestCharacter {
    infusion: bool,
    use_skill: bool,
}

impl Sim2TestCharacter {
    pub fn new() -> Self {
        Self {
            infusion: false,
            use_skill: true,
        }
    }

    pub fn infusion(mut self, infusion: bool) -> Self {
        self.infusion = infusion;
        self
    }

    pub fn use_skill(mut self, use_skill: bool) -> Self {
        self.use_skill = use_skill;
        self
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
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // is burst CD off and has enough energy
        if state.rel_time.burst >= 12.0 && state.energy >= 40.0 {
            CharacterAction::Burst
        // check if skill can be used
        } else if self.use_skill && state.rel_time.press >= 6.0 {
            CharacterAction::PressSkill
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.4 {
            data.na_idx.to_na(4, state.carryover(0.4))
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        match event {
            CharacterAction::PressSkill => field_energy.push_p(Particle::new(data.character.vision, 2.0)),
            _ => (),
        }
    }
}

impl CharacterAttack for Sim2TestCharacter {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.push(Attack {
            kind: DamageType::Burst,
            multiplier: 300.0,
            element: data.character.vision.to_gauge(),
            aura_application: state.apply_aura(time, event),
            time,
            idx: data.idx,
        });
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.push(Attack {
            kind: DamageType::Skill,
            multiplier: 200.0,
            element: data.character.vision.to_gauge(),
            aura_application: state.apply_aura(time, event),
            time,
            idx: data.idx,
        });
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.push(Attack {
            kind: DamageType::Na,
            multiplier: 100.0,
            element: if state.infusion || self.infusion {
                data.character.vision.to_gauge()
            } else {
                &PHYSICAL_GAUGE
            },
            aura_application: state.apply_aura(time, event),
            time,
            idx: data.idx,
        });
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.na1(time, event, data, atk_queue, state, enemy);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.na1(time, event, data, atk_queue, state, enemy);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.na1(time, event, data, atk_queue, state, enemy);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {}
}

#[derive(Debug)]
pub struct NoopTimeline {}

impl Timeline for NoopTimeline {}

pub fn history_7at02() -> History<1> {
    History::<1> {
        end_time: 7.0,
        unit_time: 0.2,
        action: vec![
        [Burst],
        [PressSkill],
        [Na1(0.)], [StandStill],
        [Na2(0.)], [StandStill],
        [Na3(0.)], [StandStill],
        [Na4(0.)], [StandStill],
        [Na1(0.)], [StandStill],
        [Na2(0.)], [StandStill],
        [Na3(0.)], [StandStill],
        [Na4(0.)], [StandStill],
        [Na1(0.)], [StandStill],
        [Na2(0.)], [StandStill],
        [Na3(0.)], [StandStill],
        [Na4(0.)], [StandStill],
        [Na1(0.)], [StandStill],
        [Na2(0.)], [StandStill],
        [Na3(0.)], [StandStill],
        [PressSkill],
        [Na1(0.)], [StandStill],
        [Na2(0.)]
        ],
        state: vec![
[ActionState { current_time: 0.0, abs_time: ActionColumn { burst: 0.0, press: -1.0, hold: -1.0, na: -1.0, ca: -1.0 }, rel_time: ActionColumn { burst: 0.0, press: 100.0, hold: 100.0, na: 100.0, ca: 100.0 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 0.0, er: 0.0 }], 
[ActionState { current_time: 0.2, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: -1.0, ca: -1.0 }, rel_time: ActionColumn { burst: 0.2, press: 0.0, hold: 100.2, na: 100.0, ca: 100.2 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 0.0, er: 0.0 }], 
[ActionState { current_time: 0.4, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 0.4, ca: -1.0 }, rel_time: ActionColumn { burst: 0.4, press: 0.2, hold: 100.399994, na: 0.0, ca: 100.399994 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 0.6, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 0.4, ca: -1.0 }, rel_time: ActionColumn { burst: 0.6, press: 0.4, hold: 100.59999, na: 0.2, ca: 100.59999 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 0.8, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 0.8, ca: -1.0 }, rel_time: ActionColumn { burst: 0.8, press: 0.6, hold: 100.79999, na: 0.0, ca: 100.79999 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 1.0, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 0.8, ca: -1.0 }, rel_time: ActionColumn { burst: 1.0, press: 0.8, hold: 100.999985, na: 0.2, ca: 100.999985 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 1.2, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 1.2, ca: -1.0 }, rel_time: ActionColumn { burst: 1.2, press: 1.0, hold: 101.19998, na: 0.0, ca: 101.19998 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 1.4000001, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 1.2, ca: -1.0 }, rel_time: ActionColumn { burst: 1.4000001, press: 1.2, hold: 101.39998, na: 0.2, ca: 101.39998 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 1.6000001, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 1.6000001, ca: -1.0 }, rel_time: ActionColumn { burst: 1.6000001, press: 1.4000001, hold: 101.599976, na: 0.0, ca: 101.599976 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 1.8000002, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 1.6000001, ca: -1.0 }, rel_time: ActionColumn { burst: 1.8000002, press: 1.6000001, hold: 101.79997, na: 0.2, ca: 101.79997 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 2.0000002, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 2.0000002, ca: -1.0 }, rel_time: ActionColumn { burst: 2.0000002, press: 1.8000002, hold: 101.99997, na: 0.0, ca: 101.99997 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 2.2000003, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 2.0000002, ca: -1.0 }, rel_time: ActionColumn { burst: 2.2000003, press: 2.0000002, hold: 102.19997, na: 0.2, ca: 102.19997 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 2.4000003, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 2.4000003, ca: -1.0 }, rel_time: ActionColumn { burst: 2.4000003, press: 2.2000003, hold: 102.39996, na: 0.0, ca: 102.39996 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 2.6000004, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 2.4000003, ca: -1.0 }, rel_time: ActionColumn { burst: 2.6000004, press: 2.4000003, hold: 102.59996, na: 0.2, ca: 102.59996 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 2.8000004, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 2.8000004, ca: -1.0 }, rel_time: ActionColumn { burst: 2.8000004, press: 2.6000004, hold: 102.79996, na: 0.0, ca: 102.79996 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 3.0000005, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 2.8000004, ca: -1.0 }, rel_time: ActionColumn { burst: 3.0000005, press: 2.8000004, hold: 102.999954, na: 0.2, ca: 102.999954 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 3.2000005, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 3.2000005, ca: -1.0 }, rel_time: ActionColumn { burst: 3.2000005, press: 3.0000005, hold: 103.19995, na: 0.0, ca: 103.19995 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 3.4000006, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 3.2000005, ca: -1.0 }, rel_time: ActionColumn { burst: 3.4000006, press: 3.2000005, hold: 103.39995, na: 0.2, ca: 103.39995 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 3.6000006, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 3.6000006, ca: -1.0 }, rel_time: ActionColumn { burst: 3.6000006, press: 3.4000006, hold: 103.599945, na: 0.0, ca: 103.599945 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 3.8000007, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 3.6000006, ca: -1.0 }, rel_time: ActionColumn { burst: 3.8000007, press: 3.6000006, hold: 103.79994, na: 0.2, ca: 103.79994 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 4.0000005, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 4.0000005, ca: -1.0 }, rel_time: ActionColumn { burst: 4.0000005, press: 3.8000007, hold: 103.99994, na: 0.0, ca: 103.99994 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 4.2000003, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 4.0000005, ca: -1.0 }, rel_time: ActionColumn { burst: 4.2000003, press: 4.0000005, hold: 104.199936, na: 0.2, ca: 104.199936 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 4.4, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 4.4, ca: -1.0 }, rel_time: ActionColumn { burst: 4.4, press: 4.2000003, hold: 104.39993, na: 0.0, ca: 104.39993 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 4.6, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 4.4, ca: -1.0 }, rel_time: ActionColumn { burst: 4.6, press: 4.4, hold: 104.59993, na: 0.2, ca: 104.59993 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 4.7999997, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 4.7999997, ca: -1.0 }, rel_time: ActionColumn { burst: 4.7999997, press: 4.6, hold: 104.79993, na: 0.0, ca: 104.79993 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 4.9999995, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 4.7999997, ca: -1.0 }, rel_time: ActionColumn { burst: 4.9999995, press: 4.7999997, hold: 104.99992, na: 0.2, ca: 104.99992 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 5.1999993, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 5.1999993, ca: -1.0 }, rel_time: ActionColumn { burst: 5.1999993, press: 4.9999995, hold: 105.19992, na: 0.0, ca: 105.19992 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 5.399999, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 5.1999993, ca: -1.0 }, rel_time: ActionColumn { burst: 5.399999, press: 5.1999993, hold: 105.39992, na: 0.2, ca: 105.39992 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 5.599999, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 5.599999, ca: -1.0 }, rel_time: ActionColumn { burst: 5.599999, press: 5.399999, hold: 105.599915, na: 0.0, ca: 105.599915 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 5.7999988, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 5.599999, ca: -1.0 }, rel_time: ActionColumn { burst: 5.7999988, press: 5.599999, hold: 105.79991, na: 0.2, ca: 105.79991 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 5.9999986, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 5.9999986, ca: -1.0 }, rel_time: ActionColumn { burst: 5.9999986, press: 5.7999988, hold: 105.99991, na: 0.0, ca: 105.99991 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 6.1999984, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 5.9999986, ca: -1.0 }, rel_time: ActionColumn { burst: 6.1999984, press: 5.9999986, hold: 106.199905, na: 0.2, ca: 106.199905 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 6.399998, abs_time: ActionColumn { burst: 0.0, press: 6.399998, hold: -1.0, na: 5.9999986, ca: -1.0 }, rel_time: ActionColumn { burst: 6.399998, press: 0.0, hold: 106.3999, na: 100.0, ca: 106.3999 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
[ActionState { current_time: 6.599998, abs_time: ActionColumn { burst: 0.0, press: 6.399998, hold: -1.0, na: 6.599998, ca: -1.0 }, rel_time: ActionColumn { burst: 6.599998, press: 0.2, hold: 106.5999, na: 0.0, ca: 106.5999 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 12.0, er: 0.0 }], 
[ActionState { current_time: 6.799998, abs_time: ActionColumn { burst: 0.0, press: 6.399998, hold: -1.0, na: 6.599998, ca: -1.0 }, rel_time: ActionColumn { burst: 6.799998, press: 0.4, hold: 106.7999, na: 0.2, ca: 106.7999 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 12.0, er: 0.0 }], 
[ActionState { current_time: 6.9999976, abs_time: ActionColumn { burst: 0.0, press: 6.399998, hold: -1.0, na: 6.9999976, ca: -1.0 }, rel_time: ActionColumn { burst: 6.9999976, press: 0.6, hold: 106.99989, na: 0.0, ca: 106.99989 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 12.0, er: 0.0 }]
        ]
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::sim2::simulate;
//     use crate::sim2::types::Vision;
//     use crate::sim2::record::{WeaponRecord, Artifact};
//     #[test]
//     fn debug() {
//         let mut history = History::<1>::new(7., 0.2);
//         let mut character = Sim2TestCharacter::new();
//         let mut weapon = NoopTimeline {};
//         let mut artifact = NoopTimeline {};
//         let mut states = [ActionState::new(); 1];
//         let mut members = [TimelineMember {
//             character: &mut character,
//             weapon: &mut weapon,
//             artifact: &mut artifact,
//         }; 1];
//         let cr = Sim2TestCharacter::record(Vision::Pyro);
//         let wr = WeaponRecord::default();
//         let ar = Artifact::default();
//         let mut data = [CharacterData::new(0, &cr, &wr, &ar); 1];
//         states[0].energy += 40.0;
//         simulate::decide_action(&mut history, &mut members, &mut states, &mut data);
//         println!("{:?}", history.action);
//         println!("{:?}", history.state);
//         assert!(false);
//     }
// }

pub fn history_2at02() -> History<2> {
    History::<2> {
        end_time: 2.0,
        unit_time: 0.2,
        action: vec![
        [Burst, Burst],
        [PressSkill, PressSkill],
        [Na1(0.), Na1(0.)],
        [StandStill, StandStill],
        [Na2(0.), Na2(0.)],
        [StandStill, StandStill],
        [Na3(0.), Na3(0.)],
        [StandStill, StandStill],
        [Na4(0.), Na4(0.)],
        [StandStill, StandStill]],
        state: vec![
[ActionState { current_time: 0.0, abs_time: ActionColumn { burst: 0.0, press: -1.0, hold: -1.0, na: -1.0, ca: -1.0 }, rel_time: ActionColumn { burst: 0.0, press: 100.0, hold: 100.0, na: 100.0, ca: 100.0 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 0.0, er: 0.0 }, ActionState { current_time: 0.0, abs_time: ActionColumn { burst: 0.0, press: -1.0, hold: -1.0, na: -1.0, ca: -1.0 }, rel_time: ActionColumn { burst: 0.0, press: 100.0, hold: 100.0, na: 100.0, ca: 100.0 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 0.0, er: 0.0 }], 
[ActionState { current_time: 0.2, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: -1.0, ca: -1.0 }, rel_time: ActionColumn { burst: 0.2, press: 0.0, hold: 100.2, na: 100.0, ca: 100.2 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 0.0, er: 0.0 }, ActionState { current_time: 0.2, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: -1.0, ca: -1.0 }, rel_time: ActionColumn { burst: 0.2, press: 0.0, hold: 100.2, na: 100.0, ca: 100.2 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 0.0, er: 0.0 }], 
[ActionState { current_time: 0.4, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 0.4, ca: -1.0 }, rel_time: ActionColumn { burst: 0.4, press: 0.2, hold: 100.399994, na: 0.0, ca: 100.399994 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 12.0, er: 0.0 }, ActionState { current_time: 0.4, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 0.4, ca: -1.0 }, rel_time: ActionColumn { burst: 0.4, press: 0.2, hold: 100.399994, na: 0.0, ca: 100.399994 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 7.2, er: 0.0 }], 
[ActionState { current_time: 0.6, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 0.4, ca: -1.0 }, rel_time: ActionColumn { burst: 0.6, press: 0.4, hold: 100.59999, na: 0.2, ca: 100.59999 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 12.0, er: 0.0 }, ActionState { current_time: 0.6, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 0.4, ca: -1.0 }, rel_time: ActionColumn { burst: 0.6, press: 0.4, hold: 100.59999, na: 0.2, ca: 100.59999 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 7.2, er: 0.0 }], 
[ActionState { current_time: 0.8, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 0.8, ca: -1.0 }, rel_time: ActionColumn { burst: 0.8, press: 0.6, hold: 100.79999, na: 0.0, ca: 100.79999 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 12.0, er: 0.0 }, ActionState { current_time: 0.8, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 0.8, ca: -1.0 }, rel_time: ActionColumn { burst: 0.8, press: 0.6, hold: 100.79999, na: 0.0, ca: 100.79999 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 7.2, er: 0.0 }], 
[ActionState { current_time: 1.0, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 0.8, ca: -1.0 }, rel_time: ActionColumn { burst: 1.0, press: 0.8, hold: 100.999985, na: 0.2, ca: 100.999985 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 12.0, er: 0.0 }, ActionState { current_time: 1.0, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 0.8, ca: -1.0 }, rel_time: ActionColumn { burst: 1.0, press: 0.8, hold: 100.999985, na: 0.2, ca: 100.999985 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 7.2, er: 0.0 }], 
[ActionState { current_time: 1.2, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 1.2, ca: -1.0 }, rel_time: ActionColumn { burst: 1.2, press: 1.0, hold: 101.19998, na: 0.0, ca: 101.19998 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 12.0, er: 0.0 }, ActionState { current_time: 1.2, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 1.2, ca: -1.0 }, rel_time: ActionColumn { burst: 1.2, press: 1.0, hold: 101.19998, na: 0.0, ca: 101.19998 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 7.2, er: 0.0 }], 
[ActionState { current_time: 1.4000001, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 1.2, ca: -1.0 }, rel_time: ActionColumn { burst: 1.4000001, press: 1.2, hold: 101.39998, na: 0.2, ca: 101.39998 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 12.0, er: 0.0 }, ActionState { current_time: 1.4000001, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 1.2, ca: -1.0 }, rel_time: ActionColumn { burst: 1.4000001, press: 1.2, hold: 101.39998, na: 0.2, ca: 101.39998 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 7.2, er: 0.0 }], 
[ActionState { current_time: 1.6000001, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 1.6000001, ca: -1.0 }, rel_time: ActionColumn { burst: 1.6000001, press: 1.4000001, hold: 101.599976, na: 0.0, ca: 101.599976 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 12.0, er: 0.0 }, ActionState { current_time: 1.6000001, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 1.6000001, ca: -1.0 }, rel_time: ActionColumn { burst: 1.6000001, press: 1.4000001, hold: 101.599976, na: 0.0, ca: 101.599976 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 7.2, er: 0.0 }], 
[ActionState { current_time: 1.8000002, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 1.6000001, ca: -1.0 }, rel_time: ActionColumn { burst: 1.8000002, press: 1.6000001, hold: 101.79997, na: 0.2, ca: 101.79997 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 12.0, er: 0.0 }, ActionState { current_time: 1.8000002, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: -1.0, na: 1.6000001, ca: -1.0 }, rel_time: ActionColumn { burst: 1.8000002, press: 1.6000001, hold: 101.79997, na: 0.2, ca: 101.79997 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 7.2, er: 0.0 }], 
[ActionState { current_time: 0.0, abs_time: ActionColumn { burst: -1.0, press: -1.0, hold: -1.0, na: -1.0, ca: -1.0 }, rel_time: ActionColumn { burst: 100.0, press: 100.0, hold: 100.0, na: 100.0, ca: 100.0 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 0.0, er: 0.0 }, ActionState { current_time: 0.0, abs_time: ActionColumn { burst: -1.0, press: -1.0, hold: -1.0, na: -1.0, ca: -1.0 }, rel_time: ActionColumn { burst: 100.0, press: 100.0, hold: 100.0, na: 100.0, ca: 100.0 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 0.0, er: 0.0 }]
]
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::sim2::simulate;
//     use crate::sim2::types::Vision;
//     use crate::sim2::record::{WeaponRecord, Artifact};
//     #[test]
//     fn debug() {
//         let mut history = History::<2>::new(2., 0.2);
//         let mut character1 = Sim2TestCharacter::new();
//         let mut character2 = Sim2TestCharacter::new();
//         let mut weapon1    = NoopTimeline {};
//         let mut weapon2    = NoopTimeline {};
//         let mut artifact1  = NoopTimeline {};
//         let mut artifact2  = NoopTimeline {};
//         let mut states = [ActionState::new(); 2];
//         let mut members = [TimelineMember {
//             character: &mut character1,
//             weapon: &mut weapon1,
//             artifact: &mut artifact1,
//         }, TimelineMember {
//             character: &mut character2,
//             weapon: &mut weapon2,
//             artifact: &mut artifact2,
//         }];
//         let cr = Sim2TestCharacter::record(Vision::Pyro);
//         let wr = WeaponRecord::default();
//         let ar = Artifact::default();
//         let mut data = [CharacterData::new(0, &cr, &wr, &ar), CharacterData::new(1, &cr, &wr, &ar)];
//         states[0].energy += 40.0;
//         states[1].energy += 40.0;
//         simulate::decide_action(&mut history, &mut members, &mut states, &mut data);
//         println!("{:?}", history.action);
//         println!("{:?}", history.state);
//         assert!(false);
//     }
// }

pub fn history_12at02enrgy15() -> History<1> {
    History::<1> {
        end_time: 12.0,
        unit_time: 0.2,
        action: vec![
        [PressSkill],
        [Na1(0.)], [StandStill],
        [Na2(0.)], [StandStill],
        [Na3(0.)], [StandStill],
        [Na4(0.)], [StandStill],
        [Na1(0.)], [StandStill],
        [Na2(0.)], [StandStill],
        [Na3(0.)], [StandStill],
        [Na4(0.)], [StandStill],
        [Na1(0.)], [StandStill],
        [Na2(0.)], [StandStill],
        [Na3(0.)], [StandStill],
        [Na4(0.)], [StandStill],
        [Na1(0.)], [StandStill],
        [Na2(0.)], [StandStill],
        [Na3(0.)], [StandStill],
        [PressSkill],
        [Na1(0.)], [StandStill],
        [Na2(0.)], [StandStill],
        [Na3(0.)], [StandStill],
        [Na4(0.)], [StandStill],
        [Na1(0.)], [StandStill],
        [Na2(0.)], [StandStill],
        [Na3(0.)], [StandStill],
        [Na4(0.)], [StandStill],
        [Na1(0.)], [StandStill],
        [Na2(0.)], [StandStill],
        [Na3(0.)], [StandStill],
        [Na4(0.)], [StandStill],
        [Na1(0.)], [StandStill],
        [Na2(0.)], [StandStill],
        [Na3(0.)]],
        state: vec![
[ActionState { current_time: 0.0, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: -1.0, ca: -1.0 }, rel_time: ActionColumn { burst: 100.0, press: 0.0, hold: 100.0, na: 100.0, ca: 100.0 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 15.0, er: 0.0 }], 
[ActionState { current_time: 0.2, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 0.2, ca: -1.0 }, rel_time: ActionColumn { burst: 100.2, press: 0.2, hold: 100.2, na: 0.0, ca: 100.2 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 0.4, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 0.2, ca: -1.0 }, rel_time: ActionColumn { burst: 100.399994, press: 0.4, hold: 100.399994, na: 0.2, ca: 100.399994 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 0.6, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 0.6, ca: -1.0 }, rel_time: ActionColumn { burst: 100.59999, press: 0.6, hold: 100.59999, na: 0.0, ca: 100.59999 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 0.8, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 0.6, ca: -1.0 }, rel_time: ActionColumn { burst: 100.79999, press: 0.8, hold: 100.79999, na: 0.2, ca: 100.79999 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 1.0, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 1.0, ca: -1.0 }, rel_time: ActionColumn { burst: 100.999985, press: 1.0, hold: 100.999985, na: 0.0, ca: 100.999985 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 1.2, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 1.0, ca: -1.0 }, rel_time: ActionColumn { burst: 101.19998, press: 1.2, hold: 101.19998, na: 0.2, ca: 101.19998 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 1.4000001, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 1.4000001, ca: -1.0 }, rel_time: ActionColumn { burst: 101.39998, press: 1.4000001, hold: 101.39998, na: 0.0, ca: 101.39998 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 1.6000001, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 1.4000001, ca: -1.0 }, rel_time: ActionColumn { burst: 101.599976, press: 1.6000001, hold: 101.599976, na: 0.2, ca: 101.599976 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 1.8000002, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 1.8000002, ca: -1.0 }, rel_time: ActionColumn { burst: 101.79997, press: 1.8000002, hold: 101.79997, na: 0.0, ca: 101.79997 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 2.0000002, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 1.8000002, ca: -1.0 }, rel_time: ActionColumn { burst: 101.99997, press: 2.0000002, hold: 101.99997, na: 0.2, ca: 101.99997 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 2.2000003, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 2.2000003, ca: -1.0 }, rel_time: ActionColumn { burst: 102.19997, press: 2.2000003, hold: 102.19997, na: 0.0, ca: 102.19997 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 2.4000003, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 2.2000003, ca: -1.0 }, rel_time: ActionColumn { burst: 102.39996, press: 2.4000003, hold: 102.39996, na: 0.2, ca: 102.39996 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 2.6000004, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 2.6000004, ca: -1.0 }, rel_time: ActionColumn { burst: 102.59996, press: 2.6000004, hold: 102.59996, na: 0.0, ca: 102.59996 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 2.8000004, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 2.6000004, ca: -1.0 }, rel_time: ActionColumn { burst: 102.79996, press: 2.8000004, hold: 102.79996, na: 0.2, ca: 102.79996 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 3.0000005, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 3.0000005, ca: -1.0 }, rel_time: ActionColumn { burst: 102.999954, press: 3.0000005, hold: 102.999954, na: 0.0, ca: 102.999954 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 3.2000005, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 3.0000005, ca: -1.0 }, rel_time: ActionColumn { burst: 103.19995, press: 3.2000005, hold: 103.19995, na: 0.2, ca: 103.19995 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 3.4000006, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 3.4000006, ca: -1.0 }, rel_time: ActionColumn { burst: 103.39995, press: 3.4000006, hold: 103.39995, na: 0.0, ca: 103.39995 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 3.6000006, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 3.4000006, ca: -1.0 }, rel_time: ActionColumn { burst: 103.599945, press: 3.6000006, hold: 103.599945, na: 0.2, ca: 103.599945 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 3.8000007, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 3.8000007, ca: -1.0 }, rel_time: ActionColumn { burst: 103.79994, press: 3.8000007, hold: 103.79994, na: 0.0, ca: 103.79994 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 4.0000005, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 3.8000007, ca: -1.0 }, rel_time: ActionColumn { burst: 103.99994, press: 4.0000005, hold: 103.99994, na: 0.2, ca: 103.99994 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 4.2000003, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 4.2000003, ca: -1.0 }, rel_time: ActionColumn { burst: 104.199936, press: 4.2000003, hold: 104.199936, na: 0.0, ca: 104.199936 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 4.4, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 4.2000003, ca: -1.0 }, rel_time: ActionColumn { burst: 104.39993, press: 4.4, hold: 104.39993, na: 0.2, ca: 104.39993 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 4.6, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 4.6, ca: -1.0 }, rel_time: ActionColumn { burst: 104.59993, press: 4.6, hold: 104.59993, na: 0.0, ca: 104.59993 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 4.7999997, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 4.6, ca: -1.0 }, rel_time: ActionColumn { burst: 104.79993, press: 4.7999997, hold: 104.79993, na: 0.2, ca: 104.79993 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 4.9999995, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 4.9999995, ca: -1.0 }, rel_time: ActionColumn { burst: 104.99992, press: 4.9999995, hold: 104.99992, na: 0.0, ca: 104.99992 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 5.1999993, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 4.9999995, ca: -1.0 }, rel_time: ActionColumn { burst: 105.19992, press: 5.1999993, hold: 105.19992, na: 0.2, ca: 105.19992 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 5.399999, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 5.399999, ca: -1.0 }, rel_time: ActionColumn { burst: 105.39992, press: 5.399999, hold: 105.39992, na: 0.0, ca: 105.39992 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 5.599999, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 5.399999, ca: -1.0 }, rel_time: ActionColumn { burst: 105.599915, press: 5.599999, hold: 105.599915, na: 0.2, ca: 105.599915 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 5.7999988, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 5.7999988, ca: -1.0 }, rel_time: ActionColumn { burst: 105.79991, press: 5.7999988, hold: 105.79991, na: 0.0, ca: 105.79991 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 5.9999986, abs_time: ActionColumn { burst: -1.0, press: 0.0, hold: -1.0, na: 5.7999988, ca: -1.0 }, rel_time: ActionColumn { burst: 105.99991, press: 5.9999986, hold: 105.99991, na: 0.2, ca: 105.99991 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 6.1999984, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 5.7999988, ca: -1.0 }, rel_time: ActionColumn { burst: 106.199905, press: 0.0, hold: 106.199905, na: 100.0, ca: 106.199905 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 21.0, er: 0.0 }], 
[ActionState { current_time: 6.399998, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 6.399998, ca: -1.0 }, rel_time: ActionColumn { burst: 106.3999, press: 0.2, hold: 106.3999, na: 0.0, ca: 106.3999 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 6.599998, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 6.399998, ca: -1.0 }, rel_time: ActionColumn { burst: 106.5999, press: 0.4, hold: 106.5999, na: 0.2, ca: 106.5999 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 6.799998, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 6.799998, ca: -1.0 }, rel_time: ActionColumn { burst: 106.7999, press: 0.6, hold: 106.7999, na: 0.0, ca: 106.7999 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 6.9999976, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 6.799998, ca: -1.0 }, rel_time: ActionColumn { burst: 106.99989, press: 0.8, hold: 106.99989, na: 0.2, ca: 106.99989 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 7.1999974, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 7.1999974, ca: -1.0 }, rel_time: ActionColumn { burst: 107.19989, press: 1.0, hold: 107.19989, na: 0.0, ca: 107.19989 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 7.399997, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 7.1999974, ca: -1.0 }, rel_time: ActionColumn { burst: 107.39989, press: 1.2, hold: 107.39989, na: 0.2, ca: 107.39989 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 7.599997, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 7.599997, ca: -1.0 }, rel_time: ActionColumn { burst: 107.599884, press: 1.4000001, hold: 107.599884, na: 0.0, ca: 107.599884 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 7.799997, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 7.599997, ca: -1.0 }, rel_time: ActionColumn { burst: 107.79988, press: 1.6000001, hold: 107.79988, na: 0.2, ca: 107.79988 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 7.9999967, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 7.9999967, ca: -1.0 }, rel_time: ActionColumn { burst: 107.99988, press: 1.8000002, hold: 107.99988, na: 0.0, ca: 107.99988 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 8.199997, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 7.9999967, ca: -1.0 }, rel_time: ActionColumn { burst: 108.199875, press: 2.0000002, hold: 108.199875, na: 0.2, ca: 108.199875 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 8.399997, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 8.399997, ca: -1.0 }, rel_time: ActionColumn { burst: 108.39987, press: 2.2000003, hold: 108.39987, na: 0.0, ca: 108.39987 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 8.599997, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 8.399997, ca: -1.0 }, rel_time: ActionColumn { burst: 108.59987, press: 2.4000003, hold: 108.59987, na: 0.2, ca: 108.59987 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 8.799996, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 8.799996, ca: -1.0 }, rel_time: ActionColumn { burst: 108.799866, press: 2.6000004, hold: 108.799866, na: 0.0, ca: 108.799866 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 8.999996, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 8.799996, ca: -1.0 }, rel_time: ActionColumn { burst: 108.99986, press: 2.8000004, hold: 108.99986, na: 0.2, ca: 108.99986 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 9.199996, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 9.199996, ca: -1.0 }, rel_time: ActionColumn { burst: 109.19986, press: 3.0000005, hold: 109.19986, na: 0.0, ca: 109.19986 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 9.399996, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 9.199996, ca: -1.0 }, rel_time: ActionColumn { burst: 109.39986, press: 3.2000005, hold: 109.39986, na: 0.2, ca: 109.39986 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 9.599996, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 9.599996, ca: -1.0 }, rel_time: ActionColumn { burst: 109.59985, press: 3.4000006, hold: 109.59985, na: 0.0, ca: 109.59985 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 9.799995, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 9.599996, ca: -1.0 }, rel_time: ActionColumn { burst: 109.79985, press: 3.6000006, hold: 109.79985, na: 0.2, ca: 109.79985 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 9.999995, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 9.999995, ca: -1.0 }, rel_time: ActionColumn { burst: 109.99985, press: 3.8000007, hold: 109.99985, na: 0.0, ca: 109.99985 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 10.199995, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 9.999995, ca: -1.0 }, rel_time: ActionColumn { burst: 110.199844, press: 4.0000005, hold: 110.199844, na: 0.2, ca: 110.199844 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 10.399995, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 10.399995, ca: -1.0 }, rel_time: ActionColumn { burst: 110.39984, press: 4.2000003, hold: 110.39984, na: 0.0, ca: 110.39984 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 10.599995, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 10.399995, ca: -1.0 }, rel_time: ActionColumn { burst: 110.59984, press: 4.4, hold: 110.59984, na: 0.2, ca: 110.59984 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 10.799994, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 10.799994, ca: -1.0 }, rel_time: ActionColumn { burst: 110.799835, press: 4.6, hold: 110.799835, na: 0.0, ca: 110.799835 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 10.999994, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 10.799994, ca: -1.0 }, rel_time: ActionColumn { burst: 110.99983, press: 4.7999997, hold: 110.99983, na: 0.2, ca: 110.99983 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 11.199994, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 11.199994, ca: -1.0 }, rel_time: ActionColumn { burst: 111.19983, press: 4.9999995, hold: 111.19983, na: 0.0, ca: 111.19983 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 11.399994, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 11.199994, ca: -1.0 }, rel_time: ActionColumn { burst: 111.399826, press: 5.1999993, hold: 111.399826, na: 0.2, ca: 111.399826 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 11.599994, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 11.599994, ca: -1.0 }, rel_time: ActionColumn { burst: 111.59982, press: 5.399999, hold: 111.59982, na: 0.0, ca: 111.59982 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 11.7999935, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 11.599994, ca: -1.0 }, rel_time: ActionColumn { burst: 111.79982, press: 5.599999, hold: 111.79982, na: 0.2, ca: 111.79982 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }], 
[ActionState { current_time: 11.999993, abs_time: ActionColumn { burst: -1.0, press: 6.1999984, hold: -1.0, na: 11.999993, ca: -1.0 }, rel_time: ActionColumn { burst: 111.99982, press: 5.7999988, hold: 111.99982, na: 0.0, ca: 111.99982 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 27.0, er: 0.0 }]
]
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::sim2::simulate;
//     use crate::sim2::types::Vision;
//     use crate::sim2::record::{WeaponRecord, Artifact};
//     #[test]
//     fn debug() {
//         let mut history = History::<1>::new(12., 0.2);
//         let mut character = Sim2TestCharacter::new();
//         let mut weapon = NoopTimeline {};
//         let mut artifact = NoopTimeline {};
//         let mut states = [ActionState::new(); 1];
//         let mut members = [TimelineMember {
//             character: &mut character,
//             weapon: &mut weapon,
//             artifact: &mut artifact,
//         }; 1];
//         let cr = Sim2TestCharacter::record(Vision::Pyro);
//         let wr = WeaponRecord::default();
//         let ar = Artifact::default();
//         let mut data = [CharacterData::new(0, &cr, &wr, &ar); 1];
//         states[0].energy += 15.0;
//         simulate::decide_action(&mut history, &mut members, &mut states, &mut data);
//         println!("{:?}", history.action);
//         println!("{:?}", history.state);
//         assert!(false);
//     }
// }
