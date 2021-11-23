use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, CharacterAttack, AtkQueue};
use crate::sim2::types::{CharacterAction, DamageType, Vision, FieldCharacterIndex, WeaponType, FieldEnergy, Particle, VecFieldEnergy, ToNaAction};
use crate::sim2::element::{ElementalGauge, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::sim2::record::{CharacterRecord, CharacterData, Enemy};

use WeaponType::*;
use Vision::*;

// After using Kamisato Art: Hyouka, Kamisato Ayaka's Normal and Charged attacks
// deal 30% increased DMG for 6s.

// When the Cryo application at the end of Kamisato Art: Senho hits an opponent,
// Kamisato Ayaka gains the following effects:
// - Restores 10 Stamina
// - Gains 18% Cryo DMG Bonus for 10s.
#[derive(Debug)]
pub struct Ayaka {
    skill_time: f32,
}

impl Ayaka {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Ayaka").vision(Cryo).weapon(Sword).version(2.0)
            .base_hp(12858.0).base_atk(342.0).base_def(784.0)
            .cd(88.4)
            .energy_cost(80.)
    }

    pub fn new() -> Self {
        Self {
            skill_time: -99.
        }
    }
}

impl Timeline for Ayaka {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // check if skill can be used
        if state.rel_time.press >= 10. {
            CharacterAction::PressSkill
        // is burst CD off and has enough energy
        } else if state.rel_time.burst >= 20. && state.energy >= 80. {
            CharacterAction::Burst
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.4234 {
            // 5 attacks in 2.117 seconds
            data.na_idx.to_na(5, state.na_carryover(0.4234))
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        match event {
            CharacterAction::PressSkill => field_energy.push_p(Particle::new(data.character.vision, 4.5)),
            _ => (),
        }
    }
}

impl CharacterAttack for Ayaka {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        for i in 0..19 {
            atk_queue.add_burst(202.14, &CRYO_GAUGE1A, time + 0.3333 * i as f32, event, data, state);
        }
        atk_queue.add_burst(303.21, &CRYO_GAUGE1A, time + 5., event, data, state);
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(430.56, &CRYO_GAUGE2B, time, event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(90.39, &CRYO_GAUGE1A, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(96.24, &CRYO_GAUGE1A, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(123.79, &CRYO_GAUGE1A, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(44.77, &CRYO_GAUGE1A, time, event, data, state);
        atk_queue.add_na(44.77, &CRYO_GAUGE1A, time, event, data, state);
        atk_queue.add_na(44.77, &CRYO_GAUGE1A, time, event, data, state);
    }

    fn na5(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(154.55, &CRYO_GAUGE1A, time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_skill() {
            self.skill_time = action_state.current_time;
        }
        if attack.idx == data.idx {
            state.cryo_dmg += 18.;
            if attack.time - self.skill_time <= 6. {
                state.na_dmg += 30.;
                state.ca_dmg += 30.;
            }
        }
    }

    fn reset_modify(&mut self) -> () {
        self.skill_time = -99.;
    }
}

// During Niwabi Fire-Dance, shots from Yoimiya's Normal Attack will increase
// her Pyro DMG Bonus by 2% on hit. This effect lasts for 3s and can have a
// maximum of 10 stacks.

// Using Ryuukin Saxifrage causes nearby party members (not including Yoimiya)
// to gain a 10% ATK increase for 15s. Additionally, a further ATK Bonus will be
// added on based on the number of "Tricks of the Trouble-Maker" stacks Yoimiya
// possesses when using Ryuukin Saxifrage. Each stack increases this ATK Bonus
// by 1%.
#[derive(Debug)]
pub struct Yoimiya {
    skill_time: f32,
    a1_time: f32,
    a1_stack: f32,
    a4_time: f32,
    a4_bonus: f32,
}

impl Yoimiya {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Yoimiya").vision(Pyro).weapon(Bow).version(2.0)
            .base_hp(10164.0).base_atk(323.0).base_def(615.0)
            .cr(24.2)
            .energy_cost(60.)
    }

    pub fn new() -> Self {
        Self {
            skill_time: -99.,
            a1_time: -99.,
            a1_stack: 0.,
            a4_time: -99.,
            a4_bonus: 0.,
        }
    }

    fn infusion(&self, time: f32, is_on_field: bool) -> &'static ElementalGauge {
        if is_on_field && time - self.skill_time <= 10. {
            &PYRO_GAUGE1A
        } else {
            &PHYSICAL_GAUGE
        }
    }
}

impl Timeline for Yoimiya {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // is burst CD off and has enough energy
        if state.rel_time.burst >= 15. && state.energy >= 60. {
            CharacterAction::Burst
        // check if skill can be used
        } else if state.rel_time.press >= 18. {
            CharacterAction::PressSkill
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.42 {
            // 5 attacks in 2.1 seconds
            data.na_idx.to_na(5, state.na_carryover(0.42))
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

impl CharacterAttack for Yoimiya {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_burst(228.96, &PYRO_GAUGE2B, time, event, data, state);
        for i in 1..6 {
            atk_queue.add_burst(219.6, &PYRO_GAUGE1A, time + (2 * i) as f32, event, data, state);
        }
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.skill_time = time;
        atk_queue.add_skill(0.0, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(63.59, self.infusion(time, data.idx.is_on_field()), time, event, data, state);
        atk_queue.add_na(63.59, self.infusion(time, data.idx.is_on_field()), time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(121.99, self.infusion(time, data.idx.is_on_field()), time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(158.59, self.infusion(time, data.idx.is_on_field()), time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(82.82, self.infusion(time, data.idx.is_on_field()), time, event, data, state);
        atk_queue.add_na(82.82, self.infusion(time, data.idx.is_on_field()), time, event, data, state);
    }

    fn na5(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(188.87, self.infusion(time, data.idx.is_on_field()), time, event, data, state);
    }

    fn reset_attack(&mut self) -> () {
        self.skill_time = -99.;
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_burst() {
            self.a4_time = action_state.current_time;
            self.a4_bonus = 10. + 1. * self.a1_stack;
        }
        if action_state.did_skill() {
            self.skill_time = action_state.current_time;
        }
        if attack.idx == data.idx {
            if data.idx.is_on_field() && attack.time - self.skill_time <= 10. {
                state.na_talent += 61.74;
                if attack.kind == DamageType::Na {
                    self.a1_time = attack.time;
                    self.a1_stack += 1.;
                }
            }
            if attack.time - self.a1_time <= 3. {
                state.pyro_dmg += 2. * self.a1_stack;
            } else {
                self.a1_stack = 0.;
            }
        } else {
            if attack.time - self.a4_time <= 15. {
                state.atk += self.a4_bonus;
            }
        }
    }

    fn reset_modify(&mut self) -> () {
        self.skill_time = -99.;
        self.a1_time = -99.;
        self.a1_stack = 0.;
        self.a4_time = -99.;
        self.a4_bonus = 0.;
    }
}

// When Sayu triggers a Swirl reaction while active, she heals all your
// characters and nearby allies for 300 HP. She will also heal an additional 1.2
// HP for every point of Elemental Mastery she has. This effect can be triggered
// once every 2s.

// The Muji-Muji Daruma created by Yoohoo Art: Mujina Flurry gains the following
// effects:
// - When healing a character, it will also heal characters near that healed
//   character for 20% the amount of HP.
// - Increases the AoE of its attack against opponents.
#[derive(Debug)]
pub struct Sayu {}

impl Sayu {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Sayu").vision(Anemo).weapon(Claymore).version(2.0)
            .base_hp(11854.0).base_atk(244.0).base_def(745.0)
            .em(96.0)
            .energy_cost(80.)
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Timeline for Sayu {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // check if skill can be used
        if state.rel_time.press >= 6. {
            CharacterAction::PressSkill
        // is burst CD off and has enough energy
        } else if state.rel_time.burst >= 20. && state.energy >= 80. {
            CharacterAction::Burst
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.654 {
            // 4 attacks in 2.616 seconds
            data.na_idx.to_na(4, state.na_carryover(0.654))
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        match event {
            CharacterAction::PressSkill => field_energy.push_p(Particle::new(data.character.vision, 2.)),
            _ => (),
        }
    }
}

impl CharacterAttack for Sayu {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_burst(210.24, &ANEMO_GAUGE1A, time, event, data, state);
        for i in 1..7 {
            atk_queue.add_burst(93.6, &ANEMO_GAUGE1A, time + (2 * i) as f32, event, data, state);
        }
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(64.8, &ANEMO_GAUGE1A, time, event, data, state);
        atk_queue.add_skill(285.12, &ANEMO_GAUGE1A, time, event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(142.8, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(141.1, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(85.85, &PHYSICAL_GAUGE, time, event, data, state);
        atk_queue.add_na(85.85, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(193.97, &PHYSICAL_GAUGE, time, event, data, state);
    }

    // fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
    // }

    // fn reset_modify(&mut self) -> () {
    // }
}

// When another nearby character in the party obtains an Abundance Amulet
// created by Lightning Blade, Lightning Blade's CD is decreased by 1.5s.

// Increases the Energy Recharge effect granted by Lightning Blade's Abundance
// Amulet by 10% of the Traveler's Energy Recharge.
#[derive(Debug)]
pub struct TravelerElectro {}

impl TravelerElectro {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Traveler (Electro)").vision(Electro).weapon(Sword).version(1.0)
            .base_hp(10875.0).base_atk(212.0).base_def(683.0)
            .atk(24.0)
            .energy_cost(80.)
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Timeline for TravelerElectro {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // check if skill can be used (a1)
        if state.rel_time.press >= 10.5 {
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
            CharacterAction::PressSkill => {
                field_energy.push_p(Particle::new(data.character.vision, 3.5));
                // Abundance Amulets
                let bonus = 1. + 0.1 * state.er / 100.;
                field_energy.push_e(8. * bonus);
            },
            CharacterAction::Burst => {
                // the thunder hits 20 times
                field_energy.push_e(20.);
            },
            _ => (),
        }
    }
}

impl CharacterAttack for TravelerElectro {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_burst(205.92, &ELECTRO_GAUGE2B, time, event, data, state);
        for i in 1..21 {
            atk_queue.add_burst(59.04, &ELECTRO_GAUGE1A, time + 0.5 * i as f32, event, data, state);
        }
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(141.6, &ELECTRO_GAUGE1A, time, event, data, state);
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
    }

    // fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
    // }

    // fn reset_modify(&mut self) -> () {
    // }
}
