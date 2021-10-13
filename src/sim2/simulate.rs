use crate::sim2::state::State;
use crate::sim2::timeline::ActionState;
use crate::sim2::attack::Attack;
use crate::sim2::types::{CharacterAction, DamageType, Vision, FieldCharacterIndex, FieldEnergy};
use crate::sim2::record::{TimelineMember, FieldMember, CharacterData, Enemy};

// why? to reuse allocations for all characters, because if `end_time` and
// `unit_time` are fixed, all histories have the same size.
#[derive(Debug)]
pub struct History<const N: usize> {
    end_time: f32,
    unit_time: f32,
    action: Vec<[CharacterAction; N]>,
    state: Vec<[ActionState; N]>,
}

impl<const N: usize> History<N> {
    pub fn new(end_time: f32, unit_time: f32) -> Self {
        let size = 1 + (end_time / unit_time) as usize;
        Self {
            end_time,
            unit_time,
            action: Vec::with_capacity(size),
            state: vec![[ActionState::new(); N]; size],
        }
    }

    pub fn state_index(&self, time: f32) -> usize {
        (time / self.unit_time).floor() as usize
    }
}

pub fn decide_action<const N: usize>(history: &mut History<N>, members: &mut [TimelineMember; N], states: &mut [ActionState; N]) -> () {
    let mut field_energy: Vec<FieldEnergy> = Vec::new();
    let mut current_time: f32 = 0.0;
    let mut idx = 0;
    while current_time <= history.end_time {
        let mut actions = [CharacterAction::StandStill; N];
        for (i, member) in members.iter_mut().enumerate() {
            history.state[idx][i].copy(&states[i]);
            let action = member.character.decide_action(&states[i]);
            member.character.accelerate(&mut field_energy, &action, &mut states[i]);
            member.weapon.accelerate(&mut field_energy, &action, &mut states[i]);
            member.artifact.accelerate(&mut field_energy, &action, &mut states[i]);
            actions[i] = action;
        }
        for (i, member) in members.iter_mut().enumerate() {
            let mut energy: f32 = 0.0;
            for fe in field_energy.iter() {
                match fe {
                    FieldEnergy::Particle(ref p) => energy += if i == 0 {
                        // attacker is on field
                        // TODO
                        p.on_field_energy(&Vision::Pyro) * states[i].er()
                    } else {
                        p.off_field_energy(&Vision::Pyro) * states[i].er()
                    },
                    FieldEnergy::Energy(e) => energy += e,
                }
            }
            states[i].update(&actions[i], current_time, history.unit_time, energy);
        }
        history.action.push(actions);
        field_energy.clear();
        current_time += history.unit_time;
        idx += 1;
    }
}

fn calculate_damage<const N: usize>(history: &mut History<N>, members: &mut [FieldMember; N], data: &[CharacterData; N], enemy: &mut Enemy) -> f32 {
    let mut atk_queue: Vec<Attack> = Vec::new();
    let mut states = [State::default(); N];
    for i in 0..N {
        let member = &mut members[i];
        for (state, event) in history.state.iter().zip(history.action.iter()) {
            member.character.attack(state[i].current_time, &event[i], &data[i], &mut atk_queue, &mut states[i]);
            member.weapon.attack(state[i].current_time, &event[i], &mut atk_queue, &mut states[i]);
        }
    }
    atk_queue.sort_unstable_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    let mut total_dmg = 0.0;
    for attack in atk_queue.iter_mut() {
        for i in 0..N {
            // character state first (infusion first)
            let d = &data[i];
            let member = &mut members[i];
            let action_state = &history.state[history.state_index(attack.time)][i];
            let state = &mut states[attack.idx.0];
            state.init(d);
            member.character.modify(action_state, d, attack, state, enemy);
            member.weapon.modify(action_state, d, attack, state, enemy);
            member.artifact.modify(action_state, d, attack, state, enemy);
        }
        // then change enemy state (within fn elemental_reaction)
        let dmg = attack.outgoing_damage(&states[attack.idx.0], &data[attack.idx.0]);
        total_dmg += attack.incoming_damage(dmg, &states[attack.idx.0], &data[attack.idx.0], enemy);
    }
    total_dmg
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::sim2::testutil::{Sim2TestCharacter, NoopTimeline};
    use crate::sim2::element::{ElementalGauge, ElementalGaugeDecay};
    use crate::sim2::types::{Vision};
    use crate::sim2::timeline::{ActionColumn, Timeline};
    use crate::sim2::record::{WeaponRecord, Artifact};

    use Vision::*;

    fn history_7at02() -> History<1> {
        use CharacterAction::*;
        History::<1> {
            end_time: 7.0,
            unit_time: 0.2,
            action: vec![
            [Burst], [PressSkill], [Na1], [StandStill], [Na2], [StandStill], [Na3],
            [StandStill], [Na4], [StandStill], [Na1], [StandStill], [Na2], [StandStill],
            [Na3], [StandStill], [Na4], [StandStill], [Na1], [StandStill], [Na2],
            [StandStill], [Na3], [StandStill], [Na4], [StandStill], [Na1], [StandStill],
            [Na2], [StandStill], [Na3], [StandStill], [PressSkill], [Na4], [StandStill],
            [Na1] ],
            state: vec![
            [ActionState { current_time: 0.0, abs_time: ActionColumn { burst: 0.0, press: 0.0, hold: 0.0, na: 0.0, ca: 0.0 }, rel_time: ActionColumn { burst: 99.0, press: 99.0, hold: 99.0, na: 99.0, ca: 99.0 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 40.0, er: 0.0 }],
            [ActionState { current_time: 0.0, abs_time: ActionColumn { burst: 0.0, press: 0.0, hold: 0.0, na: 0.0, ca: 0.0 }, rel_time: ActionColumn { burst: 0.2, press: 99.2, hold: 99.2, na: 99.2, ca: 99.2 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 0.0, er: 0.0 }], 
            [ActionState { current_time: 0.2, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 0.0, ca: 0.0 }, rel_time: ActionColumn { burst: 0.4, press: 0.2, hold: 99.399994, na: 99.399994, ca: 99.399994 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 0.4, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 0.4, ca: 0.0 }, rel_time: ActionColumn { burst: 0.6, press: 0.4, hold: 99.59999, na: 0.2, ca: 99.59999 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 0.6, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 0.4, ca: 0.0 }, rel_time: ActionColumn { burst: 0.8, press: 0.6, hold: 99.79999, na: 0.4, ca: 99.79999 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 0.8, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 0.8, ca: 0.0 }, rel_time: ActionColumn { burst: 1.0, press: 0.8, hold: 99.999985, na: 0.2, ca: 99.999985 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 1.0, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 0.8, ca: 0.0 }, rel_time: ActionColumn { burst: 1.2, press: 1.0, hold: 100.19998, na: 0.4, ca: 100.19998 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 1.2, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 1.2, ca: 0.0 }, rel_time: ActionColumn { burst: 1.4000001, press: 1.2, hold: 100.39998, na: 0.2, ca: 100.39998 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 1.4, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 1.2, ca: 0.0 }, rel_time: ActionColumn { burst: 1.6000001, press: 1.4000001, hold: 100.599976, na: 0.4, ca: 100.599976 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 1.6, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 1.6, ca: 0.0 }, rel_time: ActionColumn { burst: 1.8000002, press: 1.6000001, hold: 100.79997, na: 0.2, ca: 100.79997 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 1.8, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 1.6, ca: 0.0 }, rel_time: ActionColumn { burst: 2.0000002, press: 1.8000002, hold: 100.99997, na: 0.4, ca: 100.99997 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 2.0, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 2.0, ca: 0.0 }, rel_time: ActionColumn { burst: 2.2000003, press: 2.0000002, hold: 101.19997, na: 0.2, ca: 101.19997 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 2.2, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 2.0, ca: 0.0 }, rel_time: ActionColumn { burst: 2.4000003, press: 2.2000003, hold: 101.39996, na: 0.4, ca: 101.39996 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 2.4, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 2.4, ca: 0.0 }, rel_time: ActionColumn { burst: 2.6000004, press: 2.4000003, hold: 101.59996, na: 0.2, ca: 101.59996 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 2.6, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 2.4, ca: 0.0 }, rel_time: ActionColumn { burst: 2.8000004, press: 2.6000004, hold: 101.79996, na: 0.4, ca: 101.79996 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 2.8, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 2.8, ca: 0.0 }, rel_time: ActionColumn { burst: 3.0000005, press: 2.8000004, hold: 101.999954, na: 0.2, ca: 101.999954 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 3.0, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 2.8, ca: 0.0 }, rel_time: ActionColumn { burst: 3.2000005, press: 3.0000005, hold: 102.19995, na: 0.4, ca: 102.19995 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 3.2, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 3.2, ca: 0.0 }, rel_time: ActionColumn { burst: 3.4000006, press: 3.2000005, hold: 102.39995, na: 0.2, ca: 102.39995 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 3.4, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 3.2, ca: 0.0 }, rel_time: ActionColumn { burst: 3.6000006, press: 3.4000006, hold: 102.599945, na: 0.4, ca: 102.599945 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 3.6, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 3.6, ca: 0.0 }, rel_time: ActionColumn { burst: 3.8000007, press: 3.6000006, hold: 102.79994, na: 0.2, ca: 102.79994 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 3.8, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 3.6, ca: 0.0 }, rel_time: ActionColumn { burst: 4.0000005, press: 3.8000007, hold: 102.99994, na: 0.4, ca: 102.99994 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 4.0, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 4.0, ca: 0.0 }, rel_time: ActionColumn { burst: 4.2000003, press: 4.0000005, hold: 103.199936, na: 0.2, ca: 103.199936 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 4.2, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 4.0, ca: 0.0 }, rel_time: ActionColumn { burst: 4.4, press: 4.2000003, hold: 103.39993, na: 0.4, ca: 103.39993 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 4.4, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 4.4, ca: 0.0 }, rel_time: ActionColumn { burst: 4.6, press: 4.4, hold: 103.59993, na: 0.2, ca: 103.59993 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 4.6, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 4.4, ca: 0.0 }, rel_time: ActionColumn { burst: 4.7999997, press: 4.6, hold: 103.79993, na: 0.4, ca: 103.79993 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 4.8, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 4.8, ca: 0.0 }, rel_time: ActionColumn { burst: 4.9999995, press: 4.7999997, hold: 103.99992, na: 0.2, ca: 103.99992 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 5.0, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 4.8, ca: 0.0 }, rel_time: ActionColumn { burst: 5.1999993, press: 4.9999995, hold: 104.19992, na: 0.4, ca: 104.19992 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 5.2, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 5.2, ca: 0.0 }, rel_time: ActionColumn { burst: 5.399999, press: 5.1999993, hold: 104.39992, na: 0.2, ca: 104.39992 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 5.4, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 5.2, ca: 0.0 }, rel_time: ActionColumn { burst: 5.599999, press: 5.399999, hold: 104.599915, na: 0.4, ca: 104.599915 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 5.6, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 5.6, ca: 0.0 }, rel_time: ActionColumn { burst: 5.7999988, press: 5.599999, hold: 104.79991, na: 0.2, ca: 104.79991 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 5.8, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 5.6, ca: 0.0 }, rel_time: ActionColumn { burst: 5.9999986, press: 5.7999988, hold: 104.99991, na: 0.4, ca: 104.99991 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 6.0, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 6.0, ca: 0.0 }, rel_time: ActionColumn { burst: 6.1999984, press: 5.9999986, hold: 105.199905, na: 0.2, ca: 105.199905 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 6.2, abs_time: ActionColumn { burst: 0.0, press: 0.2, hold: 0.0, na: 6.0, ca: 0.0 }, rel_time: ActionColumn { burst: 6.399998, press: 6.1999984, hold: 105.3999, na: 0.4, ca: 105.3999 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 6.0, er: 0.0 }], 
            [ActionState { current_time: 6.4, abs_time: ActionColumn { burst: 0.0, press: 6.4, hold: 0.0, na: 6.0, ca: 0.0 }, rel_time: ActionColumn { burst: 6.599998, press: 0.2, hold: 105.5999, na: 0.6, ca: 105.5999 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 12.0, er: 0.0 }], 
            [ActionState { current_time: 6.6, abs_time: ActionColumn { burst: 0.0, press: 6.4, hold: 0.0, na: 6.6, ca: 0.0 }, rel_time: ActionColumn { burst: 6.799998, press: 0.4, hold: 105.7999, na: 0.2, ca: 105.7999 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 12.0, er: 0.0 }], 
            [ActionState { current_time: 6.8, abs_time: ActionColumn { burst: 0.0, press: 6.4, hold: 0.0, na: 6.6, ca: 0.0 }, rel_time: ActionColumn { burst: 6.9999976, press: 0.6, hold: 105.99989, na: 0.4, ca: 105.99989 }, atk_spd: 0.0, reduce_skill: 0.0, energy: 12.0, er: 0.0 }],
            ]
        }
    }

    #[test]
    fn simple_timeline() {
        let target = history_7at02();
        let mut history = History::<1>::new(7.0, 0.2);
        let mut character = Sim2TestCharacter::new();
        let mut weapon = NoopTimeline {};
        let mut artifact = NoopTimeline {};
        let mut states = [ActionState::new(); 1];
        let mut members = [TimelineMember {
            character: &mut character,
            weapon: &mut weapon,
            artifact: &mut artifact,
        }; 1];

        states[0].energy += 40.0;
        states[0].rel_time.add(99.0);
        decide_action(&mut history, &mut members, &mut states);

        assert_eq!(history.action, target.action);
        assert_eq!(states[0].energy, 12.0);
    }

    #[test]
    fn simple_damage() {
        let mut history = history_7at02();
        let mut enemy = Enemy::simple();
        let mut character = Sim2TestCharacter::new();
        let mut weapon = WeaponRecord::default();
        let mut artifact = Artifact::default();
        let mut members = [FieldMember {
            character: &mut character,
            weapon: &mut weapon,
            artifact: &mut artifact,
        }; 1];
        let cr = Sim2TestCharacter::record(Pyro);
        let wr = WeaponRecord::default();
        let ar = Artifact::default();
        let mut data = [CharacterData::new(0, &cr, &wr, &ar); 1];

        let dmg = calculate_damage(&mut history, &mut members, &mut data, &mut enemy);
        assert_eq!(dmg, 2400.0);
    }
}
