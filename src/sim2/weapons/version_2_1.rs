use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, WeaponAttack};
use crate::sim2::types::{DamageType, CharacterAction, WeaponType, FieldEnergy, Vision};
use crate::sim2::element::{ElementalGauge, ElementalReactionType, ElementalReaction, PHYSICAL_GAUGE};
use crate::sim2::record::{CharacterData, WeaponRecord, Enemy};

use WeaponType::*;
// use Vision::*;

pub struct EngulfingLightning {
    time: f32,
}

impl EngulfingLightning {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Engulfing Lightning").type_(Polearm).version(2.1)
            .base_atk(608.0)
            .er(55.1)
    }

    pub fn new() -> Self {
        Self {
            time: -99.
        }
    }
}

impl Timeline for EngulfingLightning {
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        if *event == CharacterAction::Burst {
            self.time = state.current_time;
        }
        if state.current_time - self.time <= 12. {
            state.er += 30.;
        }
    }

    fn reset_timeline(&mut self) -> () {
        self.time = -99.;
    }
}

impl WeaponAttack for EngulfingLightning {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx {
            state.atk += 0.28 * action_state.er;
        }
    }
}

pub struct EverlastingMoonglow {
    time: f32,
}

impl EverlastingMoonglow {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Everlasting Moonglow").type_(Catalyst).version(2.1)
            .base_atk(608.0)
            // TODO healing bonus
            .hp(49.6)
    }

    pub fn new() -> Self {
        Self {
            time: -99.,
        }
    }
}

impl Timeline for EverlastingMoonglow {
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        if event.is_burst() {
            self.time = state.current_time;
        }
        if state.current_time - self.time <= 12. && state.did_na() {
            state.energy += 0.6;
        }
    }

    fn reset_timeline(&mut self) -> () {
        self.time = -99.;
    }
}

impl WeaponAttack for EverlastingMoonglow {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx && attack.kind == DamageType::Na {
            state.flat_atk += 0.01 * state.HP();
        }
    }
}

pub struct LuxuriousSeaLord {}

impl LuxuriousSeaLord {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Luxurious Sea-Lord").type_(Claymore).version(2.1)
            .base_atk(454.0)
            .atk(55.1).burst_dmg(24.0)
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Timeline for LuxuriousSeaLord {}

impl WeaponAttack for LuxuriousSeaLord {
    fn attack(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        if event.is_burst() {
            atk_queue.push(Attack {
                kind: DamageType::AdditionalAttack,
                multiplier: 200.,
                element: &PHYSICAL_GAUGE,
                aura_application: false,
                time,
                idx: data.idx,
            });
        }
    }
}

pub struct TheCatch {
}

impl TheCatch {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("The Catch").type_(Polearm).version(2.1)
            .base_atk(510.0)
            .er(45.9).burst_dmg(32.0)
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Timeline for TheCatch {}

impl WeaponAttack for TheCatch {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx && attack.kind == DamageType::Burst {
            state.cr += 12.;
        }
    }
}

// Elemental Skill and Elemental Burst DMG increased by 12%. After a Normal
// Attack, Charged Attack, Elemental Skill or Elemental Burst hits an opponent,
// 1 stack of Ashen Nightstar will be gained for 12s. When 1/2/3/4 stacks of
// Ashen Nightstar are present, ATK is increased by 10/20/30/48%. The stack of
// Ashen Nightstar created by the Normal Attack, Charged Attack, Elemental Skill
// or Elemental Burst will be counted independently of the others.
pub struct PolarStar {
    na: f32,
    ca: f32,
    skill: f32,
    burst: f32,
}

impl PolarStar {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Polar Star").type_(Bow).version(2.2)
            .base_atk(608.0)
            .cr(33.1)
            .skill_dmg(12.0).burst_dmg(12.0)
    }

    pub fn new() -> Self {
        Self {
            na: -99.,
            ca: -99.,
            skill: -99.,
            burst: -99.,
        }
    }
}

impl Timeline for PolarStar {}

impl WeaponAttack for PolarStar {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx {
            match &attack.kind {
                DamageType::Burst => self.burst = attack.time,
                DamageType::Skill => self.skill = attack.time,
                DamageType::Na => self.na = attack.time,
                DamageType::Ca => self.ca = attack.time,
                _ => (),
            }
            let mut stack = 0;
            if attack.time - self.burst <= 12. {
                stack += 1;
            }
            if attack.time - self.skill <= 12. {
                stack += 1;
            }
            if attack.time - self.na <= 12. {
                stack += 1;
            }
            // if attack.time - self.ca <= 12. {
            //     stack += 1;
            // }
            stack += 1;
            match stack {
                4 => state.atk += 48.0,
                3 => state.atk += 30.0,
                2 => state.atk += 20.0,
                1 => state.atk += 10.0,
                _ => (),
            }
        }
    }

    fn reset_modify(&mut self) -> () {
        self.na = -99.;
        self.ca = -99.;
        self.skill = -99.;
        self.burst = -99.;
    }
}

pub struct PolarStarSword(PolarStar);
impl PolarStarSword {
    pub fn record() -> WeaponRecord { PolarStar::record().type_(Sword).name("Polar Star (Sword)") }
    pub fn new() -> Self { Self(PolarStar::new()) }
}
impl Timeline for PolarStarSword {}
impl WeaponAttack for PolarStarSword {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () { self.0.modify(action_state, data, attack, state, enemy); }
    fn reset_modify(&mut self) -> () { self.0.reset_modify(); }
}

pub struct PolarStarClaymore(PolarStar);
impl PolarStarClaymore {
    pub fn record() -> WeaponRecord { PolarStar::record().type_(Claymore).name("Polar Star (Claymore)") }
    pub fn new() -> Self { Self(PolarStar::new()) }
}
impl Timeline for PolarStarClaymore {}
impl WeaponAttack for PolarStarClaymore {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () { self.0.modify(action_state, data, attack, state, enemy); }
    fn reset_modify(&mut self) -> () { self.0.reset_modify(); }
}

pub struct PolarStarPolearm(PolarStar);
impl PolarStarPolearm {
    pub fn record() -> WeaponRecord { PolarStar::record().type_(Polearm).name("Polar Star (Polearm)") }
    pub fn new() -> Self { Self(PolarStar::new()) }
}
impl Timeline for PolarStarPolearm {}
impl WeaponAttack for PolarStarPolearm {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () { self.0.modify(action_state, data, attack, state, enemy); }
    fn reset_modify(&mut self) -> () { self.0.reset_modify(); }
}

pub struct PolarStarCatalyst(PolarStar);
impl PolarStarCatalyst {
    pub fn record() -> WeaponRecord { PolarStar::record().type_(Catalyst).name("Polar Star (Catalyst)") }
    pub fn new() -> Self { Self(PolarStar::new()) }
}
impl Timeline for PolarStarCatalyst {}
impl WeaponAttack for PolarStarCatalyst {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () { self.0.modify(action_state, data, attack, state, enemy); }
    fn reset_modify(&mut self) -> () { self.0.reset_modify(); }
}

#[derive(Debug)]
pub struct Akuoumaru {}

impl Akuoumaru {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Akuoumaru").type_(Claymore).version(2.2)
            .base_atk(510.0)
            .atk(41.3)
            .burst_dmg(80.)
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Timeline for Akuoumaru {}

impl WeaponAttack for Akuoumaru {}

// DEF is increased by 20%. Normal and Charged Attack DMG is increased by 28% of
// DEF.
#[derive(Debug)]
pub struct RedhornStonethresher {}

impl RedhornStonethresher {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Redhorn Stonethresher").type_(Claymore).version(2.3)
            .base_atk(608.0)
            .cd(66.2)
            .def(20.)
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Timeline for RedhornStonethresher {}

impl WeaponAttack for RedhornStonethresher {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx {
            if attack.kind == DamageType::Na || attack.kind == DamageType::Ca {
                let bonus = 0.28 * state.DEF();
                state.flat_atk += bonus;
            }
            // let bonus = 0.28 * state.DEF();
            // state.na_dmg += bonus;
            // state.ca_dmg += bonus;
        }
    }
}
