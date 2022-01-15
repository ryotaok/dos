use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, CharacterAttack, AtkQueue};
use crate::sim2::types::{CharacterAction, DamageType, Vision, FieldCharacterIndex, WeaponType, FieldEnergy, Particle, VecFieldEnergy, ToNaAction, PeriodicStack};
use crate::sim2::element::{ElementalGauge, PHYSICAL_GAUGE, PYRO_GAUGE1A, PYRO_GAUGE2B, HYDRO_GAUGE1A, HYDRO_GAUGE2B, ELECTRO_GAUGE1A, ELECTRO_GAUGE2B, CRYO_GAUGE1A, CRYO_GAUGE2B, ANEMO_GAUGE1A, ANEMO_GAUGE2B, GEO_GAUGE1A, GEO_GAUGE2B, DENDRO_GAUGE1A, DENDRO_GAUGE2B};
use crate::sim2::record::{CharacterRecord, CharacterData, Enemy};

use WeaponType::*;
use Vision::*;

// When Arataki Itto uses consecutive Arataki Kesagiri, he obtains the following
// effects:
// • Each slash causes the subsequent slash to have 10% more ATK SPD. Max ATK
//   SPD increase is 30%.
// • Increases his resistance to interruption.
// These effects will be cleared once he stops performing consecutive slashes.

// Arataki Kesagiri DMG is increased by 35% of Arataki Itto's DEF.
#[derive(Debug)]
pub struct AratakiItto {
    ca_combo: bool,
    burst_time: f32,
    superstrength: u8,
    periodic_stack: PeriodicStack,
}

impl AratakiItto {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Arataki Itto").vision(Geo).weapon(Claymore).version(2.3)
            .base_hp(12858.).base_atk(227.).base_def(959.)
            .cr(24.2)
            .energy_cost(70.)
    }

    pub fn new() -> Self {
        Self {
            ca_combo: false,
            burst_time: -99.,
            superstrength: 0,
            periodic_stack: PeriodicStack::disable(),
        }
    }

    fn infusion(&self, time: f32, is_on_field: bool) -> &'static ElementalGauge {
        if is_on_field && time - self.burst_time <= 11. {
            &GEO_GAUGE1A
        } else {
            &PHYSICAL_GAUGE
        }
    }
}

impl Timeline for AratakiItto {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // is burst CD off and has enough energy
        if state.rel_time.burst >= 18. && state.energy >= 70. {
            CharacterAction::Burst
        // check if skill can be used
        } else if state.rel_time.press >= 10. {
            CharacterAction::PressSkill
        // use ca
        } else if self.superstrength == 5 && state.rel_time.ca >= 0.5 {
            self.ca_combo = true;
            CharacterAction::Ca(0.)
        } else if self.ca_combo && self.superstrength > 0 && state.rel_time.ca >= 0.5 {
            // TODO ca final?
            if self.superstrength == 1 {
                self.ca_combo = false;
            }
            CharacterAction::Ca(state.ca_carryover(0.5))
        // check if normal attacks can be used (both animations are ended)
        } else if !self.ca_combo && state.rel_time.na >= 0.654 {
            // 4 attacks in 2.616 seconds
            data.na_idx.to_na(4, state.na_carryover(0.654))
        } else {
            CharacterAction::StandStill
        }
    }

    // generate energy and modify acceleration states according to the event
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        match event {
            CharacterAction::Burst => self.burst_time = state.current_time,
            CharacterAction::PressSkill => {
                self.periodic_stack = PeriodicStack::new(state.current_time, 2., 6.);
                field_energy.push_p(Particle::new(data.character.vision, 3.));
            },
            CharacterAction::Ca(_) => self.superstrength -= 1,
            CharacterAction::Na2(_) => self.superstrength += 1,
            CharacterAction::Na4(_) => self.superstrength += 2,
            _ => (),
        }
        self.superstrength += self.periodic_stack.grant(state.current_time);
        if state.current_time - self.burst_time <= 11. {
            state.atk_spd += 10.;
            match event {
                CharacterAction::Na1(_) |
                CharacterAction::Na3(_) => self.superstrength += 1,
                _ => (),
            }
        }
        if self.superstrength > 5 {
            self.superstrength = 5;
        }
    }

    fn reset_timeline(&mut self) -> () {
        self.burst_time = -99.;
        self.superstrength = 0;
        self.ca_combo = false;
    }
}

impl CharacterAttack for AratakiItto {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.burst_time = time;
        atk_queue.add_burst(0., &GEO_GAUGE1A, time, event, data, state);
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_skill(552.96, &GEO_GAUGE1A, time, event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(156.62, self.infusion(time, data.idx.is_on_field()), time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(150.96, self.infusion(time, data.idx.is_on_field()), time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(181.15, self.infusion(time, data.idx.is_on_field()), time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(231.72, self.infusion(time, data.idx.is_on_field()), time, event, data, state);
    }

    fn ca(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        // atk_queue.add_ca(180.2, self.infusion(time, data.idx.is_on_field()), time, event, data, state);
        // 180.2, 377.4
        // 377.4 - 180.2 = 197.2
        // 197.2 / 5 = 39.44
        // 180.2 + 39.44 = 219.64
        atk_queue.add_ca(219.64, self.infusion(time, data.idx.is_on_field()), time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_burst() {
            self.burst_time = action_state.current_time;
        }
        if attack.idx == data.idx {
            if attack.time - self.burst_time <= 11. {
                state.flat_atk += 1.0368 * state.DEF();
            }
            if attack.kind == DamageType::Ca {
                state.flat_dmg += 0.35 * state.DEF();
            }
        }
    }

    fn reset_modify(&mut self) -> () {
        self.burst_time = -99.;
    }
}

// Increases all nearby party members' DEF by 25% for 12s after using Juuga:
// Forward Unto Victory.

// Gorou receives the following DMG Bonuses to his attacks based on his DEF:
// * Inuzaka All-Round Defense's Skill DMG is increased by 156% of his DEF.
// * Juuga: Forward Unto Victory's Skill DMG and Crystal Collapse DMG is
//   increased by 15.6% of DEF.
#[derive(Debug)]
pub struct Gorou {
    burst_time: f32,
    skill_time: f32,
}

impl Gorou {
    pub fn record() -> CharacterRecord {
        CharacterRecord::default()
            .name("Gorou C6").vision(Geo).weapon(Bow).version(2.3)
            .base_hp(9570.).base_atk(183.).base_def(648.)
            .geo_dmg(24.)
            .energy_cost(80.)
    }

    pub fn new() -> Self {
        Self {
            burst_time: -99.,
            skill_time: -99.,
        }
    }
}

impl Timeline for Gorou {
    // perform an action
    fn decide_action(&mut self, state: &ActionState, data: &mut CharacterData) -> CharacterAction {
        // check if skill can be used
        // } else if state.rel_time.press >= 10. {
        // C1
        if state.rel_time.press >= 8. {
            CharacterAction::PressSkill
        // is burst CD off and has enough energy
        } else if state.rel_time.burst >= 20. && state.energy >= 80. {
            CharacterAction::Burst
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.4875 {
            // 4 attacks in 1.95 seconds
            data.na_idx.to_na(4, state.na_carryover(0.4875))
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

impl CharacterAttack for Gorou {
    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        // atk_queue.add_burst(176.79, &GEO_GAUGE2B, time, event, data, state);
        // for i in 0..6 {
        //     atk_queue.add_burst(110.34, &GEO_GAUGE1A, time + (i as f32) * 1.5, event, data, state);
        // }
        // C5
        atk_queue.add_burst(208.71, &GEO_GAUGE2B, time, event, data, state);
        for i in 0..6 {
            atk_queue.add_burst(130.26, &GEO_GAUGE1A, time + (i as f32) * 1.5, event, data, state);
        }
    }

    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        // atk_queue.add_skill(192.96, &GEO_GAUGE2B, time, event, data, state);
        // C3
        atk_queue.add_skill(227.8, &GEO_GAUGE2B, time, event, data, state);
    }

    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(74.63, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(73.44, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(97.75, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        atk_queue.add_na(116.62, &PHYSICAL_GAUGE, time, event, data, state);
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_burst() {
            self.burst_time = action_state.current_time;
        }
        if action_state.did_skill() {
            self.skill_time = action_state.current_time;
        }
        // if attack.time - self.skill_time <= 10. {
        if attack.time - self.skill_time <= 13. /* C2 */ {
            // state.flat_def += 371.;
            // state.geo_dmg += 15.;
            // C3
            state.flat_def += 438.;
            state.geo_dmg += 15.;
        }
        if attack.time - self.burst_time <= 12. {
            state.def += 25.;
            // C6
            if attack.element.aura == Vision::Geo {
                state.cd += 40.;
            }
        }
        // a4
        if attack.idx == data.idx {
            if attack.kind == DamageType::Skill {
                state.flat_dmg += 1.56 * state.DEF();
            }
            if attack.kind == DamageType::Burst {
                state.flat_dmg += 0.156 * state.DEF();
            }
        }
    }

    fn reset_modify(&mut self) -> () {
        self.burst_time = -99.;
        self.skill_time = -99.;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::sim2::simulate;
    use crate::sim2::simulate::History;
    use crate::sim2::testutil;
    use crate::sim2::testutil::{NoopTimeline};
    use crate::sim2::types::CharacterAction;
    use crate::sim2::attack::{DamageResultUtil};
    use crate::sim2::record::{TimelineMember, WeaponRecord, Artifact};

    #[test]
    fn itto_1() {
        let mut history = History::<1>::new(12., 0.2);
        let mut character = AratakiItto::new();
        let mut weapon = NoopTimeline {};
        let mut artifact = NoopTimeline {};
        let mut states = [ActionState::new(); 1];
        let mut members = [TimelineMember {
            character: &mut character,
            weapon: &mut weapon,
            artifact: &mut artifact,
        }; 1];
        let cr = AratakiItto::record();
        let wr = WeaponRecord::default();
        let ar = Artifact::default();
        let mut data = [CharacterData::new(0, &cr, &wr, &ar); 1];

        states[0].energy += 70.0;
        simulate::decide_action(&mut history, &mut members, &mut states, &mut data);
        use CharacterAction::*;
        assert_eq!(history.action, vec![[Burst],
          [PressSkill],
          [Na1(0.0)], [StandStill], [StandStill],
          [Na2(0.006000042)], [StandStill], [StandStill],
          [Na3(0.012000084)], [StandStill], [StandStill],
          [Na4(0.018000126)],
          [Ca(0.0)], [StandStill], [StandStill],
          [Ca(0.100000024)], [StandStill],
          [Ca(0.0)], [StandStill], [StandStill],
          [Ca(0.100000024)], [StandStill],
          [Ca(0.0)], [StandStill], [StandStill],
          [Ca(0.100000024)],
          [Na1(0.0)], [StandStill], [StandStill],
          [Na2(0.006000042)], [StandStill], [StandStill],
          [Na3(0.012000084)], [StandStill], [StandStill],
          [Na4(0.018000126)],
          [Ca(0.0)], [StandStill], [StandStill],
          [Ca(0.100000024)], [StandStill],
          [Ca(0.0)], [StandStill], [StandStill],
          [Ca(0.100000024)], [StandStill],
          [Ca(0.0)],
          [Na1(0.0)], [StandStill], [StandStill],
          [Na2(0.006000042)], [StandStill],
          [PressSkill],
          [Na1(0.0)], [StandStill], [StandStill],
          [Na2(0.006000042)],
          [Ca(0.0)], [StandStill], [StandStill],
          [Ca(0.100000024)]]
        );
    }
}