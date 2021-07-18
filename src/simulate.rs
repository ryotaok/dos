use std::borrow::Borrow;

use crate::state::State;
use crate::fc::{FieldCharacter, Enemy, FieldCharacterData};
use crate::types::{AttackType};
use crate::action::{Attack, AttackEvent, TimerGuard};

use AttackType::*;

// `members` is assumed that they are ordered in such a way:
//     0 = the main DPS character who does normal attacks frequently
// later = support characters whose skills and bursts are prioritized higher
//         than the 0 member's. They never do normal attacks or charge attacks.
pub fn simulate(members: &mut [FieldCharacterData], enemy: &mut Enemy, time: f32) -> f32 {
    let mut maybe_attack: Vec<Option<AttackEvent>> = Vec::with_capacity(members.len());
    // perform an attack
    for i in (0..members.len()).rev() {
        let FieldCharacterData { fc, character, .. } = &members[i];
        maybe_attack.push(if let Some(attack) = character.maybe_attack(fc) {
            Some(AttackEvent::new(attack, fc.idx.0))
        } else {
            None
        });
    }
    let mut attack_event = AttackEvent::empty();
    'outer: for a in &[Burst, HoldSkill, PressSkill, Ca, Na] {
        for &ma in maybe_attack.iter() {
            if let Some(ref ma) = ma {
                if ma.kind == *a {
                    attack_event = *ma;
                    break 'outer;
                }
            }
        }
    }
    println!("{:?}", attack_event);
    // collect attacks
    for FieldCharacterData { fc, atk_queue, particles, character, weapon, artifact } in members.iter_mut() {
        character.additional_attack(atk_queue, particles, fc, &enemy);
        weapon.additional_attack(atk_queue, particles, fc, &enemy);
        artifact.additional_attack(atk_queue, particles, fc, &enemy);
    }
    // update to the on-field attack
    let mut modifiable_state: Vec<State> = Vec::with_capacity(members.len());
    for _ in 0..members.len() {
        modifiable_state.push(State::new());
    }
    for FieldCharacterData { fc, atk_queue, particles, character, weapon, artifact } in members.iter_mut() {
        let mut guard = TimerGuard::with_first_2(&attack_event, &fc);
        character.update(&mut guard, &atk_queue, &particles, fc, &enemy, time);
        // TODO review how to use TimerGuard; it has prevented CD from starting
        // even if the character does nothing.
        guard.first = true;
        weapon.update(&mut guard, &atk_queue, &particles, fc, &enemy, time);
        artifact.update(&mut guard, &atk_queue, &particles, fc, &enemy, time);
        character.modify(&mut modifiable_state, fc, enemy);
        weapon.modify(&mut modifiable_state, fc, enemy);
        artifact.modify(&mut modifiable_state, fc, enemy);
        character.accelerate((*weapon).borrow());
        // weapon.accelerate(character.timer_mut());
        // artifact.accelerate(character.timer_mut());
    }
    // update state and energy
    for (mut new_state, FieldCharacterData { fc, particles, .. }) in modifiable_state.into_iter().zip(members.iter_mut()) {
        if attack_event.kind == Burst && attack_event.idx == fc.idx {
            // consume energy on burst
            new_state.energy.0 -= fc.cr.energy_cost;
        }
        // attacker is on field
        if fc.idx.0 == 0 {
            // TODO fix particles can be obtained by everyone
            // since particles need to be distributed to all members, we cannot
            // `drain` them.
            for p in particles.iter() {
                new_state.energy.0 += p.on_field_energy(&fc.vision);
            }
        } else {
            for p in particles.iter() {
                new_state.energy.0 += p.off_field_energy(&fc.vision);
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
    let mut atk_queue: Vec<*const Attack> = Vec::with_capacity(members.iter().fold(0, |acc, q| acc + q.atk_queue.len()));
    for FieldCharacterData { atk_queue: q, .. } in members.iter_mut() {
        atk_queue.append(q);
    }
    for attack in atk_queue.into_iter() {
        let attack = unsafe { &*attack };
        let FieldCharacterData { fc, character, weapon, artifact, .. } = &members[attack.idx.0];
        let state = match (
            character.intensify(&attack),
            weapon.intensify(&attack),
            artifact.intensify(&attack),
        ) {
            (None, None, None) => None,
            (Some(s), None, None) => Some(s),
            (None, Some(s), None) => Some(s),
            (None, None, Some(s)) => Some(s),
            (Some(mut s), Some(t), None) => { s.merge(&t); Some(s) },
            (Some(mut s), None, Some(t)) => { s.merge(&t); Some(s) },
            (None, Some(mut s), Some(t)) => { s.merge(&t); Some(s) },
            (Some(mut s), Some(t), Some(u)) => {
                s.merge(&t);
                s.merge(&u);
                Some(s)
            }
        };
        total_dmg += calculate(attack, state, fc, enemy);
    }
    // remove all particles
    for FieldCharacterData { particles, .. } in members.iter_mut() {
        particles.clear()
    }
    enemy.update(time);
    total_dmg
}

// calculate damage of the character
pub fn calculate(attack: &Attack, state: Option<State>, fc: &FieldCharacter, enemy: &mut Enemy) -> f32 {
    // TODO assert `State` intensified will not have `infusion`.
    let attack_element = fc.infused_element(attack);
    let mut dmg = attack.outgoing_damage(attack_element, state, fc);
    dmg = attack.incoming_damage(attack_element, dmg, fc, enemy);
    println!("  dmg = {:?}: {:?}({:?})", dmg, &attack.kind, attack.idx.0);
    // println!("{:?} {:?}", &attack.kind, dmg);
    dmg
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::{TestEnvironment};
    use crate::types::{Vision, ElementalGauge, ElementalGaugeDecay};
    use crate::fc::FieldCharacterIndex;

    use Vision::*;

    #[test]
    fn simple_setup() {
        let mut members = vec![TestEnvironment::fc(State::new())];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..11 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // skill na na na
        let expect = 200.0 + 100.0 + 100.0 + 100.0 ;
        assert_eq!(total_dmg, 0.5 * expect);
    }

    #[test]
    fn ca() {
        let mut members = vec![TestEnvironment::ca(FieldCharacterIndex(0), State::new(), Pyro)];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..11 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // skill (na ca) na na na
        let expect = 200.0 + (100.0 + 150.0) + 3.0 * 100.0;
        assert_eq!(total_dmg, 0.5 * expect);
    }

    #[test]
    fn hold() {
        let mut members = vec![TestEnvironment::hold(FieldCharacterIndex(0), State::new(), Pyro)];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..11 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // hold na na skill na
        let expect = 250.0 + 200.0 + 3.0 * 100.0;
        assert_eq!(total_dmg, 0.5 * expect);
    }

    #[test]
    fn hold_and_recharge() {
        let mut members = vec![TestEnvironment::hold(FieldCharacterIndex(0), State::new(), Pyro)];
        let mut enemy = TestEnvironment::enemy();
        let mut _total_dmg = 0.0;
        for _ in 0..11 {
            _total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // hold na na skill na
        let energy = members[0].fc.state.energy.0;
        assert_eq!(energy, 15.0);
    }

    #[test]
    fn burst() {
        let mut members = vec![TestEnvironment::fc(State::new())];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        members[0].fc.state.energy.0 = members[0].fc.cr.energy_cost;
        for _ in 0..11 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // burst skill na na na
        let expect = 300.0 + 200.0 + 3.0 * 100.0;
        assert_eq!(total_dmg, 0.5 * expect);
    }

    #[test]
    fn burst_and_recharge() {
        let mut members = vec![TestEnvironment::fc(State::new())];
        let mut enemy = TestEnvironment::enemy();
        let mut _total_dmg = 0.0;
        members[0].fc.state.energy.0 = members[0].fc.cr.energy_cost;
        for _ in 0..11 {
            _total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        let energy = members[0].fc.state.energy.0;
        // the skill was used once.
        assert_eq!(energy, 6.0);
    }

    #[test]
    fn infuse_goblet() {
        let mut members = vec![TestEnvironment::fc(State::new().pyro_dmg(46.6))];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..11 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // skill na na na
        let expect = 200.0 * 1.466 + 100.0 + 100.0 + 100.0;
        assert_eq!(total_dmg, 0.5 * expect);
    }

    #[test]
    fn attack_infusion() {
        let mut members = vec![TestEnvironment::fc(State::new().pyro_dmg(46.6).infusion(true))];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..11 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // skill na na na
        let expect = 1.466 * (200.0 + 100.0 + 100.0 + 100.0);
        let differnce = (total_dmg - 0.5 * expect).abs();
        // assert!(differnce <= f32::EPSILON);
        assert!(differnce <= 0.001);
    }

    #[test]
    fn vaporize() {
        let mut members = vec![
            TestEnvironment::vision(FieldCharacterIndex(0), State::new(), Pyro)
        ];
        let mut enemy = TestEnvironment::enemy();
        enemy.aura = ElementalGauge {
            aura: Hydro,
            unit: 1.0,
            decay: ElementalGaugeDecay::A,
        };
        let mut total_dmg = 0.0;
        for _ in 0..11 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // skill na na na
        let expect = 200.0 * 1.5 + 100.0 + 100.0 + 100.0;
        assert_eq!(total_dmg, 0.5 * expect);
        assert_eq!(enemy.aura.aura, Hydro);
        assert!(0.0 < enemy.aura.unit && enemy.aura.unit < 1.0);
    }

    #[test]
    fn icd_of_reaction_1_hit_counter() {
        let mut members = vec![
            TestEnvironment::no_skill(FieldCharacterIndex(0), State::new().infusion(true), Pyro)
        ];
        let mut enemy = TestEnvironment::enemy();
        enemy.aura = ElementalGauge {
            aura: Hydro,
            unit: 1.0,
            decay: ElementalGaugeDecay::A,
        };
        let mut total_dmg = 0.0;
        for _ in 0..11 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // vaporize na na vaporize
        let expect = 2.0 * 150.0 + 2.0 * 100.0;
        assert_eq!(total_dmg, 0.5 * expect);
        assert_eq!(enemy.aura, ElementalGauge::default());
    }

    #[test]
    fn icd_of_reaction_2_timer() {
        let mut members = vec![
            TestEnvironment::no_skill(FieldCharacterIndex(0), State::new().infusion(true), Pyro)
        ];
        let mut enemy = TestEnvironment::enemy();
        enemy.aura = ElementalGauge {
            aura: Hydro,
            unit: 99.0,
            decay: ElementalGaugeDecay::A,
        };
        let mut total_dmg = 0.0;
        for _ in 0..4 {
            total_dmg += simulate(&mut members, &mut enemy, 3.0);
        }
        // vaporize vaporize vaporize
        let expect = 3.0 * 150.0;
        assert_eq!(total_dmg, 0.5 * expect);
        assert_eq!(enemy.aura.aura, Hydro);
    }

    #[test]
    fn melt() {
        let mut members = vec![
            TestEnvironment::vision(FieldCharacterIndex(0), State::new(), Pyro)
        ];
        let mut enemy = TestEnvironment::enemy();
        enemy.aura = ElementalGauge {
            aura: Cryo,
            unit: 1.0,
            decay: ElementalGaugeDecay::A,
        };
        let mut total_dmg = 0.0;
        for _ in 0..11 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // skill na na na
        let expect = 200.0 * 2.0 + 100.0 + 100.0 + 100.0;
        assert_eq!(total_dmg, 0.5 * expect);
        assert_eq!(enemy.aura, ElementalGauge::default());
    }

    #[test]
    fn superconduct() {
        let mut members = vec![
            TestEnvironment::vision(FieldCharacterIndex(0), State::new(), Cryo)
        ];
        let mut enemy = TestEnvironment::enemy();
        enemy.aura = ElementalGauge {
            aura: Electro,
            unit: 1.0,
            decay: ElementalGaugeDecay::A,
        };
        let mut total_dmg = 0.0;
        for _ in 0..11 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // (skill superconduct) na na na
        // TODO transformative_reaction ignores enemy resistance
        let expect = 200.0 + 725.36 * 2.0 + 120.0 + 120.0 + 120.0;
        assert_eq!(total_dmg, 0.5 * expect);
        assert_eq!(enemy.aura, ElementalGauge::default());
    }

    #[test]
    fn atk_spd() {
        let mut members = vec![TestEnvironment::fc(State::new().atk_spd(100.0))];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..21 {
            total_dmg += simulate(&mut members, &mut enemy, 0.1);
        }
        // skill na na na (na na na)
        let expect = 200.0 + 6.0 * 100.0 ;
        assert_eq!(total_dmg, 0.5 * expect);
    }

    #[test]
    fn two_members() {
        let mut members = vec![
            TestEnvironment::fc(State::new()),
            TestEnvironment::fc1(State::new()),
            ];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..21 {
            total_dmg += simulate(&mut members, &mut enemy, 0.1);
        }
        // twice (skill na na na)
        let expect = 2.0 * (200.0 + 100.0 + 100.0 + 100.0 );
        assert_eq!(total_dmg, 0.5 * expect);
    }
}
