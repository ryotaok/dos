use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, CharacterAttack, AtkQueue};
use crate::sim2::types::{CharacterAction, DamageType, Vision, FieldCharacterIndex, WeaponType, FieldEnergy, Particle, VecFieldEnergy, ToNaAction};
use crate::sim2::element::{ElementalGauge, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, ELECTRO_GAUGE4C, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::sim2::record::{CharacterRecord, CharacterData, Enemy};

use WeaponType::*;
use Vision::*;

// version 1.0

// Sword, Claymore, or Polearm-wielding characters within the field created by
// Spirit Blade: Chonghua's Layered Frost have their Normal ATK SPD increased by
// 8%.

// When the field created by Spirit Blade: Chonghua's Layered Frost disappears,
// another spirit blade will be summoned to strike nearby opponents, dealing
// 100% of Chonghua's Layered Frost's Skill DMG as AoE Cryo DMG. Opponents hit
// by this blade will have their Cryo RES decreased by 10% for 8s.
#[derive(Debug)]
pub struct Chongyun {
    apply_debuff: bool,
    skill_time: f32,
    a4_time: f32,
}

impl Chongyun {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Chongyun").vision(Cryo).weapon(Claymore).version(1.0)
            .base_hp(10984.0).base_atk(223.0).base_def(648.0)
            .atk(24.0)
            .energy_cost(40.)
    }

    pub fn new() -> Self {
        Self {
            apply_debuff: false,
            skill_time: -99.,
            a4_time: -99.,
        }
    }
}

impl Timeline for Chongyun {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // is burst CD off and has enough energy
        if state.rel_time.burst >= 12. && state.energy >= 40. {
            CharacterAction::Burst
        // check if skill can be used
        } else if state.rel_time.press >= 15. {
            CharacterAction::PressSkill
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.7085 {
            // 4 attacks in 2.834 seconds
            data.na_idx.to_na(4, state.carryover(0.7085))
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        match event {
            CharacterAction::PressSkill => field_energy.push_p(Particle::new(data.character.vision, 4.)),
            _ => (),
        };
    }
}

impl CharacterAttack for Chongyun {
    // always apply aura
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        for i in 0..3 {
            atk_queue.push(Attack {
                kind: DamageType::Burst,
                multiplier: 256.32,
                element: &CRYO_GAUGE1A,
                aura_application: true,
                time: time + 0.3333 * i as f32,
                idx: data.idx,
            });
        }
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.skill_time = time;
        self.a4_time = time+10.;
        atk_queue.add_skill(261.44, &CRYO_GAUGE2B, time, event, data, state);
        atk_queue.add_skill(261.44, &CRYO_GAUGE2B, time+10., event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(138.38, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(124.78, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(158.78, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(200.09, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        // skill infusion, do not infuse if the attack is infused already
        if attack.time - self.skill_time <= 3. && attack.element.aura != Physical {
            attack.element = &CRYO_GAUGE1A;
        }
        // if !self.apply_debuff && attack.time - self.a4_time <= 10. {
        //     self.apply_debuff = true;
        //     enemy.debuff.cryo += 10.;
        // } else if self.apply_debuff && attack.time - self.a4_time > 10. {
        //     self.apply_debuff = false;
        //     enemy.debuff.cryo -= 10.;
        // }
    }

    fn reset(&mut self) -> () {
        self.apply_debuff = false;
        self.skill_time = -99.;
        self.a4_time = -99.;
    }
}

// Every hit with Frostgnaw regenerates HP for Kaeya equal to 15% of his ATK.

// Opponents Frozen by Frostgnaw will drop additional Elemental Particles.
// Frostgnaw may only produce a maximum of 2 additional Elemental Particles per
// use.
#[derive(Debug)]
pub struct Kaeya {}

impl Kaeya {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Kaeya").vision(Cryo).weapon(Sword).version(1.0)
            .base_hp(11636.0).base_atk(223.0).base_def(792.0)
            .er(26.7)
            .energy_cost(60.)
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Timeline for Kaeya {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // is burst CD off and has enough energy
        if state.rel_time.burst >= 15. && state.energy >= 60. {
            CharacterAction::Burst
        // check if skill can be used
        } else if state.rel_time.press >= 6. {
            CharacterAction::PressSkill
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.5468 {
            // 5 attacks in 2.734 seconds
            data.na_idx.to_na(5, state.carryover(0.5468))
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

impl CharacterAttack for Kaeya {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        for i in 0..12 {
            atk_queue.add_burst(139.92, &CRYO_GAUGE1A, time + 0.6666 * i as f32, event, data, state);
        }
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(344.16, &CRYO_GAUGE2B, time, event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(106.25, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(102.17, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(129.03, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(140.08, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na5(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(174.42, &PHYSICAL_GAUGE, time, event, data, state);
    }

    // fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
    // }

    // fn reset(&mut self) -> () {
    // }
}

// When a character under the effects of Adeptus Art: Herald of Frost triggers
// an Elemental Reaction, their Incoming Healing Bonus is increased by 20% for
// 8s.

// When Qiqi hits opponents with her Normal and Charged Attacks, she has a 50%
// chance to apply a Fortune-Preserving Talisman to them for 6s. This effect can
// only occur once every 30s.
#[derive(Debug)]
pub struct Qiqi {}

impl Qiqi {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Qiqi").vision(Cryo).weapon(Sword).version(1.0)
            .base_hp(12368.0).base_atk(287.0).base_def(922.0)
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Timeline for Qiqi {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // is burst CD off and has enough energy
        if state.rel_time.burst >= 20. && state.energy >= 80. {
            CharacterAction::Burst
        // check if skill can be used
        } else if state.rel_time.press >= 30. {
            CharacterAction::PressSkill
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.45 {
            // 5 attacks in 2.25 seconds
            data.na_idx.to_na(5, state.carryover(0.45))
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    // fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
    // }
}

impl CharacterAttack for Qiqi {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_burst(512.64, &CRYO_GAUGE2B, time, event, data, state);
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(172.8, &CRYO_GAUGE1A, time, event, data, state);
        for i in 1..5 {
            atk_queue.add_skill(64.8, &CRYO_GAUGE1A, time + (2 * i) as f32, event, data, state);
        }
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(74.63, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(76.84, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(47.77, &PHYSICAL_GAUGE, time, event, data, state);
        atk_queue.add_na(47.77, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(48.79, &PHYSICAL_GAUGE, time, event, data, state);
        atk_queue.add_na(48.79, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na5(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(124.61, &PHYSICAL_GAUGE, time, event, data, state);
    }

    // fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
    // }

    // fn reset(&mut self) -> () {
    // }
}
