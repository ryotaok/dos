use crate::sim1::state::State;
use crate::sim1::fc::{CharacterData, FieldAbility, Enemy};
use crate::sim1::types::{Vision, AttackType, FieldEnergy};
use crate::sim1::action::{Attack, AttackEvent};

use AttackType::*;

// `members` is assumed that they are ordered in such a way:
//     0 = the main DPS character who does normal attacks frequently
// later = support characters whose skills and bursts are prioritized higher
//         than the 0 member's. They never do normal attacks or charge attacks.
pub fn simulate(time: f32, members: &mut [CharacterData], abilities: &mut [FieldAbility], atk_queue: &mut Vec<*const Attack>, field_energy: &mut Vec<FieldEnergy>, enemy: &mut Enemy) -> f32 {
    // perform an attack
    let mut burst: Option<AttackEvent> = None;
    let mut skill: Option<AttackEvent> = None;
    let mut ca: Option<AttackEvent> = None;
    let mut na: Option<AttackEvent> = None;
    for i in (0..abilities.len()).rev() {
        let data = &members[i];
        let fa = &abilities[i];
        if burst.is_none() {
            burst = fa.character.burst_ref().maybe_attack(data);
            if burst.is_some() {
                continue;
            }
        }
        if skill.is_none() {
            skill = fa.character.skill_ref().maybe_attack(data);
            if skill.is_some() {
                continue;
            }
        }
        if ca.is_none() {
            ca = fa.character.ca_ref().maybe_attack(data);
            if ca.is_some() {
                continue;
            }
        }
        if na.is_none() {
            na = fa.character.na_ref().maybe_attack(data);
            if na.is_none() {
                na = fa.character.maybe_attack(data);
            }
        }
    }
    let attack_event = match (burst, skill, ca, na) {
        (Some(attack_event), _, _, _) => attack_event,
        (None, Some(attack_event), _, _) => attack_event,
        (None, None, Some(attack_event), _) => attack_event,
        (None, None, None, Some(attack_event)) => attack_event,
        _ => AttackEvent::empty(),
    };
    // println!("{:?}", attack_event);
    // collect attacks
    for i in 0..members.len() {
        abilities[i].additional_attack(atk_queue, field_energy, &members[i]);
        abilities[i].modify(members, enemy);
        abilities[i].accelerate();
    }
    // update state and energy
    for data in members.iter_mut() {
        if attack_event.kind == Burst && attack_event.idx == data.idx {
            // consume energy on burst
            data.state.energy -= data.character.energy_cost;
        }
        // attacker is on field
        if data.idx.0 == 0 {
            // since field_energy need to be distributed to all members, we cannot
            // `drain` them.
            for fe in field_energy.iter() {
                match fe {
                    FieldEnergy::Particle(ref p) => data.state.energy += p.on_field_energy(&data.character.vision) * data.state.ER(),
                    FieldEnergy::Energy(e) => data.state.energy += e,
                }
            }
        } else {
            for fe in field_energy.iter() {
                match fe {
                    FieldEnergy::Particle(ref p) => data.state.energy += p.off_field_energy(&data.character.vision) * data.state.ER(),
                    FieldEnergy::Energy(e) => data.state.energy += e,
                }
            }
        }
    }
    // calculate damages
    let mut total_dmg = 0.0;
    for &atk_ptr in atk_queue.iter() {
        let attack = unsafe { & *atk_ptr };
        let data = &members[attack.idx.0];
        let fa = &abilities[attack.idx.0];
        total_dmg += calculate(&attack, fa.intensify(&attack), data, enemy);
    }
    for i in 0..members.len() {
        abilities[i].update(time, &attack_event, &members[i], &atk_queue, &field_energy, &enemy);
    }
    // remove all field_energy
    field_energy.clear();
    atk_queue.clear();
    enemy.update(time);
    total_dmg
}

// calculate damage of the character
pub fn calculate(attack: &Attack, state: Option<State>, fc: &CharacterData, enemy: &mut Enemy) -> f32 {
    let mut dmg = attack.outgoing_damage(state, fc);
    dmg = attack.incoming_damage(dmg, fc, enemy);
    // let atk = unsafe { &(*attack.atk) };
    // println!("  dmg = {:?}: {:?}({:?})", dmg, &attack.kind, attack.idx.0);
    // println!("  dmg = {:?}: {:?}({:?}) {:?}", dmg, &atk.kind, atk.idx.0, fc.state.energy);
    // println!("{:?}\t{:?}", dmg, &atk.kind);
    dmg
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::sim1::testutil::{TestEnvironment, TestEnvironment2, TestCharacter, NoSkillTestCharacter, HoldTestCharacter};
    use crate::sim1::types::{Vision, ElementalGauge, ElementalGaugeDecay};
    use crate::sim1::fc::{FieldCharacterIndex};
    use crate::sim1::action::ICDTimers;

    use Vision::*;

    #[test]
    fn simple_setup() {
        let mut timers = ICDTimers::new();
        let cr = TestCharacter::record(Pyro);
        let mut character = TestCharacter::new(FieldCharacterIndex(0), &Pyro, &timers);
        let mut env = TestEnvironment2::character(0, &mut timers, State::new(), &cr, &mut character);

        let mut total_dmg = 0.0;
        for _ in 0..10 {
            for data in env.data.iter_mut() {
                data.state.clear();
                data.init();
            }
            total_dmg += simulate(0.2, &mut env.data, &mut env.ability, &mut env.atk_queue, &mut env.field_energy, &mut env.enemy);
        }

        // Na CD is 0.4 seconds. If 2 seconds passed in this simulation, there
        // would be 5 Na attacks. Since the first attack is a skill, there are
        // 1.8 seconds for Na attacks, so only 4 Na attacks.

        // skill na na na na
        let expect = 200.0 + 5.0 * 100.0;
        assert_eq!(total_dmg, expect);
    }

    // #[test]
    // fn ca() {
    //     let mut env = TestEnvironment::new();
    //     let mut members = vec![
    //         env.ca(FieldCharacterIndex(0), State::new(), Pyro)
    //     ];
    //     let mut enemy = TestEnvironment::enemy();
    //     let mut total_dmg = 0.0;
    //     for _ in 0..10 {
    //         total_dmg += simulate(&mut members, &mut enemy, 0.2);
    //     }
    //     // skill (na ca) na na na
    //     let expect = 200.0 + (100.0 + 150.0) + 3.0 * 100.0;
    //     assert_eq!(total_dmg, expect);
    // }

    #[test]
    fn hold() {
        let mut timers = ICDTimers::new();
        let cr = TestCharacter::record(Pyro);
        let mut character = HoldTestCharacter::new(FieldCharacterIndex(0), &Pyro, &timers);
        let mut env = TestEnvironment2::character(0, &mut timers, State::new(), &cr, &mut character);

        let mut total_dmg = 0.0;
        for _ in 0..10 {
            for data in env.data.iter_mut() {
                data.state.clear();
                data.init();
            }
            total_dmg += simulate(0.2, &mut env.data, &mut env.ability, &mut env.atk_queue, &mut env.field_energy, &mut env.enemy);
        }

        // hold na na skill na
        let expect = 250.0 + 200.0 + 5.0 * 100.0;
        assert_eq!(total_dmg, expect);
    }

    #[test]
    fn hold_and_recharge() {
        let mut timers = ICDTimers::new();
        let cr = TestCharacter::record(Pyro);
        let mut character = HoldTestCharacter::new(FieldCharacterIndex(0), &Pyro, &timers);
        let mut env = TestEnvironment2::character(0, &mut timers, State::new(), &cr, &mut character);

        let mut _total_dmg = 0.0;
        for _ in 0..10 {
            for data in env.data.iter_mut() {
                data.state.clear();
                data.init();
            }
            _total_dmg += simulate(0.2, &mut env.data, &mut env.ability, &mut env.atk_queue, &mut env.field_energy, &mut env.enemy);
        }

        // hold na na skill na
        let energy = env.data[0].state.energy;
        assert_eq!(energy, 15.0);
    }

    #[test]
    fn burst() {
        let mut timers = ICDTimers::new();
        let cr = TestCharacter::record(Pyro);
        let mut character = TestCharacter::new(FieldCharacterIndex(0), &Pyro, &timers);
        let mut env = TestEnvironment2::character(0, &mut timers, State::new(), &cr, &mut character);

        let mut total_dmg = 0.0;
        env.data[0].state.energy = env.data[0].character.energy_cost;
        for _ in 0..10 {
            for data in env.data.iter_mut() {
                data.state.clear();
                data.init();
            }
            total_dmg += simulate(0.2, &mut env.data, &mut env.ability, &mut env.atk_queue, &mut env.field_energy, &mut env.enemy);
        }

        let expect = 300.0 + 200.0 + 4.0 * 100.0;
        assert_eq!(total_dmg, expect);
    }

    #[test]
    fn burst_and_recharge() {
        let mut timers = ICDTimers::new();
        let cr = TestCharacter::record(Pyro);
        let mut character = TestCharacter::new(FieldCharacterIndex(0), &Pyro, &timers);
        let mut env = TestEnvironment2::character(0, &mut timers, State::new(), &cr, &mut character);

        let mut _total_dmg = 0.0;
        env.data[0].state.energy = env.data[0].character.energy_cost;
        for _ in 0..10 {
            for data in env.data.iter_mut() {
                data.state.clear();
                data.init();
            }
            _total_dmg += simulate(0.2, &mut env.data, &mut env.ability, &mut env.atk_queue, &mut env.field_energy, &mut env.enemy);
        }

        let energy = env.data[0].state.energy;
        // the skill was used once.
        assert_eq!(energy, 6.0);
    }

    #[test]
    fn infuse_goblet() {
        let mut timers = ICDTimers::new();
        let cr = TestCharacter::record(Pyro);
        let mut character = TestCharacter::new(FieldCharacterIndex(0), &Pyro, &timers);
        let mut env = TestEnvironment2::character(0, &mut timers, State::new().pyro_dmg(46.6), &cr, &mut character);

        let mut total_dmg = 0.0;
        for _ in 0..10 {
            for data in env.data.iter_mut() {
                data.state.clear();
                data.init();
            }
            total_dmg += simulate(0.2, &mut env.data, &mut env.ability, &mut env.atk_queue, &mut env.field_energy, &mut env.enemy);
        }

        // skill na na na na
        let expect = 200.0 * 1.466 + 5.0 * 100.0;
        assert_eq!(total_dmg, expect);
    }

    #[test]
    fn attack_infusion() {
        let mut timers = ICDTimers::new();
        let cr = TestCharacter::record(Pyro);
        let mut character = TestCharacter::new(FieldCharacterIndex(0), &Pyro, &timers);
        let mut env = TestEnvironment2::character(0, &mut timers, State::new().pyro_dmg(46.6).infusion(true), &cr, &mut character);

        let mut total_dmg = 0.0;
        for _ in 0..10 {
            for data in env.data.iter_mut() {
                data.state.clear();
                data.init();
            }
            total_dmg += simulate(0.2, &mut env.data, &mut env.ability, &mut env.atk_queue, &mut env.field_energy, &mut env.enemy);
        }

        // skill na na na na
        let expect = 1.466 * (200.0 + 5.0 * 100.0);
        let differnce = (total_dmg - expect).abs();
        assert!(differnce <= 0.001);
    }

    #[test]
    fn apply_aura() {
        let mut timers = ICDTimers::new();
        let cr = TestCharacter::record(Pyro);
        let mut character = TestCharacter::new(FieldCharacterIndex(0), &Pyro, &timers);
        let mut env = TestEnvironment2::character(0, &mut timers, State::new(), &cr, &mut character);

        let mut _total_dmg = 0.0;
        for _ in 0..10 {
            for data in env.data.iter_mut() {
                data.state.clear();
                data.init();
            }
            _total_dmg += simulate(0.2, &mut env.data, &mut env.ability, &mut env.atk_queue, &mut env.field_energy, &mut env.enemy);
        }

        // skill na na na na
        assert_eq!(env.enemy.aura.aura, Pyro);
        assert!(0.0 < env.enemy.aura.unit && env.enemy.aura.unit < 1.0);
    }

    #[test]
    fn vaporize() {
        let mut timers = ICDTimers::new();
        let cr = TestCharacter::record(Pyro);
        let mut character = TestCharacter::new(FieldCharacterIndex(0), &Pyro, &timers);
        let mut env = TestEnvironment2::character(0, &mut timers, State::new(), &cr, &mut character);

        let mut total_dmg = 0.0;
        env.enemy.aura = ElementalGauge {
            aura: Hydro,
            unit: 1.0,
            decay: ElementalGaugeDecay::A,
        };
        for _ in 0..10 {
            for data in env.data.iter_mut() {
                data.state.clear();
                data.init();
            }
            total_dmg += simulate(0.2, &mut env.data, &mut env.ability, &mut env.atk_queue, &mut env.field_energy, &mut env.enemy);
        }

        // skill na na na na
        let expect = 200.0 * 1.5 + 5.0 * 100.0;
        assert_eq!(total_dmg, expect);
        assert_eq!(env.enemy.aura.aura, Hydro);
        assert!(0.0 < env.enemy.aura.unit && env.enemy.aura.unit < 1.0);
    }

    #[test]
    fn icd_of_reaction_1_hit_counter() {
        let mut timers = ICDTimers::new();
        let cr = TestCharacter::record(Pyro);
        let mut character = NoSkillTestCharacter::new(FieldCharacterIndex(0), &Pyro, &timers);
        let mut env = TestEnvironment2::character(0, &mut timers, State::new().infusion(true), &cr, &mut character);

        let mut total_dmg = 0.0;
        env.enemy.aura = ElementalGauge {
            aura: Hydro,
            unit: 1.0,
            decay: ElementalGaugeDecay::A,
        };
        for _ in 0..10 {
            for data in env.data.iter_mut() {
                data.state.clear();
                data.init();
            }
            total_dmg += simulate(0.2, &mut env.data, &mut env.ability, &mut env.atk_queue, &mut env.field_energy, &mut env.enemy);
        }

        // vaporize na na vaporize
        let expect = 2.0 * 150.0 + 3.0 * 100.0;
        assert_eq!(total_dmg, expect);
        assert_eq!(env.enemy.aura, ElementalGauge::default());
    }

    #[test]
    fn icd_of_reaction_2_timer() {
        let mut timers = ICDTimers::new();
        let cr = TestCharacter::record(Pyro);
        let mut character = NoSkillTestCharacter::new(FieldCharacterIndex(0), &Pyro, &timers);
        let mut env = TestEnvironment2::character(0, &mut timers, State::new().infusion(true), &cr, &mut character);

        let mut total_dmg = 0.0;
        env.enemy.aura = ElementalGauge {
            aura: Hydro,
            unit: 99.0,
            decay: ElementalGaugeDecay::A,
        };
        for _ in 0..4 {
            for data in env.data.iter_mut() {
                data.state.clear();
                data.init();
            }
            total_dmg += simulate(3.0, &mut env.data, &mut env.ability, &mut env.atk_queue, &mut env.field_energy, &mut env.enemy);
        }

        // Because the current simulator needs 1 iteration for its internal
        // state, we iterate 3 times and one more.

        // vaporize vaporize vaporize
        let expect = 3.0 * 150.0;
        assert_eq!(total_dmg, expect);
        assert_eq!(env.enemy.aura.aura, Hydro);
    }

    #[test]
    fn melt() {
        let mut timers = ICDTimers::new();
        let cr = TestCharacter::record(Pyro);
        let mut character = TestCharacter::new(FieldCharacterIndex(0), &Pyro, &timers);
        let mut env = TestEnvironment2::character(0, &mut timers, State::new(), &cr, &mut character);

        let mut total_dmg = 0.0;
        env.enemy.aura = ElementalGauge {
            aura: Cryo,
            unit: 1.0,
            decay: ElementalGaugeDecay::A,
        };
        for _ in 0..10 {
            for data in env.data.iter_mut() {
                data.state.clear();
                data.init();
            }
            total_dmg += simulate(0.2, &mut env.data, &mut env.ability, &mut env.atk_queue, &mut env.field_energy, &mut env.enemy);
        }

        // skill na na na na
        let expect = 200.0 * 2.0 + 5.0 * 100.0;
        assert_eq!(total_dmg, expect);
        assert_eq!(env.enemy.aura, ElementalGauge::default());
    }

    #[test]
    fn superconduct() {
        let mut timers = ICDTimers::new();
        let cr = TestCharacter::record(Cryo);
        let mut character = TestCharacter::new(FieldCharacterIndex(0), &Cryo, &timers);
        let mut env = TestEnvironment2::character(0, &mut timers, State::new(), &cr, &mut character);

        env.enemy.aura = ElementalGauge {
            aura: Electro,
            unit: 1.0,
            decay: ElementalGaugeDecay::A,
        };

        let mut total_dmg = 0.0;
        for _ in 0..10 {
            for data in env.data.iter_mut() {
                data.state.clear();
                data.init();
            }
            total_dmg += simulate(0.2, &mut env.data, &mut env.ability, &mut env.atk_queue, &mut env.field_energy, &mut env.enemy);
        }

        // (skill superconduct) na na na na
        let expect = 200.0 + 725.36 + 5.0 * 120.0;
        assert_eq!(total_dmg, expect);
        assert_eq!(env.enemy.aura, ElementalGauge::default());
    }

    #[test]
    fn atk_spd() {
        let mut timers = ICDTimers::new();
        let cr = TestCharacter::record(Pyro);
        let mut character = TestCharacter::new(FieldCharacterIndex(0), &Pyro, &timers);
        let mut env = TestEnvironment2::character(0, &mut timers, State::new().atk_spd(50.0), &cr, &mut character);

        let mut total_dmg = 0.0;
        for _ in 0..10 {
            for data in env.data.iter_mut() {
                data.state.clear();
                data.init();
            }
            total_dmg += simulate(0.2, &mut env.data, &mut env.ability, &mut env.atk_queue, &mut env.field_energy, &mut env.enemy);
        }

        // skill na na na (na na)
        let expect = 200.0 + 7.0 * 100.0 ;
        assert_eq!(total_dmg, expect);
    }

    #[test]
    fn two_members() {
        let mut timers0 = ICDTimers::new();
        let cr0 = TestCharacter::record(Pyro);
        let mut character0 = TestCharacter::new(FieldCharacterIndex(0), &Pyro, &timers0);
        let mut env0 = TestEnvironment2::character(0, &mut timers0, State::new(), &cr0, &mut character0);

        let mut timers1 = ICDTimers::new();
        let cr1 = TestCharacter::record(Pyro);
        let mut character1 = TestCharacter::new(FieldCharacterIndex(1), &Pyro, &timers1);
        let mut env1 = TestEnvironment2::character(1, &mut timers1, State::new(), &cr1, &mut character1);

        env0.data.append(&mut env1.data);
        env0.ability.append(&mut env1.ability);

        let mut total_dmg = 0.0;
        for _ in 0..10 {
            for data in env0.data.iter_mut() {
                data.state.clear();
                data.init();
            }
            total_dmg += simulate(0.2, &mut env0.data, &mut env0.ability, &mut env0.atk_queue, &mut env0.field_energy, &mut env0.enemy);
        }

        // twice (skill na na na na)
        let expect = 2.0 * (200.0 + 3.0 * 100.0) + 100.0;
        assert_eq!(total_dmg, expect);
    }
}
