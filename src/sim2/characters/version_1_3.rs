use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, CharacterAttack, AtkQueue};
use crate::sim2::types::{CharacterAction, DamageType, Vision, FieldCharacterIndex, WeaponType, FieldEnergy, Particle, VecFieldEnergy, ToNaAction};
use crate::sim2::element::{ElementalGauge, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::sim2::record::{CharacterRecord, CharacterData, Enemy};

use WeaponType::*;
use Vision::*;

// While under the effects of Bane of All Evil, all DMG dealt by Xiao increases
// by 5%. DMG increases by a further 5% for every 3s the ability persists. The
// maximum DMG Bonus is 25%.

// Using Lemniscatic Wind Cycling increases the DMG of subsequent uses of
// Lemniscatic Wind Cycling by 15%. This effect lasts for 7s, and has a maximum
// of 3 stacks. Gaining a new stack refreshes the effect's duration.
#[derive(Debug)]
pub struct Xiao {
    charge: u8,
    burst_time: f32,
    skill_time: f32,
    skill_stack: f32,
}

impl Xiao {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Xiao").vision(Anemo).weapon(Polearm).version(1.3)
            .base_hp(12736.0).base_atk(349.0).base_def(799.0)
            .cr(24.2)
            .energy_cost(70.)
    }

    pub fn new() -> Self {
        Self {
            charge: 0,
            burst_time: -99.,
            skill_time: -99.,
            skill_stack: 0.,
        }
    }

    fn infusion(&self, time: f32) -> &'static ElementalGauge {
        if time - self.burst_time <= 15. {
            &ANEMO_GAUGE1A
        } else {
            &PHYSICAL_GAUGE
        }
    }
}

impl Timeline for Xiao {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        let during_burst = state.current_time - self.burst_time <= 18.;
        // is burst CD off and has enough energy
        if state.rel_time.burst >= 18. && state.energy >= 70. {
            self.burst_time = state.current_time;
            CharacterAction::Burst
        // check if skill can be used
        } else if state.rel_time.press >= 10. {
            self.charge = 1;
            CharacterAction::PressSkill
        } else if self.charge < 2 {
            self.charge += 1;
            CharacterAction::PressSkill
        // check if normal attacks can be used (both animations are ended)
        } else if during_burst && state.rel_time.ca >= 1.7 {
            CharacterAction::Ca(state.ca_carryover(1.7))
        } else if !during_burst && state.rel_time.na >= 0.625 {
            // 6 attacks in 3.75 seconds
            data.na_idx.to_na(6, state.na_carryover(0.625))
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        match event {
            CharacterAction::PressSkill => field_energy.push_p(Particle::new(data.character.vision, 3.)),
            _ => (),
        }
    }

    fn reset_timeline(&mut self) -> () {
        self.burst_time = -99.;
        self.charge = 0;
    }
}

impl CharacterAttack for Xiao {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.burst_time = time;
        atk_queue.add_burst(0.0, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(455.04, &ANEMO_GAUGE2B, time, event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(49.14, self.infusion(time), time, event, data, state);
        atk_queue.add_na(49.14, self.infusion(time), time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(101.58, self.infusion(time), time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(122.3, self.infusion(time), time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(67.2, self.infusion(time), time, event, data, state);
        atk_queue.add_na(67.2, self.infusion(time), time, event, data, state);
    }

    fn na5(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(127.64, self.infusion(time), time, event, data, state);
    }

    fn na6(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(170.97, self.infusion(time), time, event, data, state);
    }

    fn ca(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_ca(404.02, self.infusion(time), time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx {
            match action_state.to_damagetype() {
                DamageType::Burst => self.burst_time = action_state.current_time,
                DamageType::Skill => {
                    self.skill_time = action_state.current_time;
                    self.skill_stack += 1.;
                },
                _ => (),
            }
            let burst_duration = attack.time - self.burst_time;
            if burst_duration <= 18. {
                state.na_dmg += 95.2;
                state.ca_dmg += 95.2;
            }
            state.all_dmg += if burst_duration < 3. {
                5.
            } else if burst_duration < 6. {
                10.
            } else if burst_duration < 9. {
                15.
            } else if burst_duration < 12. {
                20.
            } else if burst_duration < 15. {
                25.
            } else {
                0.
            };
            if attack.time - self.skill_time > 7. {
                self.skill_stack = 0.;
            }
            state.skill_dmg += 15. * self.skill_stack;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.burst_time = -99.;
        self.skill_time = -99.;
        self.skill_stack = 0.;
    }
}

// When a Paramita Papilio state activated by Guide to Afterlife ends, all
// allies in the party (excluding Hu Tao herself) will have their CRIT Rate
// increased by 12% for 8s.

// When Hu Tao's HP is equal to or less than 50%, her Pyro DMG Bonus is
// increased by 33%.
#[derive(Debug)]
pub struct HuTao {
    skill_time: f32,
}

impl HuTao {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Hu Tao").vision(Pyro).weapon(Polearm).version(1.3)
            .base_hp(15552.0).base_atk(106.0).base_def(876.0)
            .cd(88.4)
            // a4
            .pyro_dmg(33.0)
            .energy_cost(60.)
    }

    pub fn new() -> Self {
        Self {
            skill_time: -99.
        }
    }

    fn infusion(&self, time: f32) -> &'static ElementalGauge {
        if time - self.skill_time <= 9. {
            &PYRO_GAUGE1A
        } else {
            &PHYSICAL_GAUGE
        }
    }
}

impl Timeline for HuTao {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        let during_skill = state.current_time - self.skill_time <= 9.;
        // is burst CD off and has enough energy
        if state.rel_time.burst >= 15. && state.energy >= 60. {
            CharacterAction::Burst
        // check if skill can be used
        } else if state.rel_time.press >= 16. {
            CharacterAction::PressSkill
        // check if normal attacks can be used (both animations are ended)
        } else if during_skill && state.rel_time.ca >= 0.935 {
            CharacterAction::Ca(state.ca_carryover(0.935))
        } else if !during_skill && state.rel_time.na >= 0.4875 {
            // 6 attacks in 2.925 seconds
            data.na_idx.to_na(6, state.na_carryover(0.4875))
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        match event {
            CharacterAction::PressSkill => {
                self.skill_time = state.current_time;
                field_energy.push_p(Particle::new(data.character.vision, 3.));
            },
            _ => (),
        }
    }

    fn reset_timeline(&mut self) -> () {
        self.skill_time = -99.;
    }
}

impl CharacterAttack for HuTao {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_burst(617.44, &PYRO_GAUGE2B, time, event, data, state);
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.skill_time = time;
        atk_queue.add_skill(0., &PHYSICAL_GAUGE, time, event, data, state);
        atk_queue.add_skill(115.2, &PYRO_GAUGE1A, time+4., event, data, state);
        atk_queue.add_skill(115.2, &PYRO_GAUGE1A, time+8., event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(83.65, self.infusion(time), time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(86.09, self.infusion(time), time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(108.92, self.infusion(time), time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(117.11, self.infusion(time), time, event, data, state);
    }

    fn na5(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(59.36, self.infusion(time), time, event, data, state);
        atk_queue.add_na(62.8, self.infusion(time), time, event, data, state);
    }

    fn na6(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(153.36, self.infusion(time), time, event, data, state);
    }

    fn ca(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(83.65, self.infusion(time), time, event, data, state);
        atk_queue.add_ca(242.57, self.infusion(time), time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_skill() {
            self.skill_time = action_state.current_time;
        }
        if attack.idx == data.idx && attack.time - self.skill_time <= 9. {
            state.flat_atk += 0.0626 * state.HP();
        }
        if attack.idx != data.idx && attack.time - (self.skill_time + 9.) <= 8. {
            state.cr += 12.;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.skill_time = -99.;
    }
}
