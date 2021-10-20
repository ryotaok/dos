use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, CharacterAttack, AtkQueue};
use crate::sim2::types::{CharacterAction, DamageType, Vision, FieldCharacterIndex, WeaponType, FieldEnergy, Particle, VecFieldEnergy, ToNaAction};
use crate::sim2::element::{ElementalGauge, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::sim2::record::{CharacterRecord, CharacterData, Enemy};

use WeaponType::*;
use Vision::*;

// If Chihayaburu comes into contact with Hydro/Pyro/Cryo/Electro when cast,
// Chihayaburu will absorb that element and if Plunging Attack: Midare Ranzan is
// used before the effect expires, it will deal an additional 200% ATK of the
// absorbed elemental type as DMG. This will be considered Plunging Attack DMG.
// Elemental Absorption may only occur once per use of Chihayaburu.

// Upon triggering a Swirl reaction, Kaedehara Kazuha will grant all party
// members a 0.04% Elemental DMG Bonus to the element absorbed by Swirl for
// every point of Elemental Mastery he has for 8s. Bonuses for different
// elements obtained through this method can co-exist.
#[derive(Debug)]
pub struct Kazuha {
    a4_time: f32,
    a4_bonus: f32,
    midare_ranzan: bool,
}

impl Kazuha {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Kazuha").vision(Anemo).weapon(Sword).version(1.6)
            .base_hp(13348.0).base_atk(297.0).base_def(807.0)
            .em(115.2)
            .energy_cost(60.)
    }

    pub fn new() -> Self {
        Self {
            a4_time: -99.,
            a4_bonus: 0.,
            midare_ranzan: false,
        }
    }
}

impl Timeline for Kazuha {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // is burst CD off and has enough energy
        if state.rel_time.burst >= 15. && state.energy >= 60. {
            CharacterAction::Burst
        } else if self.midare_ranzan {
            CharacterAction::Ca(0.)
        // check if skill can be used
        } else if state.rel_time.hold >= 9. {
            CharacterAction::HoldSkill
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.4332 {
            // 5 attacks in 2.166 seconds
            data.na_idx.to_na(5, state.na_carryover(0.4332))
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        match event {
            CharacterAction::Ca(_) => self.midare_ranzan = false,
            CharacterAction::HoldSkill => {
                self.midare_ranzan = true;
                field_energy.push_p(Particle::new(data.character.vision, 4.));
            },
            _ => (),
        }
    }

    fn reset_timeline(&mut self) -> () {
        self.midare_ranzan = false;
    }
}

impl CharacterAttack for Kazuha {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        let absorbed = enemy.absorb_element();
        atk_queue.apply_burst(419.04, &ANEMO_GAUGE2B, time, event, data, state);
        for i in 1..5 {
            let t = time + (2 * i) as f32;
            if absorbed.aura != Physical {
                atk_queue.apply_burst(64.8, absorbed, t, event, data, state);
            }
            atk_queue.apply_burst(216.0, &ANEMO_GAUGE1A, t, event, data, state);
        }
    }

    fn hold(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(469.44, &ANEMO_GAUGE1A, time, event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(88.91, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(89.42, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(51.0, &PHYSICAL_GAUGE, time, event, data, state);
        atk_queue.add_na(61.2, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(120.02, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na5(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(50.15, &PHYSICAL_GAUGE, time, event, data, state);
        atk_queue.add_na(50.15, &PHYSICAL_GAUGE, time, event, data, state);
        atk_queue.add_na(50.15, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn ca(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        let absorbed = enemy.absorb_element();
        if absorbed.aura != Physical {
            atk_queue.apply_ca(200., absorbed, time, event, data, state);
        }
        atk_queue.apply_ca(404.02, &ANEMO_GAUGE1A, time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx && enemy.trigger_er(&attack.element.aura).is_swirl() {
            self.a4_time = attack.time;
            self.a4_bonus = 0.04 * state.em;
        }
        if attack.time - self.a4_time <= 8. {
            state.pyro_dmg += self.a4_bonus;
            state.hydro_dmg += self.a4_bonus;
            state.electro_dmg += self.a4_bonus;
            state.cryo_dmg += self.a4_bonus;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.a4_time = -99.;
        self.a4_bonus = 0.;
    }
}
