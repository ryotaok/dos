use crate::sim2::types::{CharacterAction, DamageType, FieldCharacterIndex};
use crate::sim2::timeline::ActionState;
use crate::sim2::state::State;
use crate::sim2::element::{ElementalGauge, ElementalReactionType, ElementalReaction};
use crate::sim2::record::{CharacterData, Enemy};

#[derive(Debug)]
pub struct DamageResult {
    pub name: &'static str,
    pub kind: DamageType,
    pub time: f32,

    // // outgoing power
    // pub atk: f32,
    // pub bonus: f32,
    // pub crcd: f32,
    // pub multiplier: f32,
    // // incoming power
    // pub defense: f32,
    // pub resistance: f32,

    // outgoing and incoming damage
    pub damage: f32,

    // damage of reaction
    pub reaction: f32,
}

impl DamageResult {
    pub fn new(attack: Attack, state: &State, data: &CharacterData, enemy: &mut Enemy) -> Self {
        if attack.aura_application {
            Self::reaction(attack, state, data, enemy)
        } else {
            Self::without_reaction(attack, state, data, enemy)
        }
    }

    pub fn reaction(attack: Attack, state: &State, data: &CharacterData, enemy: &mut Enemy) -> Self {
        use ElementalReactionType::*;
        let atk = attack.atk(state, data.character.name);
        let bonus = attack.bonus(state);
        let crcd = state.CRCD();
        let multiplier = attack.multiplier(state);
        let defense = attack.defense(&enemy);
        let resistance = attack.resistance(&enemy);
        let damage = atk * bonus * crcd * multiplier * defense * resistance;
        let elemental_reaction = ElementalReaction::new(enemy.aura.aura, attack.element.aura);
        let reaction = match elemental_reaction {
            Overloaded(ref er) |
            Shatter(ref er) |
            ElectorCharged(ref er) |
            Superconduct(ref er) |
            Swirl(ref er) => enemy.resistance(attack.time, &er.attack) * er.transformative_reaction(state.em, state.transformative_bonus),
            Vaporize(ref er) |
            Melt(ref er) => damage * er.amplifying_reaction(state.em, state.amplifying_bonus),
            Crystallize(_) |
            Equalize(_) |
            Freeze(_) |
            Burn(_) |
            Neutralize(_) => 0.,
        };
        enemy.undergo_reaction(&attack, &elemental_reaction);
        let Attack { kind, time, idx, .. } = attack;
        Self {
            name: data.character.name,
            kind, time,
            damage, reaction,
        }
    }

    pub fn without_reaction(attack: Attack, state: &State, data: &CharacterData, enemy: &mut Enemy) -> Self {
        let atk = attack.atk(state, data.character.name);
        let bonus = attack.bonus(state);
        let crcd = state.CRCD();
        let multiplier = attack.multiplier(state);
        let defense = attack.defense(&enemy);
        let resistance = attack.resistance(&enemy);
        let Attack { kind, time, idx, .. } = attack;
        Self {
            name: data.character.name,
            kind, time,
            damage: atk * bonus * crcd * multiplier * defense * resistance,
            reaction: 0.
        }
    }

    pub fn total_damage(&self) -> f32 {
        self.damage + self.reaction
    }
}

pub trait DamageResultUtil {
    fn total_damage(&self) -> f32;
}

impl DamageResultUtil for Vec<DamageResult> {
    fn total_damage(&self) -> f32 {
        let mut total = 0.;
        for i in self.iter() {
            total += i.damage + i.reaction;
        }
        total
    }
}

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
    pub fn atk(&self, state: &State, name: &str) -> f32 {
        state.flat_dmg + match (name, &self.kind) {
            ("Albedo", DamageType::Skill) |
            ("Noelle", DamageType::Skill) => state.DEF(),
            _ => state.ATK(),
        }
    }

    pub fn bonus(&self, state: &State) -> f32 {
        state.DMG_bonus(&self.kind, &self.element.aura)
    }

    pub fn multiplier(&self, state: &State) -> f32 {
        self.multiplier / 100.0 * state.get_talent_bonus(&self.kind)
    }

    pub fn defense(&self, enemy: &Enemy) -> f32 {
        let def_down = 1.0 - enemy.def_down / 100.0;
        enemy.level / (enemy.level * def_down + enemy.level)
    }

    pub fn resistance(&self, enemy: &Enemy) -> f32 {
        enemy.resistance(self.time, &self.element.aura)
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
            Ca(_) => self.ca(time, event, data, atk_queue, state, enemy),
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

    fn reset_attack(&mut self) -> () {}

    // `ActionState` is the state of this character
    // `Attack` and `State` can be owned by this character or the others
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {}

    fn reset_modify(&mut self) -> () {}
}

pub trait WeaponAttack {
    fn attack(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {}

    fn reset_attack(&mut self) -> () {}

    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {}

    fn reset_modify(&mut self) -> () {}
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
