use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, CharacterAttack, AtkQueue};
use crate::sim2::types::{CharacterAction, DamageType, Vision, FieldCharacterIndex, WeaponType, FieldEnergy, Particle, VecFieldEnergy, ToNaAction};
use crate::sim2::element::{ElementalGauge, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, ELECTRO_GAUGE4C, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::sim2::record::{CharacterRecord, CharacterData, Enemy};

use WeaponType::*;
use Vision::*;

// version 1.0

// Counterattacking with Tidecaller at the precise moment when the character is
// hit grants the maximum DMG Bonus.

// Gain the following effects for 10s after unleashing Tidecaller with its
// maximum DMG Bonus: DMG dealt by Normal and Charged Attacks is increased by
// 15%. ATK SPD of Normal and Charged Attacks is increased by 15%. Greatly
// reduced delay before unleashing Charged Attacks.
#[derive(Debug)]
pub struct Beidou {}

impl Beidou {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Beidou").vision(Electro).weapon(Claymore).version(1.0)
            .base_hp(13050.0).base_atk(225.0).base_def(648.0)
            .electro_dmg(24.0)
            .energy_cost(80.)
            // // a4
            // .na_dmg(15.).ca_dmg(15.).atk_spd(15.)
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Timeline for Beidou {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // check if skill can be used
        if state.rel_time.press >= 7.5 {
            CharacterAction::PressSkill
        // is burst CD off and has enough energy
        } else if state.rel_time.burst >= 20. && state.energy >= 80. {
            CharacterAction::Burst
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.75 {
            // 5 attacks in 3.75 seconds
            data.na_idx.to_na(5, state.na_carryover(0.75))
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        match event {
            CharacterAction::PressSkill => field_energy.push_p(Particle::new(data.character.vision, 2.5)),
            _ => (),
        }
    }
}

impl CharacterAttack for Beidou {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_burst(218.88, &ELECTRO_GAUGE4C, time, event, data, state);
        for i in 1..11 {
            atk_queue.add_burst(172.8, &ELECTRO_GAUGE1A, time + i as f32, event, data, state);
        }
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(218.88, &ELECTRO_GAUGE2B, time, event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(140.59, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(140.08, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(174.59, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(171.02, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na5(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(221.68, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx && attack.kind == DamageType::Skill {
            state.skill_dmg += 288.;
        }
    }

    // fn reset_modify(&mut self) -> () {
    // }
}

// When Fischl hits Oz with a fully-charged Aimed Shot, Oz brings down
// Thundering Retribution, dealing AoE Electro DMG equal to 152.7% of the
// arrow's DMG.

// If your active character triggers an Electro-related Elemental Reaction when
// Oz is on the field, the opponent shall be stricken with Thundering
// Retribution, dealing Electro DMG equal to 80% of Fischl's ATK.
#[derive(Debug)]
pub struct Fischl {
    skill_time: f32,
    thundering_retribution: bool,
}

impl Fischl {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Fischl").vision(Electro).weapon(Bow).version(1.0)
            .base_hp(9189.0).base_atk(244.0).base_def(594.0)
            .atk(24.0)
            .energy_cost(60.)
    }

    pub fn new() -> Self {
        Self {
            skill_time: -99.,
            thundering_retribution: false,
        }
    }
}

impl Timeline for Fischl {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // check if skill can be used
        if data.idx.is_on_field() && state.rel_time.press >= 25. {
            CharacterAction::PressSkill
        // is burst CD off and has enough energy
        } else if state.rel_time.burst >= 15. && state.energy >= 60. {
            CharacterAction::Burst
        } else if data.idx.is_off_field() && state.rel_time.press >= 25. {
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
            // reset cd on burst
            CharacterAction::Burst => state.reduce_skill += 25.,
            CharacterAction::PressSkill => field_energy.push_p(Particle::new(data.character.vision, 10.)),
            _ => (),
        }
    }
}

impl CharacterAttack for Fischl {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_burst(374.4, &ELECTRO_GAUGE1A, time, event, data, state);
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(207.79, &ELECTRO_GAUGE1A, time, event, data, state);
        for i in 1..11 {
            atk_queue.add_skill(159.84, &ELECTRO_GAUGE1A, time+i as f32, event, data, state);
        }
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(87.21, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(92.48, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(114.92, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(114.07, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na5(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(142.46, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.time - self.skill_time <= 12. && enemy.trigger_er(&attack.element.aura).is_electro() {
            self.thundering_retribution = true;
        }
        if attack.idx == data.idx && attack.kind == DamageType::Skill && self.thundering_retribution {
            attack.multiplier += 80.;
            self.thundering_retribution = false;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.skill_time = -99.;
        self.thundering_retribution = false;
    }
}

// Hits by Charged Attacks apply Violet Arc's Conductive status to opponents.

// Opponents hit by Lightning Rose have their DEF decreased by 15% for 10s.
#[derive(Debug)]
pub struct Lisa {
    burst_time: f32,
    conductive_status: u8,
    apply_debuff: bool,
}

impl Lisa {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Lisa").vision(Electro).weapon(Catalyst).version(1.0)
            .base_hp(9570.0).base_atk(232.0).base_def(573.0)
            .em(96.0)
            .energy_cost(80.)
    }

    pub fn new() -> Self {
        Self {
            burst_time: -99.,
            conductive_status: 0,
            apply_debuff: false,
        }
    }
}

impl Timeline for Lisa {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // check if skill can be used (both animations are ended)
        if self.conductive_status == 3 && state.rel_time.hold >= 16. {
            CharacterAction::HoldSkill
        } else if state.rel_time.press >= 1. && state.rel_time.hold >= 16. {
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
            CharacterAction::PressSkill => {
                self.conductive_status += 1;
                field_energy.push_p(Particle::new(data.character.vision, 0.3));
            },
            CharacterAction::HoldSkill => {
                self.conductive_status = 0;
                field_energy.push_p(Particle::new(data.character.vision, 5.));
            },
            _ => (),
        }
    }

    fn reset_timeline(&mut self) -> () {
        self.conductive_status = 0;
    }
}

impl CharacterAttack for Lisa {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        for i in 0..28 {
            let t = time + 0.5555 * i as f32;
            atk_queue.add_burst(65.81, &ELECTRO_GAUGE1A, t, event, data, state);
        }
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(144.0, &ELECTRO_GAUGE1A, time, event, data, state);
    }

    fn hold(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(876.96, &ELECTRO_GAUGE2B, time, event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(71.28, &ELECTRO_GAUGE1A, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(64.66, &ELECTRO_GAUGE1A, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(77.04, &ELECTRO_GAUGE1A, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(98.93, &ELECTRO_GAUGE1A, time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_burst() {
            self.burst_time = action_state.current_time;
        }
        if !self.apply_debuff && attack.time - self.burst_time <= 25. {
            self.apply_debuff = true;
            enemy.def_down += 15.;
        } else if self.apply_debuff && attack.time - self.burst_time > 25. {
            self.apply_debuff = false;
            enemy.def_down -= 15.;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.burst_time = -99.;
        self.conductive_status = 0;
        self.apply_debuff = false;
    }
}

// Decreases Claw and Thunder's CD by 18%. Using Lightning Fang resets the CD of
// Claw and Thunder.

// When Razor's Energy is below 50%, increases Energy Recharge by 30%.
#[derive(Debug)]
pub struct Razor {
    electro_sigil: u8,
    burst_time: f32,
}

impl Razor {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Razor").vision(Electro).weapon(Claymore).version(1.0)
            .base_hp(11962.0).base_atk(234.0).base_def(751.0)
            .physical_dmg(30.0)
            .energy_cost(80.)
    }

    pub fn new() -> Self {
        Self {
            electro_sigil: 0,
            burst_time: -99.,
        }
    }
}

impl Timeline for Razor {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // check if skill can be used (both animations are ended)
        if self.electro_sigil == 3 && state.rel_time.hold >= 8. {
            CharacterAction::HoldSkill
        } else if state.rel_time.press >= 5. && state.rel_time.hold >= 8. { // a1
            CharacterAction::PressSkill
        // is burst CD off and has enough energy
        } else if state.rel_time.burst >= 20. && state.energy >= 80. {
            CharacterAction::Burst
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.6835 {
            // 4 attacks in 2.734 seconds
            data.na_idx.to_na(4, state.na_carryover(0.6835))
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        if state.energy < 40. {
            state.er += 30.;
        }
        match event {
            CharacterAction::Burst => {
                self.burst_time = state.current_time;
                // a1
                state.reduce_skill += 20.;
            },
            CharacterAction::PressSkill => {
                self.electro_sigil += 1;
                field_energy.push_p(Particle::new(data.character.vision, 4.));
            },
            CharacterAction::HoldSkill => {
                state.energy += (5 * self.electro_sigil) as f32;
                field_energy.push_p(Particle::new(data.character.vision, 5.));
                self.electro_sigil = 0;
            },
            _ => (),
        }
        if state.current_time - self.burst_time <= 15. {
            state.atk_spd += 40.;
        }
    }

    fn reset_timeline(&mut self) -> () {
        self.electro_sigil = 0;
    }
}

impl CharacterAttack for Razor {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        // burst can be used if on field
        atk_queue.add_burst(288.0, &ELECTRO_GAUGE2B, time, event, data, state);
        if data.idx.0 == 0 {
            for i in 0..20 {
                let t = time + 0.75 * i as f32;
                atk_queue.add_burst(43.2, &ELECTRO_GAUGE1A, t, event, data, state);
            }
        }
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(358.56, &ELECTRO_GAUGE2B, time, event, data, state);
    }

    fn hold(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(531.36, &ELECTRO_GAUGE2B, time, event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(171.13, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(147.42, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(184.32, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(242.72, &PHYSICAL_GAUGE, time, event, data, state);
    }

    // fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
    // }

    fn reset_modify(&mut self) -> () {
        self.electro_sigil = 0;
    }
}

#[derive(Debug)]
pub struct Keqing {
    burst_time: f32,
    skill_time: f32,
}

// After recasting Stellar Restoration while a Lightning Stiletto is present,
// Keqing's weapon gains an Electro Infusion for 5s.

// When casting Starward Sword, Keqing's CRIT Rate is increased by 15%, and her
// Energy Recharge is increased by 15%. This effect lasts for 8s.
impl Keqing {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Keqing").vision(Electro).weapon(Sword).version(1.0)
            .base_hp(13103.0).base_atk(323.0).base_def(799.0)
            .cd(88.4)
            .energy_cost(40.)
    }

    pub fn new() -> Self {
        Self {
            burst_time: -99.,
            skill_time: -99.,
        }
    }

    fn infusion(&self, time: f32) -> &'static ElementalGauge {
        if time - self.skill_time <= 5. {
            &ELECTRO_GAUGE1A
        } else {
            &PHYSICAL_GAUGE
        }
    }
}

impl Timeline for Keqing {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // check if skill can be used
        if state.rel_time.press >= 7.5 {
            CharacterAction::PressSkill
        // is burst CD off and has enough energy
        } else if state.rel_time.burst >= 12. && state.energy >= 40. {
            CharacterAction::Burst
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.4034 {
            // 5 attacks in 2.017 seconds
            data.na_idx.to_na(5, state.na_carryover(0.4034))
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        match event {
            CharacterAction::Burst => self.burst_time = state.current_time,
            CharacterAction::PressSkill => field_energy.push_p(Particle::new(data.character.vision, 2.5)),
            _ => (),
        };
        if state.current_time - self.burst_time <= 8. {
            state.er += 15.;
        }
    }

    fn reset_timeline(&mut self) -> () {
        self.burst_time = -99.;
    }
}

impl CharacterAttack for Keqing {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_burst(158.4, &ELECTRO_GAUGE1A, time, event, data, state);
        for i in 1..9 {
            atk_queue.add_burst(43.2, &ELECTRO_GAUGE1A, time + i as f32, event, data, state);
        }
        atk_queue.add_burst(339.84, &ELECTRO_GAUGE1A, time + 9., event, data, state);
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.skill_time = time;
        atk_queue.add_skill(90.72, &ELECTRO_GAUGE1A, time, event, data, state);
        atk_queue.add_skill(302.4, &ELECTRO_GAUGE1A, time+0.5, event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(81.09, self.infusion(time), time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(81.09, self.infusion(time), time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(107.61, self.infusion(time), time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(62.22, self.infusion(time), time, event, data, state);
        atk_queue.add_na(68.0, self.infusion(time), time, event, data, state);
    }

    fn na5(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(132.43, self.infusion(time), time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_burst() {
            self.burst_time = action_state.current_time;
        }
        if attack.idx == data.idx && attack.time - self.burst_time <= 8. {
            state.cr += 15.;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.burst_time = -99.;
        self.skill_time = -99.;
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     use crate::sim2::types::Vision;
//     use crate::sim2::simulate::simulate;
//     use crate::sim2::testutil::{TestEnvironment2, TestCharacter};

//     #[test] #[ignore]
//     fn lisa() {
//         let mut timers = ICDTimers::new();
//         let cr = TestCharacter::record(Vision::Electro);
//         let mut character = Lisa::new(FieldCharacterIndex(0), &timers);
//         character.na = NaLoop::noop(FieldCharacterIndex(0));

//         let mut env = TestEnvironment2::character(0, &mut timers, State::new(), &cr, &mut character);

//         let mut total_dmg = 0.0;
//         for _ in 0..10 {
//             total_dmg += simulate(0.5, &mut env.data, &mut env.ability, &mut env.atk_queue, &mut env.field_energy, &mut env.enemy);
//         }

//         let expect = 3.0 * 144.0 + 876.96;
//         assert_eq!(total_dmg, expect);
//     }
// }
