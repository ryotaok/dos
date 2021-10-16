use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, WeaponAttack};
use crate::sim2::types::{DamageType, CharacterAction, WeaponType, FieldEnergy, Vision};
use crate::sim2::element::{ElementalGauge, ElementalReactionType, ElementalReaction, PHYSICAL_GAUGE};
use crate::sim2::record::{CharacterData, WeaponRecord, Enemy};

use WeaponType::*;
// use Vision::*;


// version 1.0

pub struct PrototypeStarglitterR5 {
    stack: f32,
    time: f32,
}

impl PrototypeStarglitterR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Prototype Starglitter").type_(Polearm).version(1.0)
            .base_atk(510.0)
            .er(45.9)
    }

    pub fn new() -> Self {
        Self {
            stack: 0.,
            time: -99.,
        }
    }
}

impl WeaponAttack for PrototypeStarglitterR5 {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_skill() {
            self.time = action_state.current_time;
            self.stack += 1.;
            if self.stack > 2. {
                self.stack = 2.;
            }
        }
        if attack.idx == data.idx && attack.time - self.time <= 12. {
            state.na_dmg += 16. * self.stack;
            state.ca_dmg += 16. * self.stack;
        }
    }

    fn reset(&mut self) -> () {
        self.stack = 0.;
        self.time = -99.;
    }
}

pub struct CrescentPikeR5 {
}

impl CrescentPikeR5 {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl CrescentPikeR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Crescent Pike").type_(Polearm).version(1.0)
            .base_atk(566.0)
            .physical_dmg(34.5)
    }
}

impl WeaponAttack for CrescentPikeR5 {
    fn attack(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        if event.is_na() || event.is_ca() {
            atk_queue.push(Attack {
                kind: DamageType::AdditionalAttack,
                multiplier: 40.,
                element: &PHYSICAL_GAUGE,
                aura_application: false,
                time,
                idx: data.idx,
            });
        }
    }
}

pub struct DeathmatchR5;

impl WeaponAttack for DeathmatchR5 {}

impl DeathmatchR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Deathmatch").type_(Polearm).version(1.0)
            .base_atk(454.0)
            .atk(48.0).cr(36.8)
    }
}

// one stack is always active
pub struct BlackcliffPoleR5;

impl WeaponAttack for BlackcliffPoleR5 {}

impl BlackcliffPoleR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Blackcliff Pole").type_(Polearm).version(1.0)
            .base_atk(510.0)
            .atk(24.0).cd(55.1)
    }
}

pub struct RoyalSpearR5;

impl WeaponAttack for RoyalSpearR5 {}

impl RoyalSpearR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Royal Spear").type_(Polearm).version(1.0)
            .base_atk(565.0)
            .atk(27.6)
    }
}

pub struct WhiteTasselR5;

impl WeaponAttack for WhiteTasselR5 {}

impl WhiteTasselR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("White Tassel").type_(Polearm).version(1.0)
            .base_atk(401.0)
            .cr(23.4)
            .na_dmg(48.0)
    }
}

pub struct DragonsBaneR5 {
}

impl DragonsBaneR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Dragon's Bane").type_(Polearm).version(1.0)
            .base_atk(454.0)
            .em(221.0)
    }

    pub fn new() -> Self {
        Self {
        }
    }
}

impl WeaponAttack for DragonsBaneR5 {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx && (enemy.aura.aura == Vision::Pyro || enemy.aura.aura == Vision::Hydro) {
            state.all_dmg += 36.0;
        }
    }
}
