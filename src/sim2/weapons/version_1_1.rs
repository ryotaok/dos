use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, WeaponAttack};
use crate::sim2::types::{DamageType, CharacterAction, WeaponType, FieldEnergy, Vision};
use crate::sim2::element::{ElementalGauge, ElementalReactionType, ElementalReaction};
use crate::sim2::record::{CharacterData, WeaponRecord, Enemy};

use WeaponType::*;
// use Vision::*;

pub struct GoldenMajesty {
    stack: f32,
    time: f32,
}

impl GoldenMajesty {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .version(1.1)
            .base_atk(608.0)
            .atk(49.6)
    }

    pub fn new() -> Self {
        Self {
            stack: 0.,
            time: -99.,
        }
    }
}

impl Timeline for GoldenMajesty {}

impl WeaponAttack for GoldenMajesty {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.current_time - self.time >= 0.3 &&
           (action_state.did_na() || action_state.did_ca() || action_state.did_skill() || action_state.did_burst()) {
            self.time = action_state.current_time;
            self.stack += 1.;
            if self.stack > 5. {
                self.stack = 5.;
            }
        }
        if attack.idx == data.idx && attack.time - self.time <= 8. {
            state.atk += 8. * self.stack;
        }
    }

    fn reset(&mut self) -> () {
        self.stack = 0.;
        self.time = -99.;
    }
}

pub struct TheUnforged(GoldenMajesty);

impl TheUnforged {
    pub fn record() -> WeaponRecord {
        GoldenMajesty::record().name("The Unforged").type_(Claymore)
    }

    pub fn new() -> Self {
        Self(GoldenMajesty::new())
    }
}

impl Timeline for TheUnforged {}

impl WeaponAttack for TheUnforged {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        self.0.modify(action_state, data, attack, state, enemy);
    }

    fn reset(&mut self) -> () { WeaponAttack::reset(&mut self.0) }
}

pub struct SummitShaper(GoldenMajesty);

impl SummitShaper {
    pub fn record() -> WeaponRecord {
        GoldenMajesty::record().name("Summit shaper").type_(Sword)
    }

    pub fn new() -> Self {
        Self(GoldenMajesty::new())
    }
}

impl Timeline for SummitShaper {}

impl WeaponAttack for SummitShaper {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        self.0.modify(action_state, data, attack, state, enemy);
    }

    fn reset(&mut self) -> () { WeaponAttack::reset(&mut self.0) }
}

pub struct VortexVanquisher(GoldenMajesty);

impl VortexVanquisher {
    pub fn record() -> WeaponRecord {
        GoldenMajesty::record().name("Vortex Vanquisher").type_(Polearm)
    }

    pub fn new() -> Self {
        Self(GoldenMajesty::new())
    }
}

impl Timeline for VortexVanquisher {}

impl WeaponAttack for VortexVanquisher {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        self.0.modify(action_state, data, attack, state, enemy);
    }

    fn reset(&mut self) -> () { WeaponAttack::reset(&mut self.0) }
}

pub struct MemoryOfDust(GoldenMajesty);

impl MemoryOfDust {
    pub fn record() -> WeaponRecord {
        GoldenMajesty::record().name("Memory of Dust").type_(Catalyst)
    }

    pub fn new() -> Self {
        Self(GoldenMajesty::new())
    }
}

impl Timeline for MemoryOfDust {}

impl WeaponAttack for MemoryOfDust {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        self.0.modify(action_state, data, attack, state, enemy);
    }

    fn reset(&mut self) -> () { WeaponAttack::reset(&mut self.0) }
}
