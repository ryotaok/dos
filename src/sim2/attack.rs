use crate::sim2::types::{CharacterAction, DamageType, FieldCharacterIndex};
use crate::sim2::timeline::ActionState;
use crate::sim2::state::State;
use crate::sim2::element::{ElementalGauge, ElementalReactionType, ElementalReaction};
use crate::sim2::record::{CharacterData, Enemy};

#[derive(Debug)]
pub struct Attack {
    // type of this `Attack`. For example, Xiangling's skill summons Guoba to
    // deal DoT Pyro DMG. since these damages are created by her skill, the
    // `kind` is `DamageType::Skill`.
    pub kind: DamageType,

    pub multiplier: f32,

    // elemental gauge of this `Attack`.
    pub element: &'static ElementalGauge,

    pub aura_application: bool,

    // the time when this attack hits the enemy
    pub time: f32,

    pub idx: FieldCharacterIndex,
}

impl Attack {
    pub fn outgoing_damage(&self, state: &State, data: &CharacterData) -> f32 {
        let bonus = state.DMG_bonus(&self.kind, &self.element.aura);
        let crcd = state.CRCD();
        let atk = match (data.character.name, &self.kind) {
            ("Albedo", DamageType::Skill) |
            ("Noelle", DamageType::Skill) => state.DEF(),
            _ => state.ATK(),
        };
        let power = atk * bonus * crcd;
        self.multiplier / 100.0 * power * state.get_talent_bonus(&self.kind)
    }

    pub fn incoming_damage(&self, outgoing_damage: f32, state: &State, data: &CharacterData, enemy: &mut Enemy) -> f32 {
        let def_down = 1.0 - enemy.def_down / 100.0;
        let enemy_defense = enemy.level / (enemy.level * def_down + enemy.level);
        let resistance = enemy.resistance(self.time, &self.element.aura);
        let dmg = outgoing_damage * resistance * enemy_defense;
        if self.aura_application {
            self.elemental_reaction(dmg, resistance, state, data, enemy)
        } else {
            dmg
        }
    }

    pub fn elemental_reaction(&self, outgoing_damage: f32, resistance: f32, state: &State, data: &CharacterData, enemy: &mut Enemy) -> f32 {
        use ElementalReactionType::*;
        let elemental_reaction = ElementalReaction::new(enemy.aura.aura, self.element.aura);
        let dmg = match elemental_reaction {
            Overloaded(ref er) |
            Shatter(ref er) |
            ElectorCharged(ref er) |
            Superconduct(ref er) |
            Swirl(ref er) => outgoing_damage + enemy.resistance(self.time, &er.attack) * er.transformative_reaction(state.em, state.transformative_bonus),
            Vaporize(ref er) |
            Melt(ref er) => outgoing_damage * er.amplifying_reaction(state.em, state.amplifying_bonus),
            Crystallize(_) |
            Equalize(_) |
            Freeze(_) |
            Burn(_) |
            Neutralize(_) => outgoing_damage,
        };
        enemy.aura.trigger2(self.time, &mut enemy.aura_time, &self);
        match &elemental_reaction {
            Freeze(_) => enemy.isfrozen = true,
            Superconduct(_) => enemy.superconduct_time = self.time,
            _ => (),
        }
        dmg
    }
}

pub trait CharacterAttack {
    fn attack(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State) -> () {
        use CharacterAction::*;
        match event {
            StandStill => (),
            Burst => self.burst(time, event, data, atk_queue, state),
            PressSkill => self.press(time, event, data, atk_queue, state),
            HoldSkill => self.hold(time, event, data, atk_queue, state),
            Ca => self.ca(time, event, data, atk_queue, state),
            Na1 => self.na1(time, event, data, atk_queue, state),
            Na2 => self.na2(time, event, data, atk_queue, state),
            Na3 => self.na3(time, event, data, atk_queue, state),
            Na4 => self.na4(time, event, data, atk_queue, state),
            Na5 => self.na5(time, event, data, atk_queue, state),
            Na6 => self.na6(time, event, data, atk_queue, state),
        };
    }

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {}

    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State) -> () {}
    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State) -> () {}
    fn hold(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State) -> () {}
    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State) -> () {}
    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State) -> () {}
    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State) -> () {}
    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State) -> () {}
    fn na5(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State) -> () {}
    fn na6(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State) -> () {}
    fn ca(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State) -> () {}
}

pub trait WeaponAttack {
    fn attack(&mut self, time: f32, event: &CharacterAction, atk_queue: &mut Vec<Attack>, state: &mut State) -> () {}

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {}
}
