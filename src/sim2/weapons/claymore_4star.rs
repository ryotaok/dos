use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, WeaponAttack};
use crate::sim2::types::{DamageType, CharacterAction, WeaponType, FieldEnergy, Vision};
use crate::sim2::element::{ElementalGauge, ElementalReactionType, ElementalReaction, PHYSICAL_GAUGE};
use crate::sim2::record::{CharacterData, WeaponRecord, Enemy};
use crate::sim2::testutil;

use WeaponType::*;
// use Vision::*;

// version 1.0

pub struct PrototypeArchaicR5 {
    time: f32,
}

impl PrototypeArchaicR5 {
    pub fn new() -> Self {
        Self {
            time: -99.,
        }
    }

    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Prototype Archaic").type_(Claymore).version(1.0)
            .base_atk(566.0)
            .atk(27.6)
    }
}

impl Timeline for PrototypeArchaicR5 {}

impl WeaponAttack for PrototypeArchaicR5 {
    fn attack(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        if time - self.time > 15. &&
           testutil::chance() < 0.5 &&
           (event.is_na() || event.is_ca()) {
            self.time = time;
            atk_queue.push(Attack {
                kind: DamageType::AdditionalAttack,
                multiplier: 480.,
                element: &PHYSICAL_GAUGE,
                aura_application: false,
                time,
                idx: data.idx,
            });
        }
    }

    fn reset(&mut self) -> () {
        self.time = -99.;
    }
}

pub struct WhiteblindR5 {
    stack: f32,
    time: f32,
}

impl WhiteblindR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Whiteblind").type_(Claymore).version(1.0)
            .base_atk(510.0)
            .def(51.7)
    }

    pub fn new() -> Self {
        Self {
            stack: 0.,
            time: -99.,
        }
    }
}

impl Timeline for WhiteblindR5 {}

impl WeaponAttack for WhiteblindR5 {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_na() || action_state.did_ca() {
            self.time = action_state.current_time;
            self.stack += 1.;
            if self.stack > 4. {
                self.stack = 4.;
            }
        }
        if attack.idx == data.idx && attack.time - self.time <= 6. {
            state.atk += 12. * self.stack;
            state.def += 12. * self.stack;
        }
    }

    fn reset(&mut self) -> () {
        self.stack = 0.;
        self.time = -99.;
    }
}

pub struct SerpentSpineR5 {
    stack: f32,
    time: f32,
}

impl SerpentSpineR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Serpent Spine").type_(Claymore).version(1.0)
            .base_atk(510.0)
            .cr(27.6)
    }

    pub fn new() -> Self {
        Self {
            stack: 0.,
            time: -99.,
        }
    }
}

impl Timeline for SerpentSpineR5 {}

impl WeaponAttack for SerpentSpineR5 {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.current_time - self.time >= 4. {
            self.time = action_state.current_time;
            self.stack += 1.;
            if self.stack > 5. {
                self.stack = 5.;
            }
        }
        if attack.idx == data.idx && attack.time - self.time <= 8. {
            state.all_dmg += 10. * self.stack;
        }
    }

    fn reset(&mut self) -> () {
        self.stack = 0.;
        self.time = -99.;
    }
}

// one stack is always active
pub struct BlackcliffSlasherR5;

impl Timeline for BlackcliffSlasherR5 {}

impl WeaponAttack for BlackcliffSlasherR5 {}

impl BlackcliffSlasherR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Blackcliff Slasher").type_(Claymore).version(1.0)
            .base_atk(510.0)
            .atk(24.0).cd(55.1)
    }
}

pub struct RoyalGreatswordR5;

impl Timeline for RoyalGreatswordR5 {}

impl WeaponAttack for RoyalGreatswordR5 {}

impl RoyalGreatswordR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Royal Greatsword").type_(Claymore).version(1.0)
            .base_atk(565.0)
            .atk(27.6)
    }
}

pub struct RainslasherR5 {
}

impl RainslasherR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Rainslasher").type_(Claymore).version(1.0)
            .base_atk(510.0)
            .em(165.0)
    }

    pub fn new() -> Self {
        Self {
        }
    }
}

impl Timeline for RainslasherR5 {}

impl WeaponAttack for RainslasherR5 {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx && (enemy.aura.aura == Vision::Electro || enemy.aura.aura == Vision::Hydro) {
            state.all_dmg += 36.0;
        }
    }
}

// The Bell
