use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, WeaponAttack};
use crate::sim2::types::{DamageType, CharacterAction, WeaponType, FieldEnergy, Vision};
use crate::sim2::element::{ElementalGauge, ElementalReactionType, ElementalReaction, PHYSICAL_GAUGE};
use crate::sim2::record::{CharacterData, WeaponRecord, Enemy};

use WeaponType::*;
// use Vision::*;

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
            // .skill_dmg(18.0).burst_dmg(18.0)
            // .skill_dmg(24.0).burst_dmg(24.0)
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
                // 4 => state.atk += 72.0,
                // 3 => state.atk += 45.0,
                // 2 => state.atk += 30.0,
                // 1 => state.atk += 15.0,
                // 4 => state.atk += 96.0,
                // 3 => state.atk += 60.0,
                // 2 => state.atk += 40.0,
                // 1 => state.atk += 20.0,
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
    pub fn record() -> WeaponRecord { PolarStar::record().type_(Sword).name("Polar Star (Sword)").version(99.) }
    pub fn new() -> Self { Self(PolarStar::new()) }
}
impl Timeline for PolarStarSword {}
impl WeaponAttack for PolarStarSword {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () { self.0.modify(action_state, data, attack, state, enemy); }
    fn reset_modify(&mut self) -> () { self.0.reset_modify(); }
}

pub struct PolarStarClaymore(PolarStar);
impl PolarStarClaymore {
    pub fn record() -> WeaponRecord { PolarStar::record().type_(Claymore).name("Polar Star (Claymore)").version(99.) }
    pub fn new() -> Self { Self(PolarStar::new()) }
}
impl Timeline for PolarStarClaymore {}
impl WeaponAttack for PolarStarClaymore {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () { self.0.modify(action_state, data, attack, state, enemy); }
    fn reset_modify(&mut self) -> () { self.0.reset_modify(); }
}

pub struct PolarStarPolearm(PolarStar);
impl PolarStarPolearm {
    pub fn record() -> WeaponRecord { PolarStar::record().type_(Polearm).name("Polar Star (Polearm)").version(99.) }
    pub fn new() -> Self { Self(PolarStar::new()) }
}
impl Timeline for PolarStarPolearm {}
impl WeaponAttack for PolarStarPolearm {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () { self.0.modify(action_state, data, attack, state, enemy); }
    fn reset_modify(&mut self) -> () { self.0.reset_modify(); }
}

pub struct PolarStarCatalyst(PolarStar);
impl PolarStarCatalyst {
    pub fn record() -> WeaponRecord { PolarStar::record().type_(Catalyst).name("Polar Star (Catalyst)").version(99.) }
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

#[derive(Debug)]
pub struct MouunsMoon {}

impl MouunsMoon {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Mouun's Moon").type_(Bow).version(2.2)
            .base_atk(565.0)
            .atk(27.6)
            .burst_dmg(80.)
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Timeline for MouunsMoon {}
impl WeaponAttack for MouunsMoon {}

#[derive(Debug)]
pub struct WavebreakersFin {}

impl WavebreakersFin {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Wavebreaker's Fin").type_(Polearm).version(2.2)
            .base_atk(620.)
            .atk(13.8)
            .burst_dmg(80.)
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Timeline for WavebreakersFin {}
impl WeaponAttack for WavebreakersFin {}
