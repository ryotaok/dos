use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, CharacterAttack, AtkQueue};
use crate::sim2::types::{PeriodicStack, CharacterAction, DamageType, Vision, FieldCharacterIndex, WeaponType, FieldEnergy, Particle, VecFieldEnergy, ToNaAction};
use crate::sim2::element::{ElementalGauge, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::sim2::record::{CharacterRecord, CharacterData, Enemy};

use WeaponType::*;
use Vision::*;

// When Yanfei consumes Scarlet Seals by using a Charged Attack, each Scarlet
// Seal will increase Yanfei's Pyro DMG Bonus by 5%. This effect lasts for 6s.
// When a Charged Attack is used again during the effect's duration, it will
// dispel the previous effect.

// When Yanfei's Charged Attack deals a CRIT Hit to opponents, she will deal an
// additional instance of AoE Pyro DMG equal to 80% of her ATK. This DMG counts
// as Charged Attack DMG.
#[derive(Debug)]
pub struct Yanfei {
    scarlet_seal: u8,
    burst_time: f32,
    burst_seal: PeriodicStack,
    a1_bonus: f32,
    a1_time: f32,
}

impl Yanfei {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Yanfei").vision(Pyro).weapon(Catalyst).version(1.5)
            .base_hp(9352.0).base_atk(240.0).base_def(587.0)
            .pyro_dmg(24.0)
            .energy_cost(80.)
    }

    pub fn new() -> Self {
        Self {
            scarlet_seal: 0,
            burst_time: -99.,
            burst_seal: PeriodicStack::disable(),
            a1_bonus: 0.,
            a1_time: -99.,
        }
    }
}

impl Timeline for Yanfei {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // use ca
        if data.idx.is_on_field() && self.scarlet_seal >= 3 && state.rel_time.ca >= 1. && state.rel_time.na >= 0.5 {
            CharacterAction::Ca(state.ca_carryover(1.))
        // check if skill can be used
        } else if state.rel_time.press >= 9. {
            CharacterAction::PressSkill
        // is burst CD off and has enough energy
        } else if state.rel_time.burst >= 20. && state.energy >= 80. {
            CharacterAction::Burst
        // check if normal attacks can be used (both animations are ended)
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
            CharacterAction::Burst => self.burst_seal = PeriodicStack::new(state.current_time, 1., 15.),
            CharacterAction::PressSkill => {
                self.scarlet_seal = 3;
                field_energy.push_p(Particle::new(data.character.vision, 3.));
            },
            CharacterAction::Ca(_) => self.scarlet_seal = 0,
            CharacterAction::Na1(_) |
            CharacterAction::Na2(_) |
            CharacterAction::Na3(_) => self.scarlet_seal += 1,
            _ => (),
        }
        self.scarlet_seal += self.burst_seal.grant(state.current_time);
    }

    fn reset_timeline(&mut self) -> () {
        self.scarlet_seal = 0;
    }
}

impl CharacterAttack for Yanfei {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_burst(328.32, &PYRO_GAUGE2B, time, event, data, state);
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(305.28, &PYRO_GAUGE1A, time, event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(105.01, &PYRO_GAUGE1A, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(93.83, &PYRO_GAUGE1A, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(136.82, &PYRO_GAUGE1A, time, event, data, state);
    }

    fn ca(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.apply_ca(0.0, &PYRO_GAUGE1A, time, event, data, state);
        atk_queue.apply_ca(80., &PYRO_GAUGE1A, time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        let oneself = attack.idx == data.idx;
        self.scarlet_seal += self.burst_seal.grant(action_state.current_time);
        match action_state.to_damagetype() {
            DamageType::Burst => {
                self.burst_seal = PeriodicStack::new(action_state.current_time, 1., 15.);
                self.burst_time = action_state.current_time;
            },
            DamageType::Skill => self.scarlet_seal = 3,
            DamageType::Na => self.scarlet_seal += 1,
            DamageType::Ca => if oneself && attack.multiplier == 0.0 {
                let (multiplier, bonus) = match self.scarlet_seal {
                    1 => (188.22, 5.),
                    2 => (216.46, 10.),
                    3 => (244.69, 15.),
                    4 => (272.92, 20.),
                    _ => (159.99, 0.),
                };
                attack.multiplier = multiplier;
                self.a1_bonus = bonus;
                self.a1_time = attack.time;
            },
            _ => (),
        }

        if oneself {
            if attack.time - self.burst_time <= 15. {
                state.ca_dmg += 54.4;
            }
            if attack.time - self.a1_time <= 8. {
                state.pyro_dmg += self.a1_bonus;
            }
        }
    }

    fn reset_modify(&mut self) -> () {
        self.scarlet_seal = 0;
        self.burst_seal = PeriodicStack::disable();
        self.burst_time = -99.;
        self.a1_bonus = 0.;
        self.a1_time = -99.;
    }
}

// If 2 stacks of Grimheart are consumed upon unleashing the Holding Mode of
// Icetide Vortex, a Shattered Lightfall Sword will be created that will explode
// immediately, dealing 50% of the basic Physical DMG dealt by a Lightfall Sword
// created by Glacial Illumination.

// When Glacial Illumination is cast, the CD of Icetide Vortex is reset and Eula
// gains 1 stack of Grimheart.
#[derive(Debug)]
pub struct Eula {
    grimheart: u8,
    burst_time: f32,
    lightfall_sword_stack: f32,
    apply_debuff: bool,
    debuff_time: f32,
}

impl Eula {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Eula").vision(Cryo).weapon(Claymore).version(1.5)
            .base_hp(13226.0).base_atk(342.0).base_def(751.0)
            .cd(88.4)
            .energy_cost(80.)
    }

    pub fn new() -> Self {
        Self {
            grimheart: 0,
            burst_time: -99.,
            lightfall_sword_stack: 0.,
            apply_debuff: false,
            debuff_time: -99.,
        }
    }
}

impl Timeline for Eula {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        if self.grimheart == 2 && state.rel_time.hold >= 10. {
            CharacterAction::HoldSkill
        } else if state.rel_time.hold >= 10. && state.rel_time.press >= 4. {
            CharacterAction::PressSkill
        // is burst CD off and has enough energy
        } else if state.rel_time.burst >= 20. && state.energy >= 80. {
            CharacterAction::Burst
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.77 {
            // 5 attacks in 3.85 seconds
            data.na_idx.to_na(5, state.na_carryover(0.77))
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        match event {
            CharacterAction::Burst => {
                self.grimheart += 1;
                state.reduce_skill = 99.;
            },
            CharacterAction::PressSkill => {
                self.grimheart += 1;
                field_energy.push_p(Particle::new(data.character.vision, 1.5));
            },
            CharacterAction::PressSkill => {
                self.grimheart = 0;
                field_energy.push_p(Particle::new(data.character.vision, 2.5));
            },
            _ => (),
        }
    }

    fn reset_timeline(&mut self) -> () {
        self.grimheart = 0;
    }
}

impl CharacterAttack for Eula {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.grimheart += 1;
        atk_queue.add_burst(617.44, &CRYO_GAUGE2B, time, event, data, state);
        // burst can be used if on field
        // lightfall_sword
        let t = if data.idx.0 == 0 {
            time + 7.
        } else {
            time
        };
        atk_queue.add_burst(725.56, &PHYSICAL_GAUGE, t, event, data, state);
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.grimheart += 1;
        atk_queue.add_skill(263.52, &CRYO_GAUGE1A, time, event, data, state);
    }

    fn hold(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(442.08, &CRYO_GAUGE1A, time, event, data, state);
        for i in 1..(self.grimheart + 1) {
            atk_queue.add_skill(172.8, &CRYO_GAUGE1A, time + 0.3333 * i as f32, event, data, state);
        }
        if self.grimheart == 2 {
            atk_queue.add_skill(362.78, &PHYSICAL_GAUGE, time + 1., event, data, state);
        }
        self.grimheart = 0;
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(177.38, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(184.93, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(112.28, &PHYSICAL_GAUGE, time, event, data, state);
        atk_queue.add_na(112.28, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(222.67, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na5(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(142.0, &PHYSICAL_GAUGE, time, event, data, state);
        atk_queue.add_na(142.0, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn reset_attack(&mut self) -> () {
        self.grimheart = 0;
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_burst() {
            self.burst_time = action_state.current_time;
            self.lightfall_sword_stack = 0.;
        }

        if attack.idx == data.idx {
            if attack.time - self.burst_time < 7. {
                self.lightfall_sword_stack += 1.;
            }
            // burst can be used if on field
            // lightfall_sword
            if data.idx.0 == 0 && attack.kind == DamageType::Burst && attack.multiplier == 725.56 {
                attack.multiplier += 148.24 * self.lightfall_sword_stack;
            }
            if !self.apply_debuff && action_state.did_hold() {
                self.apply_debuff = true;
                self.debuff_time = action_state.current_time;
                enemy.debuff.cryo += 25.;
                enemy.debuff.physical += 25.;
            }
        }
        if self.apply_debuff && attack.time - self.debuff_time > 7.5 {
            self.apply_debuff = false;
            enemy.debuff.cryo -= 25.;
            enemy.debuff.physical -= 25.;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.grimheart = 0;
        self.burst_time = -99.;
        self.apply_debuff = false;
        self.debuff_time = -99.;
    }
}
