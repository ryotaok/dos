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
    atk: f32,
}

impl GoldenMajesty {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .version(1.1)
            .base_atk(608.0)
            .atk(49.6)
    }

    pub fn new(refinement: usize) -> Self {
        Self {
            stack: 0.,
            time: -99.,
            atk: match refinement {
                1 => 8.,
                2 => 10.,
                3 => 12.,
                4 => 14.,
                5 => 16.,
                _ => unreachable!(),
            }
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
            state.atk += self.atk * self.stack;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.stack = 0.;
        self.time = -99.;
    }
}

pub struct TheUnforged(GoldenMajesty);

impl TheUnforged {
    pub fn record(refinement: usize) -> WeaponRecord {
        let name = match refinement {
            1 => "The Unforged",
            2 => "The Unforged (R2)",
            3 => "The Unforged (R3)",
            4 => "The Unforged (R4)",
            5 => "The Unforged (R5)",
            _ => unreachable!(),
        };
        GoldenMajesty::record().name(name).type_(Claymore)
    }

    pub fn new(refinement: usize) -> Self {
        Self(GoldenMajesty::new(refinement))
    }
}

impl Timeline for TheUnforged {}

impl WeaponAttack for TheUnforged {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        self.0.modify(action_state, data, attack, state, enemy);
    }

    fn reset_modify(&mut self) -> () { self.0.reset_modify(); }
}

pub struct SummitShaper(GoldenMajesty);

impl SummitShaper {
    pub fn record(refinement: usize) -> WeaponRecord {
        let name = match refinement {
            1 => "Summit Shaper",
            2 => "Summit Shaper (R2)",
            3 => "Summit Shaper (R3)",
            4 => "Summit Shaper (R4)",
            5 => "Summit Shaper (R5)",
            _ => unreachable!(),
        };
        GoldenMajesty::record().name(name).type_(Sword)
    }

    pub fn new(refinement: usize) -> Self {
        Self(GoldenMajesty::new(refinement))
    }
}

impl Timeline for SummitShaper {}

impl WeaponAttack for SummitShaper {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        self.0.modify(action_state, data, attack, state, enemy);
    }

    fn reset_modify(&mut self) -> () { self.0.reset_modify(); }
}

pub struct VortexVanquisher(GoldenMajesty);

impl VortexVanquisher {
    pub fn record(refinement: usize) -> WeaponRecord {
        let name = match refinement {
            1 => "Vortex Vanquisher",
            2 => "Vortex Vanquisher (R2)",
            3 => "Vortex Vanquisher (R3)",
            4 => "Vortex Vanquisher (R4)",
            5 => "Vortex Vanquisher (R5)",
            _ => unreachable!(),
        };
        GoldenMajesty::record().name(name).type_(Polearm)
    }

    pub fn new(refinement: usize) -> Self {
        Self(GoldenMajesty::new(refinement))
    }
}

impl Timeline for VortexVanquisher {}

impl WeaponAttack for VortexVanquisher {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        self.0.modify(action_state, data, attack, state, enemy);
    }

    fn reset_modify(&mut self) -> () { self.0.reset_modify(); }
}

pub struct MemoryOfDust(GoldenMajesty);

impl MemoryOfDust {
    pub fn record(refinement: usize) -> WeaponRecord {
        let name = match refinement {
            1 => "Memory of Dust",
            2 => "Memory of Dust (R2)",
            3 => "Memory of Dust (R3)",
            4 => "Memory of Dust (R4)",
            5 => "Memory of Dust (R5)",
            _ => unreachable!(),
        };
        GoldenMajesty::record().name(name).type_(Catalyst)
    }

    pub fn new(refinement: usize) -> Self {
        Self(GoldenMajesty::new(refinement))
    }
}

impl Timeline for MemoryOfDust {}

impl WeaponAttack for MemoryOfDust {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        self.0.modify(action_state, data, attack, state, enemy);
    }

    fn reset_modify(&mut self) -> () { self.0.reset_modify(); }
}
