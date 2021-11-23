use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, CharacterAttack, AtkQueue};
use crate::sim2::types::{CharacterAction, DamageType, Vision, FieldCharacterIndex, WeaponType, FieldEnergy, Particle, VecFieldEnergy, ToNaAction};
use crate::sim2::element::{ElementalGauge, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::sim2::record::{CharacterRecord, CharacterData, Enemy};
// use crate::sim2::characters::CharacterName;

use WeaponType::*;
use Vision::*;

// version 1.0

// Increases the CRIT Rate of Fiery Rain by 10% and widens its AoE by 30%.

// Aimed Shot hits on weak spots increase ATK by 15% for 10s.
#[derive(Debug)]
pub struct Amber {
    ca_time: f32,
}

impl Amber {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Amber").vision(Pyro).weapon(Bow).version(1.0)
            .base_hp(9461.0).base_atk(223.0).base_def(601.0)
            .atk(24.0)
            .energy_cost(40.)
    }

    pub fn new() -> Self {
        Self {
            ca_time: -99.
        }
    }
}

impl Timeline for Amber {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // check if skill can be used
        if state.rel_time.press >= 15. {
            CharacterAction::PressSkill
        // is burst CD off and has enough energy
        } else if state.rel_time.burst >= 12. && state.energy >= 40. {
            CharacterAction::Burst
        // check if normal attacks can be used (both animations are ended)
        } else if data.idx.is_on_field() && state.rel_time.ca >= 2. {
            CharacterAction::Ca(state.ca_carryover(2.))
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

impl CharacterAttack for Amber {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        for i in 0..18 {
            let t = time + 0.1111 * i as f32;
            atk_queue.add_burst(50.54, &PYRO_GAUGE1A, t, event, data, state);
        }
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(221.76, &PYRO_GAUGE2B, time, event, data, state);
    }

    fn ca(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.apply_ca(223.2, &PYRO_GAUGE2B, time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx {
            match &attack.kind {
                // a1
                DamageType::Burst => state.cr += 10.,
                // a4
                DamageType::Ca => self.ca_time = attack.time,
                _ => (),
            };
            if attack.time - self.ca_time <= 10. {
                state.atk += 15.;
            }
        }
    }

    fn reset_modify(&mut self) -> () {
        self.ca_time = -99.;
    }
}

// Decreases Passion Overload's CD by 20%.

// When inside Fantastic Voyage's circle, Passion Overload's CD is decreased by
// 50% and Bennett cannot be launched by this skill's explosion.
#[derive(Debug)]
pub struct Bennett {
    bonus: f32,
    base_atk: f32,
    burst_time: f32,
}

impl Bennett {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Bennett").vision(Pyro).weapon(Sword).version(1.0)
            .base_hp(12397.0).base_atk(191.0).base_def(771.0)
            .er(26.7)
            .energy_cost(60.)
    }

    pub fn new() -> Self {
        Self {
            bonus: 1.008,
            base_atk: 0.,
            burst_time: -99.,
        }
    }
}

impl Timeline for Bennett {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // check if skill can be used
        if state.rel_time.press >= 4. {
            CharacterAction::PressSkill
        // is burst CD off and has enough energy
        } else if state.rel_time.burst >= 15. && state.energy >= 60. {
            CharacterAction::Burst
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.5134 {
            // 5 attacks in 2.567 seconds
            data.na_idx.to_na(5, state.na_carryover(0.5134))
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

impl CharacterAttack for Bennett {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_burst(419.04, &PYRO_GAUGE2B, time, event, data, state);
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(261.44, &PYRO_GAUGE2B, time, event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(88.06, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(84.49, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(107.95, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(117.98, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na5(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(142.12, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_burst() {
            // save the valur for later
            self.base_atk = state.base_atk;
            self.burst_time = action_state.current_time;
        }
        if attack.time - self.burst_time <= 12. {
            state.flat_atk += self.base_atk * self.bonus;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.bonus = 1.008;
        // self.base_atk = 0.;
        self.burst_time = -99.;
    }
}

// Increases the flame range of Guoba by 20%.

// When Guoba Attack's effects end, Guoba leaves a chili pepper on the spot
// where it disappeared. Picking up a chili pepper increases ATK by 10% for 10s.
#[derive(Debug)]
pub struct Xiangling {
    skill_time: f32,
}

impl Xiangling {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Xiangling").vision(Pyro).weapon(Polearm).version(1.0)
            .base_hp(10875.0).base_atk(225.0).base_def(669.0)
            .em(96.0)
            .energy_cost(80.)
    }

    pub fn new() -> Self {
        Self {
            skill_time: -99.
        }
    }
}

impl Timeline for Xiangling {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // check if skill can be used
        if state.rel_time.press >= 12. {
            CharacterAction::PressSkill
        // is burst CD off and has enough energy
        } else if state.rel_time.burst >= 20. && state.energy >= 80. {
            CharacterAction::Burst
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.48 {
            // 5 attacks in 2.4 seconds
            data.na_idx.to_na(5, state.na_carryover(0.48))
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

impl CharacterAttack for Xiangling {
    // always apply pyro aura
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.apply_burst(129.6, &PYRO_GAUGE1A, time, event, data, state);
        atk_queue.apply_burst(158.4, &PYRO_GAUGE1A, time + 0.3333, event, data, state);
        atk_queue.apply_burst(197.28, &PYRO_GAUGE1A, time + 0.6666, event, data, state);
        for i in 1..11 {
            atk_queue.apply_burst(201.6, &PYRO_GAUGE1A, time + i as f32, event, data, state);
        }
    }

    // always apply pyro aura
    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        for i in 0..4 {
            atk_queue.apply_skill(200.3, &PYRO_GAUGE1A, time + (2 * i) as f32, event, data, state);
        }
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(83.13, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(83.3, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(51.51, &PHYSICAL_GAUGE, time, event, data, state);
        atk_queue.add_na(51.51, &PHYSICAL_GAUGE, time+0.1111, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(27.88, &PHYSICAL_GAUGE, time, event, data, state);
        atk_queue.add_na(27.88, &PHYSICAL_GAUGE, time+0.1111, event, data, state);
        atk_queue.add_na(27.88, &PHYSICAL_GAUGE, time+0.2222, event, data, state);
        atk_queue.add_na(27.88, &PHYSICAL_GAUGE, time+0.3333, event, data, state);
    }

    fn na5(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(140.42, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_skill() {
            self.skill_time = action_state.current_time;
        }
        let dr = attack.time - self.skill_time;
        if attack.idx.0 == 0 && 8. < dr && dr <= 18. {
            // a4
            state.atk += 10.;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.skill_time = -99.;
    }
}

// Diluc's Charged Attack Stamina Cost is decreased by 50%, and its duration is
// increased by 3s.

// The Pyro Enchantment provided by Dawn lasts for 4s longer. Additionally.
// Diluc gains 20% Pyro DMG Bonus during the duration of this effect.
#[derive(Debug)]
pub struct Diluc {
    charge: u8,
    burst_time: f32,
}

impl Diluc {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Diluc").vision(Pyro).weapon(Claymore).version(1.0)
            .base_hp(12981.0).base_atk(335.0).base_def(784.0)
            .cr(24.2)
            .energy_cost(40.)
    }

    pub fn new() -> Self {
        Self {
            charge: 1,
            burst_time: -99.,
        }
    }

    fn infusion(&self, time: f32) -> &'static ElementalGauge {
        if time - self.burst_time <= 12. {
            &PYRO_GAUGE1A
        } else {
            &PHYSICAL_GAUGE
        }
    }
}

impl Timeline for Diluc {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // check if skill can be used
        if state.rel_time.press >= 10. {
            self.charge = 1;
            CharacterAction::PressSkill
        } else if self.charge < 3 {
            self.charge += 1;
            CharacterAction::PressSkill
        // is burst CD off and has enough energy
        } else if state.rel_time.burst >= 12. && state.energy >= 40. {
            CharacterAction::Burst
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.7 {
            // 4 attacks in 2.8 seconds
            data.na_idx.to_na(4, state.na_carryover(0.7))
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        match event {
            CharacterAction::PressSkill => field_energy.push_p(Particle::new(data.character.vision, 1.3333)),
            _ => (),
        }
    }

    fn reset_timeline(&mut self) -> () {
        self.charge = 1;
    }
}

impl CharacterAttack for Diluc {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.burst_time = time;
        atk_queue.add_burst(367.2, &PYRO_GAUGE1A, time, event, data, state);
        atk_queue.add_burst(114.0, &PYRO_GAUGE1A, time+0.5, event, data, state);
        atk_queue.add_burst(114.0, &PYRO_GAUGE1A, time+1.0, event, data, state);
        atk_queue.add_burst(114.0, &PYRO_GAUGE1A, time+1.5, event, data, state);
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        let charge = self.charge;
        self.charge += 1;
        if self.charge > 3 {
            self.charge = 1;
        }
        match charge {
            1 => atk_queue.add_skill(169.92, &PYRO_GAUGE1A, time, event, data, state),
            2 => atk_queue.add_skill(175.68, &PYRO_GAUGE1A, time, event, data, state),
            3 => atk_queue.add_skill(231.84, &PYRO_GAUGE1A, time, event, data, state),
            _ => (),
        }
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(177.31, self.infusion(time), time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(173.23, self.infusion(time), time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(195.33, self.infusion(time), time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(264.86, self.infusion(time), time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_burst() {
            self.burst_time = action_state.current_time;
        }
        // a4
        if attack.idx == data.idx && attack.time - self.burst_time <= 12. {
            state.pyro_dmg += 20.0;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.charge = 1;
        self.burst_time = -99.;
    }
}

// When Jumpy Dumpty and Normal Attacks deal DMG, Klee has a 50% chance to
// obtain an Explosive Spark. This Explosive Spark is consumed by the next
// Charged Attack, which costs no Stamina and deals 50% increased DMG.

// When Klee's Charged Attack results in a CRIT, all party members gain 2
// Elemental Energy.
#[derive(Debug)]
pub struct Klee {
    explosive_spark: bool,
}

impl Klee {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Klee").vision(Pyro).weapon(Catalyst).version(1.0)
            .base_hp(10287.0).base_atk(311.0).base_def(615.0)
            .pyro_dmg(28.8)
            .energy_cost(60.)
    }

    pub fn new() -> Self {
        Self {
            explosive_spark: false,
        }
    }
}

impl Timeline for Klee {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // use ca when explosive_spark
        if data.idx.is_on_field() && self.explosive_spark {
            self.explosive_spark = false;
            CharacterAction::Ca(0.)
        // check if skill can be used
        } else if state.rel_time.press >= 20. {
            self.explosive_spark = true;
            CharacterAction::PressSkill
        // is burst CD off and has enough energy
        } else if state.rel_time.burst >= 15. && state.energy >= 60. {
            CharacterAction::Burst
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.5 && state.rel_time.ca >= 1. {
            if data.na_idx == 3 {
                self.explosive_spark = true;
            }
            // 3 attacks in 1.5 seconds
            data.na_idx.to_na(3, state.na_carryover(0.5))
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        match event {
            CharacterAction::PressSkill => field_energy.push_p(Particle::new(data.character.vision, 4.)),
            CharacterAction::Ca(_) => field_energy.push_e(2.),
            _ => (),
        }
    }

    fn reset_timeline(&mut self) -> () {
        self.explosive_spark = false;
    }
}

impl CharacterAttack for Klee {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        // burst can be used if on field
        if data.idx.0 == 0 {
            for i in 0..6 {
                let t = time + i as f32;
                atk_queue.add_burst(76.76, &PYRO_GAUGE1A, t, event, data, state);
                atk_queue.add_burst(76.76, &PYRO_GAUGE1A, t+0.1111, event, data, state);
                atk_queue.add_burst(76.76, &PYRO_GAUGE1A, t+0.2222, event, data, state);
                atk_queue.add_burst(76.76, &PYRO_GAUGE1A, t+0.3333, event, data, state);
            }
        }
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        for i in 0..3 {
            let t = time + i as f32;
            atk_queue.add_skill(171.36, &PYRO_GAUGE2B, t, event, data, state);
        }
        // 8 hit mines
        for i in 0..8 {
            let t = time + 3. + 0.01111 * i as f32;
            atk_queue.add_skill(59.04, &PYRO_GAUGE1A, t, event, data, state);
        }
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(129.89, &PYRO_GAUGE1A, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(112.32, &PYRO_GAUGE1A, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(161.86, &PYRO_GAUGE1A, time, event, data, state);
    }

    fn ca(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_ca(283.25, &PYRO_GAUGE1A, time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx && attack.kind == DamageType::Ca {
            state.ca_dmg += 50.;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.explosive_spark = false;
    }
}
