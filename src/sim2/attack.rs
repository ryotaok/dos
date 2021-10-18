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
    fn attack(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        use CharacterAction::*;
        match event {
            StandStill => (),
            Burst => self.burst(time, event, data, atk_queue, state, enemy),
            PressSkill => self.press(time, event, data, atk_queue, state, enemy),
            HoldSkill => self.hold(time, event, data, atk_queue, state, enemy),
            Ca => self.ca(time, event, data, atk_queue, state, enemy),
            Na1(_) => self.na1(time, event, data, atk_queue, state, enemy),
            Na2(_) => self.na2(time, event, data, atk_queue, state, enemy),
            Na3(_) => self.na3(time, event, data, atk_queue, state, enemy),
            Na4(_) => self.na4(time, event, data, atk_queue, state, enemy),
            Na5(_) => self.na5(time, event, data, atk_queue, state, enemy),
            Na6(_) => self.na6(time, event, data, atk_queue, state, enemy),
        };
    }

    fn burst(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {}
    fn press(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {}
    fn hold(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {}
    fn na1(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {}
    fn na2(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {}
    fn na3(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {}
    fn na4(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {}
    fn na5(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {}
    fn na6(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {}
    fn ca(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {}

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {}

    fn reset(&mut self) -> () {}
}

pub trait WeaponAttack {
    fn attack(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {}

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {}

    fn reset(&mut self) -> () {}
}

pub trait AtkQueue {
    fn add_burst(&mut self, multiplier: f32, element: &'static ElementalGauge, time: f32, event: &CharacterAction, data: &CharacterData, state: &mut State) -> ();
    fn add_skill(&mut self, multiplier: f32, element: &'static ElementalGauge, time: f32, event: &CharacterAction, data: &CharacterData, state: &mut State) -> ();
    fn add_na(&mut self, multiplier: f32, element: &'static ElementalGauge, time: f32, event: &CharacterAction, data: &CharacterData, state: &mut State) -> ();
    fn add_ca(&mut self, multiplier: f32, element: &'static ElementalGauge, time: f32, event: &CharacterAction, data: &CharacterData, state: &mut State) -> ();
    fn apply_burst(&mut self, multiplier: f32, element: &'static ElementalGauge, time: f32, event: &CharacterAction, data: &CharacterData, state: &mut State) -> ();
    fn apply_skill(&mut self, multiplier: f32, element: &'static ElementalGauge, time: f32, event: &CharacterAction, data: &CharacterData, state: &mut State) -> ();
    fn apply_na(&mut self, multiplier: f32, element: &'static ElementalGauge, time: f32, event: &CharacterAction, data: &CharacterData, state: &mut State) -> ();
    fn apply_ca(&mut self, multiplier: f32, element: &'static ElementalGauge, time: f32, event: &CharacterAction, data: &CharacterData, state: &mut State) -> ();
}

impl AtkQueue for Vec<Attack> {
    fn add_burst(&mut self, multiplier: f32, element: &'static ElementalGauge, time: f32, event: &CharacterAction, data: &CharacterData, state: &mut State) -> () {
        self.push(Attack {
            kind: DamageType::Burst,
            multiplier,
            element,
            aura_application: state.apply_aura(time, event),
            time,
            idx: data.idx,
        });
    }

    fn add_skill(&mut self, multiplier: f32, element: &'static ElementalGauge, time: f32, event: &CharacterAction, data: &CharacterData, state: &mut State) -> () {
        self.push(Attack {
            kind: DamageType::Skill,
            multiplier,
            element,
            aura_application: state.apply_aura(time, event),
            time,
            idx: data.idx,
        });
    }

    fn add_na(&mut self, multiplier: f32, element: &'static ElementalGauge, time: f32, event: &CharacterAction, data: &CharacterData, state: &mut State) -> () {
        self.push(Attack {
            kind: DamageType::Na,
            multiplier,
            element,
            aura_application: state.apply_aura(time, event),
            time,
            idx: data.idx,
        });
    }

    fn add_ca(&mut self, multiplier: f32, element: &'static ElementalGauge, time: f32, event: &CharacterAction, data: &CharacterData, state: &mut State) -> () {
        self.push(Attack {
            kind: DamageType::Ca,
            multiplier,
            element,
            aura_application: state.apply_aura(time, event),
            time,
            idx: data.idx,
        });
    }

    fn apply_burst(&mut self, multiplier: f32, element: &'static ElementalGauge, time: f32, event: &CharacterAction, data: &CharacterData, state: &mut State) -> () {
        self.push(Attack {
            kind: DamageType::Burst,
            multiplier,
            element,
            aura_application: true,
            time,
            idx: data.idx,
        });
    }

    fn apply_skill(&mut self, multiplier: f32, element: &'static ElementalGauge, time: f32, event: &CharacterAction, data: &CharacterData, state: &mut State) -> () {
        self.push(Attack {
            kind: DamageType::Skill,
            multiplier,
            element,
            aura_application: true,
            time,
            idx: data.idx,
        });
    }

    fn apply_na(&mut self, multiplier: f32, element: &'static ElementalGauge, time: f32, event: &CharacterAction, data: &CharacterData, state: &mut State) -> () {
        self.push(Attack {
            kind: DamageType::Na,
            multiplier,
            element,
            aura_application: true,
            time,
            idx: data.idx,
        });
    }

    fn apply_ca(&mut self, multiplier: f32, element: &'static ElementalGauge, time: f32, event: &CharacterAction, data: &CharacterData, state: &mut State) -> () {
        self.push(Attack {
            kind: DamageType::Ca,
            multiplier,
            element,
            aura_application: true,
            time,
            idx: data.idx,
        });
    }
}
