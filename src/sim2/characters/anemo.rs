use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, CharacterAttack, AtkQueue};
use crate::sim2::types::{CharacterAction, DamageType, Vision, FieldCharacterIndex, WeaponType, FieldEnergy, Particle, VecFieldEnergy, ToNaAction};
use crate::sim2::element::{ElementalGauge, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, ELECTRO_GAUGE4C, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::sim2::record::{CharacterRecord, CharacterData, Enemy};

use WeaponType::*;
use Vision::*;

// version 1.0

// When Sucrose triggers a Swirl effect, all characters in the party with the
// matching element (excluding Sucrose) have their Elemental Mastery increased
// by 50 for 8s.

// When Astable Anemohypostasis Creation - 6308 or Forbidden Creation - Isomer
// 75 / Type II hits an opponent, increases all party members' (excluding
// Sucrose) Elemental Mastery based on 20% of Sucrose's Elemental Mastery for
// 8s.
#[derive(Debug)]
pub struct Sucrose {
    a1_time: f32,
    a4_time: f32,
    em: f32,
}

impl Sucrose {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Sucrose").vision(Anemo).weapon(Catalyst).version(1.0)
            .base_hp(9244.0).base_atk(170.0).base_def(703.0)
            .anemo_dmg(24.0)
            .energy_cost(80.)
    }

    pub fn new() -> Self {
        Self {
            a1_time: -99.,
            a4_time: -99.,
            em: 0.,
        }
    }
}

impl Timeline for Sucrose {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // check if skill can be used
        if state.rel_time.press >= 15. {
            CharacterAction::PressSkill
        // is burst CD off and has enough energy
        } else if state.rel_time.burst >= 20. && state.energy >= 80. {
            CharacterAction::Burst
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.375 {
            // 4 attacks in 1.5 seconds
            data.na_idx.to_na(4, state.na_carryover(0.375))
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

impl CharacterAttack for Sucrose {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        let absorbed = enemy.absorb_element();
        for i in 0..3 {
            let t = time + (2 * i) as f32;
            if absorbed.aura != Physical {
                atk_queue.apply_burst(79.2, absorbed, t, event, data, state);
            }
            atk_queue.apply_burst(266.4, &ANEMO_GAUGE1A, t, event, data, state);
        }
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(380.16, &ANEMO_GAUGE1A, time, event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(60.24, &ANEMO_GAUGE1A, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(55.11, &ANEMO_GAUGE1A, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(69.21, &ANEMO_GAUGE1A, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(86.25, &ANEMO_GAUGE1A, time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx {
            if enemy.trigger_er(&attack.element.aura).is_swirl() {
                self.a1_time = attack.time;
            }
            match &attack.kind {
                DamageType::Skill |
                DamageType::Burst => {
                    self.em = state.em;
                    self.a4_time = attack.time;
                },
                _ => (),
            }
        }
        if attack.idx != data.idx {
            if attack.time - self.a1_time <= 8. {
                state.em += 50.;
            }
            if attack.time - self.a4_time <= 8. {
                state.em += 0.2 * self.em;
            }
        }
    }

    fn reset_modify(&mut self) -> () {
        self.a1_time = -99.;
        self.a4_time = -99.;
        self.em = 0.;
    }
}

// The last hit of a Normal Attack combo unleashes a wind blade, dealing 60% of
// ATK as Anemo DMG to all opponents in its path.

// Palm Vortex kills regenerate 2% HP for 5s. This effect can only occur once
// every 5s.
#[derive(Debug)]
pub struct TravelerAnemo {}

impl TravelerAnemo {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Traveler (Anemo)").vision(Anemo).weapon(Sword).version(1.0)
            .base_hp(10875.0).base_atk(212.0).base_def(683.0)
            .atk(24.0)
            .energy_cost(60.)
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Timeline for TravelerAnemo {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // check if skill can be used
        if state.rel_time.hold >= 8. {
            CharacterAction::HoldSkill
        // is burst CD off and has enough energy
        } else if state.rel_time.burst >= 15. && state.energy >= 60. {
            CharacterAction::Burst
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.51 && state.rel_time.hold >= 2.5 {
            // 5 attacks in 2.555 seconds
            data.na_idx.to_na(5, state.na_carryover(0.51))
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        match event {
            CharacterAction::HoldSkill => {
                field_energy.push_p(Particle::new(data.character.vision, 4.));
                state.rel_time.na -= 2.;
            },
            _ => (),
        }
    }
}

impl CharacterAttack for TravelerAnemo {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        let absorbed = enemy.absorb_element();
        for i in 0..10 {
            let t = time + i as f32;
            if absorbed.aura != Physical {
                atk_queue.apply_burst(44.64, absorbed, t, event, data, state);
            }
            atk_queue.apply_burst(145.44, &ANEMO_GAUGE1A, t, event, data, state);
        }
    }

    fn hold(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        let absorbed = enemy.absorb_element();
        for i in 0..6 {
            let t = time + 0.35 * i as f32;
            if absorbed.aura != Physical {
                atk_queue.apply_skill(31.92, absorbed, t, event, data, state);
            }
            atk_queue.apply_skill(31.92, &ANEMO_GAUGE1A, t, event, data, state);
        }
        atk_queue.apply_skill(364.8, &ANEMO_GAUGE1A, time + 2.5, event, data, state);
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
        atk_queue.add_na(60., &ANEMO_GAUGE1A, time, event, data, state);
    }

   // fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
   // }

    // fn reset_modify(&mut self) -> () {
    // }
}

// Hits by Jean's Normal Attacks have a 50% chance to regenerate HP equal to 15%
// of Jean's ATK for all party members.

// Using Dandelion Breeze will regenerate 20% of its Energy.
#[derive(Debug)]
pub struct Jean {}

impl Jean {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Jean").vision(Anemo).weapon(Sword).version(1.0)
            .base_hp(14695.0).base_atk(239.0).base_def(769.0)
            .energy_cost(80.)
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Timeline for Jean {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // check if skill can be used
        if state.rel_time.press >= 6. {
            CharacterAction::PressSkill
        // is burst CD off and has enough energy
        } else if state.rel_time.burst >= 20. && state.energy >= 80. {
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
            // a4
            CharacterAction::Burst => state.energy += 16.,
            CharacterAction::PressSkill => field_energy.push_p(Particle::new(data.character.vision, 3.)),
            _ => (),
        }
    }
}

impl CharacterAttack for Jean {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.apply_burst(764.64, &ANEMO_GAUGE2B, time, event, data, state);
        for i in 1..4 {
            atk_queue.apply_burst(141.12, &ANEMO_GAUGE1A, time + (2 * i) as f32, event, data, state);
        }
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(525.6, &ANEMO_GAUGE2B, time, event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(95.54, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(90.1, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(119.17, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(130.22, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na5(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(156.57, &PHYSICAL_GAUGE, time, event, data, state);
    }

   // fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
   // }

    // fn reset_modify(&mut self) -> () {
    // }
}

// Holding Skyward Sonnet creates an upcurrent that lasts for 20s.

// Regenerates 15 Energy for Venti after the effects of Wind's Grand Ode end. If
// an Elemental Absorption occurred, this also restores 15 Energy to all
// characters of that corresponding element.
#[derive(Debug)]
pub struct Venti {}

impl Venti {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Venti").vision(Anemo).weapon(Bow).version(1.0)
            .base_hp(10531.0).base_atk(263.0).base_def(669.0)
            .er(32.0)
            .energy_cost(60.)
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Timeline for Venti {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // check if skill can be used
        if state.rel_time.press >= 6. {
            CharacterAction::PressSkill
        // is burst CD off and has enough energy
        } else if state.rel_time.burst >= 15. && state.energy >= 60. {
            CharacterAction::Burst
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.475 {
            // 6 attacks in 2.85 seconds
            data.na_idx.to_na(6, state.na_carryover(0.475))
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        match event {
            CharacterAction::Burst => field_energy.push_e(15.),
            CharacterAction::PressSkill => field_energy.push_p(Particle::new(data.character.vision, 3.)),
            _ => (),
        }
    }
}

impl CharacterAttack for Venti {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        let absorbed = enemy.absorb_element();
        for i in 0..20 {
            let t = time + 0.25 * i as f32;
            if absorbed.aura != Physical {
                atk_queue.apply_burst(33.84, absorbed, t, event, data, state);
            }
            atk_queue.apply_burst(67.68, &ANEMO_GAUGE1A, t, event, data, state);
        }
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(496.8, &ANEMO_GAUGE2B, time, event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(40.29, &PHYSICAL_GAUGE, time, event, data, state);
        atk_queue.add_na(40.29, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(87.72, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(103.53, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(51.51, &PHYSICAL_GAUGE, time, event, data, state);
        atk_queue.add_na(51.51, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na5(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(100.13, &ANEMO_GAUGE1A, time, event, data, state);
    }

    fn na6(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(140.25, &ANEMO_GAUGE1A, time, event, data, state);
    }

   // fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
   // }

    // fn reset_modify(&mut self) -> () {
    // }
}
