use crate::sim2::state::State;
use crate::sim2::timeline::ActionState;
use crate::sim2::attack::Attack;
use crate::sim2::types::{CharacterAction, DamageType, Vision, FieldCharacterIndex, FieldEnergy};
use crate::sim2::record::{TimelineMember, FieldMember, CharacterData, Enemy};

// why? to reuse allocations for all characters, because if `end_time` and
// `unit_time` are fixed, all histories have the same size.
#[derive(Debug)]
pub struct History<const N: usize> {
    pub end_time: f32,
    pub unit_time: f32,
    pub action: Vec<[CharacterAction; N]>,
    pub state: Vec<[ActionState; N]>,
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
        // TODO
        1 + (time / self.unit_time).floor() as usize
    }
}

pub fn decide_action<const N: usize>(history: &mut History<N>, members: &mut [TimelineMember; N], states: &mut [ActionState; N], data: &mut [CharacterData; N]) -> () {
    let mut field_energy: Vec<FieldEnergy> = Vec::new();
    let mut current_time: f32 = 0.0;
    let mut idx = 0;
    while current_time <= history.end_time {
        let mut actions = [CharacterAction::StandStill; N];
        for (i, member) in members.iter_mut().enumerate() {
            history.state[idx][i].copy(&states[i]);
            let action = member.character.decide_action(&states[i], &mut data[i]);
            let state = &mut states[i];
            let d = &data[i];
            state.init(d);
            member.character.accelerate(&mut field_energy, &action, state, d);
            member.weapon.accelerate(&mut field_energy, &action, state, d);
            member.artifact.accelerate(&mut field_energy, &action, state, d);
            actions[i] = action;
        }
        for (i, member) in members.iter_mut().enumerate() {
            let mut energy: f32 = 0.0;
            for fe in field_energy.iter() {
                match fe {
                    FieldEnergy::Particle(ref p) => energy += if i == 0 {
                        p.on_field_energy(&data[i].character.vision) * states[i].er()
                    } else {
                        p.off_field_energy(&data[i].character.vision) * states[i].er()
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

pub fn calculate_damage<const N: usize>(history: &mut History<N>, members: &mut [FieldMember; N], data: &[CharacterData; N], enemy: &mut Enemy) -> f32 {
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
        let state = &mut states[attack.idx.0];
        state.init(&data[attack.idx.0]);
        for i in 0..N {
            // character state first
            let d = &data[i];
            let member = &mut members[i];
            let action_state = &history.state[history.state_index(attack.time)][i];
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

    use crate::sim2::testutil;
    use crate::sim2::testutil::{Sim2TestCharacter, NoopTimeline};
    use crate::sim2::element::{ElementalGauge, ElementalGaugeDecay};
    use crate::sim2::types::{Vision};
    use crate::sim2::timeline::{ActionColumn, Timeline};
    use crate::sim2::record::{WeaponRecord, Artifact};

    use Vision::*;

    #[test]
    fn simple_timeline() {
        let target = testutil::history_7at02();
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
        let cr = Sim2TestCharacter::record(Pyro);
        let wr = WeaponRecord::default();
        let ar = Artifact::default();
        let mut data = [CharacterData::new(0, &cr, &wr, &ar); 1];

        states[0].energy += 40.0;
        states[0].rel_time.add(99.0);
        decide_action(&mut history, &mut members, &mut states, &mut data);

        assert_eq!(history.action, target.action);
        assert_eq!(states[0].energy, 12.0);
    }

    #[test]
    fn simple_damage() {
        let mut history = testutil::history_7at02();
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
        let expect = 17.*100. + 2.*200. + 1.*300.;
        assert_eq!(dmg, expect);
    }

    #[test]
    fn infuse_goblet() {
        let mut history = testutil::history_7at02();
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
        let ar = Artifact::default().pyro_dmg(50.0);
        let mut data = [CharacterData::new(0, &cr, &wr, &ar); 1];
        let dmg = calculate_damage(&mut history, &mut members, &mut data, &mut enemy);
        let expect = 17.*100. + 2.*200.*1.5 + 1.*300.*1.5;
        assert_eq!(dmg, expect);
    }

    #[test]
    fn attack_infusion() {
        let mut history = testutil::history_7at02();
        let mut enemy = Enemy::simple();
        let mut character = Sim2TestCharacter::new().infusion(true);
        let mut weapon = WeaponRecord::default();
        let mut artifact = Artifact::default();
        let mut members = [FieldMember {
            character: &mut character,
            weapon: &mut weapon,
            artifact: &mut artifact,
        }; 1];
        let cr = Sim2TestCharacter::record(Pyro);
        let wr = WeaponRecord::default();
        let ar = Artifact::default().pyro_dmg(50.0);
        let mut data = [CharacterData::new(0, &cr, &wr, &ar); 1];
        let dmg = calculate_damage(&mut history, &mut members, &mut data, &mut enemy);
        let expect = 1.5 * (17.*100. + 2.*200. + 1.*300.);
        assert_eq!(dmg, 3600.0);
    }

    #[test]
    fn vaporize() {
        let mut history = testutil::history_7at02();
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
        enemy.aura = ElementalGauge {
            aura: Hydro,
            unit: 1.,
            decay: ElementalGaugeDecay::A,
        };
        let dmg = calculate_damage(&mut history, &mut members, &mut data, &mut enemy);
        // two vaporize by burst and the 1st skill, aura application by the 2nd
        // skill
        let expect = 17.*100. + 200. + 200.*1.5 + 1.*300.*1.5;
        assert_eq!(dmg, expect);
        assert_eq!(enemy.aura.aura, Pyro);
    }

    #[test]
    fn melt() {
        let mut history = testutil::history_7at02();
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
        enemy.aura = ElementalGauge {
            aura: Cryo,
            unit: 1.,
            decay: ElementalGaugeDecay::A,
        };
        let dmg = calculate_damage(&mut history, &mut members, &mut data, &mut enemy);
        let expect = 17.*100. + 2.*200. + 1.*300.*2.;
        assert_eq!(dmg, expect);
        assert_eq!(enemy.aura.aura, Pyro);
    }

    #[test]
    fn superconduct() {
        let mut history = testutil::history_7at02();
        let mut enemy = Enemy::simple();
        let mut character = Sim2TestCharacter::new();
        let mut weapon = WeaponRecord::default();
        let mut artifact = Artifact::default();
        let mut members = [FieldMember {
            character: &mut character,
            weapon: &mut weapon,
            artifact: &mut artifact,
        }; 1];
        let cr = Sim2TestCharacter::record(Cryo);
        let wr = WeaponRecord::default();
        let ar = Artifact::default();
        let mut data = [CharacterData::new(0, &cr, &wr, &ar); 1];
        enemy.aura = ElementalGauge {
            aura: Electro,
            unit: 1.,
            decay: ElementalGaugeDecay::A,
        };
        let dmg = calculate_damage(&mut history, &mut members, &mut data, &mut enemy);
        let expect: f32 = 17.*100.*1.2 + 2.*200. + 1.*300. + 725.36;
        assert_eq!(dmg.floor(), expect.floor());
        assert_eq!(enemy.aura.aura, Cryo);
    }

    #[test]
    fn atk_spd() {
        let mut history = History::<1>::new(4.0, 0.2);
        let mut character = Sim2TestCharacter::new();
        let mut weapon = NoopTimeline {};
        let mut artifact = NoopTimeline {};
        let mut states = [ActionState::new(); 1];
        let mut members = [TimelineMember {
            character: &mut character,
            weapon: &mut weapon,
            artifact: &mut artifact,
        }; 1];
        let cr = Sim2TestCharacter::record(Pyro);
        let wr = WeaponRecord::default().atk_spd(100.);
        let ar = Artifact::default();
        let mut data = [CharacterData::new(0, &cr, &wr, &ar); 1];

        states[0].rel_time.add(99.0);
        decide_action(&mut history, &mut members, &mut states, &mut data);

        use CharacterAction::*;
        let expect = vec![[PressSkill], [Na1], [Na2], [Na3], [Na4], [Na1],
        [Na2], [Na3], [Na4], [Na1], [Na2], [Na3], [Na4], [Na1], [Na2], [Na3],
        [Na4], [Na1], [Na2], [Na3]];
        assert_eq!(history.action, expect);
    }

    #[test]
    fn two_members_timeline() {
        let target = testutil::history_2at02();
        let mut history = History::<2>::new(2.0, 0.2);
        let cr = Sim2TestCharacter::record(Pyro);
        let wr = WeaponRecord::default();
        let ar = Artifact::default();
        let mut data = [CharacterData::new(0, &cr, &wr, &ar), CharacterData::new(1, &cr, &wr, &ar)];
        let mut states = [ActionState::new(); 2];

        let mut character1 = Sim2TestCharacter::new();
        let mut weapon1 = NoopTimeline {};
        let mut artifact1 = NoopTimeline {};
        let mut character2 = Sim2TestCharacter::new();
        let mut weapon2 = NoopTimeline {};
        let mut artifact2 = NoopTimeline {};
        let mut members = [TimelineMember {
            character: &mut character1,
            weapon: &mut weapon1,
            artifact: &mut artifact1,
        }, TimelineMember {
            character: &mut character2,
            weapon: &mut weapon2,
            artifact: &mut artifact2,
        }];

        states[0].rel_time.add(99.);
        states[1].rel_time.add(99.);
        states[0].energy = 40.;
        states[1].energy = 40.;
        decide_action(&mut history, &mut members, &mut states, &mut data);

        assert_eq!(history.action, target.action);
        assert_eq!(states[0].energy, 12.0);
        assert_eq!(states[1].energy, 7.2);
    }

    #[test]
    fn two_members_damage() {
        let mut history = testutil::history_2at02();
        let mut enemy = Enemy::simple();
        let cr = Sim2TestCharacter::record(Pyro);
        let wr = WeaponRecord::default();
        let ar = Artifact::default();
        let mut data = [CharacterData::new(0, &cr, &wr, &ar), CharacterData::new(1, &cr, &wr, &ar)];

        let mut character1 = Sim2TestCharacter::new();
        let mut weapon1 = WeaponRecord::default();
        let mut artifact1 = Artifact::default();
        let mut character2 = Sim2TestCharacter::new();
        let mut weapon2 = WeaponRecord::default();
        let mut artifact2 = Artifact::default();
        let mut members = [FieldMember {
            character: &mut character1,
            weapon: &mut weapon1,
            artifact: &mut artifact1,
        }, FieldMember {
            character: &mut character2,
            weapon: &mut weapon2,
            artifact: &mut artifact2,
        }];

        let dmg = calculate_damage(&mut history, &mut members, &mut data, &mut enemy);
        let expect = 8.*100. + 2.*200. + 2.*300.;
        assert_eq!(dmg, expect);
    }

    // TODO icd
}
