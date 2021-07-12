use crate::state::State;
use crate::fc::{FieldCharacter, Enemy, FieldCharacterData};
use crate::types::{AttackType, Vision};
use crate::action::{Attack, TimerGuard};

// `members` is assumed that they are ordered in such a way:
//     0 = the main DPS character who does normal attacks frequently
// later = support characters whose skills and bursts are prioritized higher
//         than the 0 member's. They never do normal attacks or charge attacks.
pub fn simulate(members: &mut [FieldCharacterData], enemy: &mut Enemy, time: f32) -> f32 {
    let mut on_field_attack: Option<Attack> = None;
    // perform an attack
    for action_idx in &[0, 1, 2] { // 0 = burst, 1 = skill, 2 = na
        if let Some(_) = on_field_attack {
            break;
        }
        for i in (0..members.len()).rev() {
            let (fc, _, _, fa) = &members[i];
            match (action_idx, &on_field_attack) {
                (&0, None) => on_field_attack = fa.burst.value(fc),
                (&1, None) => on_field_attack = fa.skill.value(fc),
                (&2, None) => on_field_attack = fa.na.value(fc),
                _ => break,
            };
        }
    }
    // collect attacks
    for (fc, fe, atk_queue, fa) in members.iter_mut() {
        fe.character.additional_attack(atk_queue, fc, fa, &enemy);
        fe.weapon.additional_attack(atk_queue, fc, fa, &enemy);
        fe.artifact.additional_attack(atk_queue, fc, fa, &enemy);
    }
    let empty_attack = Attack::empty();
    let attack: *const Attack = if let Some(attack) = on_field_attack {
        let i = attack.on_field_character_index;
        let atk_queue = &mut members[i].2;
        atk_queue.insert(0, attack);
        &members[i].2[0]
    } else {
        &empty_attack
    };
    // update to the on-field attack
    let mut modifiable_state: Vec<State> = Vec::with_capacity(members.len());
    for _ in 0..members.len() {
        modifiable_state.push(State::new());
    }
    for (fc, fe, atk_queue, fa) in members.iter_mut() {
        let mut guard = unsafe {
            TimerGuard::with_first(&*attack, &fc)
        };
        unsafe {
            fa.burst.update(&guard, &*attack, &*atk_queue, time);
            fa.skill.update(&guard, &*attack, &*atk_queue, time);
            fa.na.update(&guard, &*attack, &*atk_queue, time);
        }
        fe.character.update(&mut guard, &atk_queue, fc, &enemy, time);
        fe.weapon.update(&mut guard, &atk_queue, fc, &enemy, time);
        fe.artifact.update(&mut guard, &atk_queue, fc, &enemy, time);
        fe.character.modify(&mut modifiable_state, fc, enemy);
        fe.weapon.modify(&mut modifiable_state, fc, enemy);
        fe.artifact.modify(&mut modifiable_state, fc, enemy);
        fe.character.accelerate(&mut fa.na, &mut fa.skill, &mut fa.burst);
        fe.weapon.accelerate(&mut fa.na, &mut fa.skill, &mut fa.burst);
        fe.artifact.accelerate(&mut fa.na, &mut fa.skill, &mut fa.burst);
        fa.na.spd += fc.state.atk_spd;
    }
    // update state and energy
    for (mut new_state, (fc, _, atk_queue, _)) in modifiable_state.drain(..).zip(members.iter_mut()) {
        for attack in atk_queue.iter() {
            if fc.idx.0 == attack.on_field_character_index {
                match (&attack.kind, &attack.particle, &attack.element) {
                    // consume energy on burst
                    (AttackType::Burst, _, _) => new_state.energy.0 -= fc.state.energy_cost,
                    // physical particles mean neutral energy
                    (_, Some(p), Vision::Physical) => new_state.energy.0 += 2.0 * (*p) * fc.state.ER(),
                    // the element of the on field character is always same as the one of the particles
                    (_, Some(p), _) => new_state.energy.0 += 3.0 * (*p) * fc.state.ER(),
                    _ => (),
                }
            } else {
                match (&attack.particle, fc.vision == attack.element, &attack.element) {
                    (Some(p), _, Vision::Physical) => new_state.energy.0 += 1.2 * (*p) * fc.state.ER(),
                    (Some(p), true, _)  => new_state.energy.0 += 1.8 * (*p) * fc.state.ER(),
                    (Some(p), false, _) => new_state.energy.0 += 0.6 * (*p) * fc.state.ER(),
                    _ => (),
                };
            }
        }
        new_state.merge(&fc.cr.state());
        new_state.merge(&fc.wr.state());
        new_state.merge(&fc.ar.state);
        new_state.energy.0 += fc.state.energy.0;
        fc.state = new_state;
    }
    // calculate damages
    let mut total_dmg = 0.0;
    // TODO slow?
    let mut atk_queue: Vec<Attack> = Vec::with_capacity(members.iter().fold(0, |acc, q| acc + q.2.len()));
    for (_, _, q, _) in members.iter_mut() {
        atk_queue.append(q);
    }
    for mut attack in atk_queue.drain(..) {
        if attack.kind != AttackType::StandStill {
            let fc = &members[attack.on_field_character_index].0;
            let fa = &members[attack.on_field_character_index].1;
            fa.character.intensify(&mut attack, &fc, &enemy);
            fa.weapon.intensify(&mut attack, &fc, &enemy);
            fa.artifact.intensify(&mut attack, &fc, &enemy);
            total_dmg += calculate(attack, fc, enemy);
        }
    }
    enemy.update(time);
    total_dmg
}

// calculate damage of the character
pub fn calculate(mut attack: Attack, fc: &FieldCharacter, enemy: &mut Enemy) -> f32 {
    let mut dmg = attack.outgoing_damage(fc);
    dmg = attack.incoming_damage(dmg, fc, enemy);
    // println!("{:?} {:?} {:?}", attack.on_field_character_index, &attack.kind, dmg);
    // println!("{:?} {:?}", &attack.kind, dmg);
    dmg
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::{TestEnvironment, TestCharacter, TestWeapon, TestArtifact};
    use crate::types::{Vision, ElementalGauge, ElementalGaugeDecay};

    #[test]
    fn simple_setup() {
        let mut members = vec![TestEnvironment::fc(State::new())];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // skill na na na
        let expect = 0.5 * (200.0 + 100.0 + 100.0 + 100.0 );
        assert_eq!(total_dmg, expect);
    }

    #[test]
    fn burst() {
        let mut members = vec![TestEnvironment::fc(State::new())];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        members[0].0.state.energy.0 = members[0].0.cr.energy_cost;
        for _ in 0..10 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // burst skill na na
        let expect = 0.5 * (300.0 + 200.0 + 100.0 + 100.0);
        assert_eq!(total_dmg, expect);
    }

    #[test]
    fn burst_and_recharge() {
        let mut members = vec![TestEnvironment::fc(State::new())];
        let mut enemy = TestEnvironment::enemy();
        let mut _total_dmg = 0.0;
        members[0].0.state.energy.0 = members[0].0.cr.energy_cost;
        for _ in 0..10 {
            _total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        let energy = members[0].0.state.energy.0;
        // the skill was used once.
        assert_eq!(energy, 6.0);
    }

    #[test]
    fn infuse_goblet() {
        let mut members = vec![TestEnvironment::fc(State::new().pyro_dmg(46.6))];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // skill na na na
        let expect = 0.5 * (200.0 * 1.466 + 100.0 + 100.0 + 100.0);
        assert_eq!(total_dmg, expect);
    }

    #[test]
    fn attack_infusion() {
        let mut members = vec![TestEnvironment::fc(State::new().pyro_dmg(46.6).infusion(true))];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // skill na na na
        let expect = 0.5 * 1.466 * (200.0 + 100.0 + 100.0 + 100.0);
        let differnce = (total_dmg - expect).abs();
        // assert!(differnce <= f32::EPSILON);
        assert!(differnce <= 0.001);
    }

    #[test]
    fn vaporize() {
        let mut members = vec![TestEnvironment::vision(State::new(), "Pyro")];
        let mut enemy = TestEnvironment::enemy();
        enemy.aura = ElementalGauge {
            aura: Vision::Hydro,
            unit: 1.0,
            decay: ElementalGaugeDecay::A,
        };
        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // skill na na na
        let expect = 0.5 * (200.0 * 1.5 + 100.0 + 100.0 + 100.0);
        assert_eq!(total_dmg, expect);
        assert_eq!(enemy.aura.aura, Vision::Hydro);
        assert!(0.0 < enemy.aura.unit && enemy.aura.unit < 1.0);
    }

    #[test]
    fn icd_of_reaction() {
        let mut members = vec![TestEnvironment::no_skill(
            Box::new(TestCharacter::new()),
            Box::new(TestWeapon),
            Box::new(TestArtifact(State::new().infusion(true)))
        )];
        let mut enemy = TestEnvironment::enemy();
        enemy.aura = ElementalGauge {
            aura: Vision::Hydro,
            unit: 1.0,
            decay: ElementalGaugeDecay::A,
        };
        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // na na na
        let expect = 0.5 * (150.0 + 100.0 + 100.0);
        assert_eq!(total_dmg, expect);
        assert_eq!(enemy.aura.aura, Vision::Hydro);
        assert!(0.0 < enemy.aura.unit && enemy.aura.unit < 1.0);
    }

    #[test]
    fn melt() {
        let mut members = vec![TestEnvironment::vision(State::new(), "Pyro")];
        let mut enemy = TestEnvironment::enemy();
        enemy.aura = ElementalGauge {
            aura: Vision::Cryo,
            unit: 1.0,
            decay: ElementalGaugeDecay::A,
        };
        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // skill na na na
        let expect = 0.5 * (200.0 * 2.0 + 100.0 + 100.0 + 100.0);
        assert_eq!(total_dmg, expect);
        assert_eq!(enemy.aura, ElementalGauge::default());
    }

    #[test]
    fn superconduct() {
        let mut members = vec![TestEnvironment::vision(State::new(), "Cryo")];
        let mut enemy = TestEnvironment::enemy();
        enemy.aura = ElementalGauge {
            aura: Vision::Electro,
            unit: 1.0,
            decay: ElementalGaugeDecay::A,
        };
        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // (skill superconduct) na na na
        // TODO transformative_reaction ignores enemy resistance
        let expect = 0.5 * ((200.0 + 725.36 * 2.0) + 120.0 + 120.0 + 120.0);
        assert_eq!(total_dmg, expect);
        assert_eq!(enemy.aura, ElementalGauge::default());
    }

    #[test]
    fn atk_spd() {
        let mut members = vec![TestEnvironment::fc(State::new().atk_spd(50.0))];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // skill na na na (na na)
        let expect = 0.5 * (200.0 + 100.0 + 100.0 + 100.0   + 100.0 + 100.0);
        assert_eq!(total_dmg, expect);
    }

    #[test]
    fn two_members() {
        let mut members = vec![
            TestEnvironment::fc(State::new()),
            TestEnvironment::fc1(State::new()),
            ];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..20 {
            total_dmg += simulate(&mut members, &mut enemy, 0.1);
        }
        // twice (skill na na na)
        let expect = 2.0 * 0.5 * (200.0 + 100.0 + 100.0 + 100.0 );
        assert_eq!(total_dmg, expect);
    }
}
