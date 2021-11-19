use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, CharacterAttack, AtkQueue};
use crate::sim2::types::{CharacterAction, DamageType, Vision, FieldCharacterIndex, WeaponType, FieldEnergy, Particle, VecFieldEnergy, ToNaAction};
use crate::sim2::element::{ElementalGauge, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::sim2::record::{CharacterRecord, CharacterData, Enemy};

use WeaponType::*;
use Vision::*;

// When nearby party members gain Elemental Orbs or Particles, Chakra Desiderata
// gains 2 Resolve stacks. This effect can occur once every 3s.

// Each 1% above 100% Energy Recharge that the Raiden Shogun possesses grants
// her:
// - 0.6% greater Energy restoration from Musou Isshin
// - 0.4% Electro DMG Bonus
#[derive(Debug)]
pub struct RaidenShogun {
    resolve_stack: f32,
    burst_time: f32,
    energy_restoration: u8,
}

impl RaidenShogun {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Raiden Shogun").vision(Electro).weapon(Polearm).version(2.1)
            .base_hp(12907.0).base_atk(337.0).base_def(789.0)
            .er(32.0)
            .energy_cost(90.)
    }

    pub fn new() -> Self {
        Self {
            resolve_stack: 40.0, // TODO starting 200 energy consumption
            burst_time: -99.,
            energy_restoration: 0,
        }
    }

    fn musou_isshin_na(&mut self, multiplier: f32, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State) -> () {
        if time - self.burst_time <= 7. {
            atk_queue.add_burst(multiplier + 1.31 * self.resolve_stack, &ELECTRO_GAUGE1A, time, event, data, state);
        } else {
            atk_queue.add_na(multiplier, &PHYSICAL_GAUGE, time, event, data, state);
        }
    }
}

impl Timeline for RaidenShogun {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // is burst CD off and has enough energy
        if state.rel_time.burst >= 18. && state.energy >= 90. {
            CharacterAction::Burst
        // check if skill can be used
        } else if state.rel_time.press >= 10. {
            CharacterAction::PressSkill
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
            CharacterAction::Burst => {
                self.burst_time = state.current_time;
                self.energy_restoration = 0;
            },
            CharacterAction::PressSkill => field_energy.push_p(Particle::new(data.character.vision, 4.4)),
            CharacterAction::Na1(_) |
            CharacterAction::Na2(_) |
            CharacterAction::Na3(_) |
            CharacterAction::Na4(_) |
            CharacterAction::Na5(_) |
            CharacterAction::Na6(_) => {
                if state.current_time - self.burst_time <= 7. && self.energy_restoration < 5 {
                    let bonus = 1.0 + 0.6 * state.er / 100.0;
                    field_energy.push_e(2.5 * bonus);
                    self.energy_restoration += 1;
                }
            },
            _ => (),
        }
    }

    fn reset_timeline(&mut self) -> () {
        self.burst_time = -99.;
        self.energy_restoration = 0;
    }
}

impl CharacterAttack for RaidenShogun {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.burst_time = time;
        atk_queue.add_burst(721.44 + 7. * self.resolve_stack, &ELECTRO_GAUGE2B, time, event, data, state);
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(210.96, &ELECTRO_GAUGE1A, time, event, data, state);
        for i in 1..12 {
            atk_queue.add_skill(75.6, &ELECTRO_GAUGE1A, time + 0.9 * i as f32, event, data, state);
        }
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.musou_isshin_na(78.37, time, event, data, atk_queue, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.musou_isshin_na(78.54, time, event, data, atk_queue, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.musou_isshin_na(98.6, time, event, data, atk_queue, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.musou_isshin_na(57.29, time, event, data, atk_queue, state);
        self.musou_isshin_na(57.29, time, event, data, atk_queue, state);
    }

    fn na5(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.musou_isshin_na(129.37, time, event, data, atk_queue, state);
    }

    fn reset_attack(&mut self) -> () {
        self.burst_time = -99.;
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        // if action_state.did_burst() {
        //     self.burst_time = action_state.current_time;
        // }
        // if action_state.did_skill() {
        //     self.skill_time = action_state.current_time;
        // }
        // TODO the skill is always active and the bonus is based on energy_cost 80
        state.burst_dmg += 24.;
        if attack.idx == data.idx {
            state.electro_dmg += 0.4 * action_state.er;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.burst_time = -99.;
        self.energy_restoration = 0;
    }
}

// While in the Crowfeather Cover state provided by Tengu Stormcall, Aimed Shot
// charge times are decreased by 60%.

// When Tengu Juurai: Ambush hits opponents, Kujou Sara will restore 1.2 Energy
// to all party members for every 100% Energy Recharge she has. This effect can
// be triggered once every 3s.
#[derive(Debug)]
pub struct KujouSara {
    base_atk: f32,
    skill_time: f32,
}

impl KujouSara {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Kujou Sara").vision(Electro).weapon(Bow).version(2.1)
            .base_hp(9570.0).base_atk(195.0).base_def(628.0)
            .atk(24.0)
            .energy_cost(80.)
    }

    pub fn new() -> Self {
        Self {
            base_atk: 0.,
            skill_time: -99.
        }
    }
}

impl Timeline for KujouSara {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // is burst CD off and has enough energy
        if state.rel_time.burst >= 20. && state.energy >= 80. {
            CharacterAction::Burst
        // check if skill can be used
        } else if state.rel_time.press >= 10. {
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
            CharacterAction::PressSkill => {
                field_energy.push_p(Particle::new(data.character.vision, 3.));
                field_energy.push_e(1.2 * (100. + state.er) / 100.);
            },
            _ => (),
        }
    }
}

impl CharacterAttack for KujouSara {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_burst(737.28, &ELECTRO_GAUGE1A, time, event, data, state);
        for i in 1..4 {
            atk_queue.add_burst(61.42, &ELECTRO_GAUGE1A, time + i as f32, event, data, state);
        }
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(226.37, &ELECTRO_GAUGE1A, time, event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(78.08, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(81.9, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(95.88, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(99.62, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na5(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(114.75, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_burst() {
            // save the valur for later
            self.base_atk = state.base_atk;
            self.skill_time = action_state.current_time;
        }
        if action_state.did_skill() {
            // save the valur for later
            self.base_atk = state.base_atk;
            self.skill_time = action_state.current_time;
        }
        if attack.time - self.skill_time <= 6. {
            state.flat_atk += 0.7733 * self.base_atk;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.skill_time = -99.;
    }
}

#[derive(Debug)]
pub struct Aloy {
    skill_time: f32,
}

impl Aloy {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Aloy").vision(Cryo).weapon(Bow).version(2.1)
            .base_hp(10899.0).base_atk(234.0).base_def(676.0)
            .cryo_dmg(28.8)
            .energy_cost(40.)
    }

    pub fn new() -> Self {
        Self {
            skill_time: -99.
        }
    }

    fn infusion(&self, time: f32) -> &'static ElementalGauge {
        if time - self.skill_time <= 10. {
            &CRYO_GAUGE1A
        } else {
            &PHYSICAL_GAUGE
        }
    }
}

// When Aloy receives the Coil effect from Frozen Wilds, her ATK is increased by
// 16%, while nearby party members' ATK is increased by 8%. This effect lasts
// 10s.

// When Aloy is in the Rushing Ice state conferred by Frozen Wilds, her Cryo DMG
// Bonus increases by 3.5% every 1s. A maximum Cryo DMG Bonus increase of 35%
// can be gained in this way.
impl Timeline for Aloy {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // is burst CD off and has enough energy
        if state.rel_time.burst >= 12. && state.energy >= 40. {
            CharacterAction::Burst
        // check if skill can be used
        } else if state.rel_time.press >= 20. {
            CharacterAction::PressSkill
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
            CharacterAction::PressSkill => field_energy.push_p(Particle::new(data.character.vision, 5.)),
            _ => (),
        }
    }
}

impl CharacterAttack for Aloy {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_burst(646.56, &CRYO_GAUGE1A, time, event, data, state);
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.skill_time = time;
        atk_queue.add_skill(319.68, &CRYO_GAUGE1A, time, event, data, state);
        for i in 1..7 {
            atk_queue.add_skill(72.0, &CRYO_GAUGE1A, time + 0.5 * i as f32, event, data, state);
        }
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(37.68, self.infusion(time), time, event, data, state);
        atk_queue.add_na(42.39, self.infusion(time), time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(76.93, self.infusion(time), time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(94.2, self.infusion(time), time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(117.12, self.infusion(time), time, event, data, state);
    }

    fn reset_attack(&mut self) -> () {
        self.skill_time = -99.;
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_skill() {
            self.skill_time = action_state.current_time;
        }
        if attack.idx == data.idx {
            if attack.time - self.skill_time <= 10. {
                state.cryo_dmg += 17.5;
                state.na_dmg += 47.6;
                state.atk += 16.;
            }
        } else {
            if attack.time - self.skill_time <= 10. {
                state.atk += 8.;
            }
        }
    }

    fn reset_modify(&mut self) -> () {
        self.skill_time = -99.;
    }
}

// If Sangonomiya Kokomi's own Bake-Kurage are on the field when she uses
// Nereid's Ascension, the Bake-Kurage's duration will be refreshed.

// While donning the Ceremonial Garment created by Nereid's Ascension, the
// Normal and Charged Attack DMG Bonus Sangonomiya Kokomi gains based on her Max
// HP will receive a further increase based on 15% of her Healing Bonus.

// Sangonomiya Kokomi has a 25% Healing Bonus, but a 100% decrease in CRIT Rate.
#[derive(Debug)]
pub struct SangonomiyaKokomi {
    burst_time: f32,
}

impl SangonomiyaKokomi {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Sangonomiya Kokomi").vision(Hydro).weapon(Catalyst).version(2.1)
            .base_hp(12262.0).base_atk(226.0).base_def(628.0)
            // passive 2?
            .cr(-100.0)
            .hydro_dmg(28.8)
            .energy_cost(70.)
    }

    pub fn new() -> Self {
        Self {
            burst_time: -99.
        }
    }
}

impl Timeline for SangonomiyaKokomi {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // is burst CD off and has enough energy
        if state.rel_time.burst >= 18. && state.energy >= 70. {
            CharacterAction::Burst
        // check if skill can be used
        } else if state.rel_time.press >= 20. {
            CharacterAction::PressSkill
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
            CharacterAction::Burst => state.reduce_skill = 99.,
            CharacterAction::PressSkill => field_energy.push_p(Particle::new(data.character.vision, 4.)),
            _ => (),
        }
    }
}

impl CharacterAttack for SangonomiyaKokomi {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_burst(100., &HYDRO_GAUGE2B, time, event, data, state);
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        for i in 0..7 {
            atk_queue.add_skill(196.54, &HYDRO_GAUGE1A, time + (2 * i) as f32, event, data, state);
        }
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(123.08, &HYDRO_GAUGE1A, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(110.77, &HYDRO_GAUGE1A, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(169.75, &HYDRO_GAUGE1A, time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_burst() {
            self.burst_time = action_state.current_time;
        }
        if attack.idx == data.idx && attack.time - self.burst_time <= 10. {
            match &attack.kind {
                DamageType::Burst => state.flat_dmg += 18.75 / 100. * state.HP(),
                DamageType::Skill => state.flat_dmg += 12.77 / 100. * state.HP(),
                DamageType::Na    => state.flat_dmg += 8.71 / 100. * state.HP(),
                _ => (),
            }
        }
    }

    fn reset_modify(&mut self) -> () {
        self.burst_time = -99.;
    }
}
