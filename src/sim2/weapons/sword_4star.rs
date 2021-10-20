use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, WeaponAttack};
use crate::sim2::types::{DamageType, CharacterAction, WeaponType, FieldEnergy, Vision};
use crate::sim2::element::{ElementalGauge, ElementalReactionType, ElementalReaction, PHYSICAL_GAUGE};
use crate::sim2::record::{CharacterData, WeaponRecord, Enemy};

use WeaponType::*;
// use Vision::*;

// version 1.0

pub struct PrototypeRancourR5 {
    stack: f32,
    time: f32,
}

impl PrototypeRancourR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Prototype Rancour").type_(Sword).version(1.0)
            .base_atk(566.0)
            .physical_dmg(34.5)
    }

    pub fn new() -> Self {
        Self {
            stack: 0.,
            time: -99.,
        }
    }
}

impl Timeline for PrototypeRancourR5 {}

impl WeaponAttack for PrototypeRancourR5 {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_na() || action_state.did_ca() {
            self.time = action_state.current_time;
            self.stack += 1.;
            if self.stack > 4. {
                self.stack = 4.;
            }
        }
        if attack.idx == data.idx && attack.time - self.time <= 6. {
            state.atk += 8. * self.stack;
            state.def += 8. * self.stack;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.stack = 0.;
        self.time = -99.;
    }
}

// iron sting

pub struct TheBlackSwordR5;

impl Timeline for TheBlackSwordR5 {}

impl WeaponAttack for TheBlackSwordR5 {}
impl TheBlackSwordR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("The Black Sword").type_(Sword).version(1.0)
            .base_atk(510.0)
            .cr(27.6)
            .na_dmg(40.0).ca_dmg(40.0)
    }
}

// one stack is always active
pub struct BlackcliffLongswordR5;

impl Timeline for BlackcliffLongswordR5 {}

impl WeaponAttack for BlackcliffLongswordR5 {}
impl BlackcliffLongswordR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Blackcliff Longsword").type_(Sword).version(1.0)
            .base_atk(565.0)
            .atk(24.0).cd(36.8)
    }
}

pub struct RoyalLongswordR5;

impl Timeline for RoyalLongswordR5 {}

impl WeaponAttack for RoyalLongswordR5 {}
impl RoyalLongswordR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Royal Longsword").type_(Sword).version(1.0)
            .base_atk(565.0)
            .atk(27.6).cr(0.0)
    }
}

// the passive is always active
pub struct HarbingerOfDawnR5;

impl Timeline for HarbingerOfDawnR5 {}

impl WeaponAttack for HarbingerOfDawnR5 {}
impl HarbingerOfDawnR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Harbinger of Dawn").type_(Sword).version(1.0)
            .base_atk(401.0)
            .cr(28.0).cd(46.9)
    }
}

pub struct TheFluteR5 {
    stack: u8,
    time: f32,
}

impl TheFluteR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("The Flute").type_(Sword).version(1.0)
            .base_atk(510.0)
            .atk(41.3)
    }

    pub fn new() -> Self {
        Self {
            stack: 0,
            time: -99.
        }
    }
}

impl Timeline for TheFluteR5 {}

impl WeaponAttack for TheFluteR5 {
    fn attack(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        if time - self.time > 0.5 &&
           (event.is_na() || event.is_ca()) {
            self.time = time;
            self.stack += 1;
            if self.stack == 5 {
                self.stack = 0;
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

    fn reset_attack(&mut self) -> () {
        self.stack = 0;
        self.time = -99.;
    }
}

pub struct LionsRoarR5 {
}

impl LionsRoarR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Lion's Roar").type_(Sword).version(1.0)
            .base_atk(510.0)
            .atk(41.3)
    }

    pub fn new() -> Self {
        Self {
        }
    }
}

impl Timeline for LionsRoarR5 {}

impl WeaponAttack for LionsRoarR5 {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx && (enemy.aura.aura == Vision::Electro || enemy.aura.aura == Vision::Pyro) {
            state.all_dmg += 36.0;
        }
    }
}
