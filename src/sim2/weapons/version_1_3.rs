use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, WeaponAttack};
use crate::sim2::types::{DamageType, CharacterAction, WeaponType, FieldEnergy, Vision};
use crate::sim2::element::{ElementalGauge, ElementalReactionType, ElementalReaction};
use crate::sim2::record::{CharacterData, WeaponRecord, Enemy};

use WeaponType::*;
// use Vision::*;

pub struct ProtectorsVirtue {
}

impl ProtectorsVirtue {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Timeline for ProtectorsVirtue {}

impl WeaponAttack for ProtectorsVirtue {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx {
            let hp = state.HP();
            state.flat_atk += hp * 0.012;
        }
    }
}

pub struct RecklessCinnabar {
}

impl RecklessCinnabar {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Timeline for RecklessCinnabar {}

impl WeaponAttack for RecklessCinnabar {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx {
            let hp = state.HP();
            state.flat_atk += hp * 0.018;
        }
    }
}

pub struct PrimordialJadeCutter(ProtectorsVirtue);

impl PrimordialJadeCutter {
    pub fn new() -> Self {
        Self(ProtectorsVirtue::new())
    }
}

impl PrimordialJadeCutter {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Primordial Jade Cutter").type_(Sword).version(1.3)
            .base_atk(542.0)
            .hp(20.0).cr(44.1)
    }
}

impl Timeline for PrimordialJadeCutter {}

impl WeaponAttack for PrimordialJadeCutter {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        self.0.modify(action_state, data, attack, state, enemy);
    }
}

pub struct PrimordialJadeGS(ProtectorsVirtue);

impl PrimordialJadeGS {
    pub fn new() -> Self {
        Self(ProtectorsVirtue::new())
    }
}

impl PrimordialJadeGS {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("PrimordialJadeGS").type_(Claymore).version(99.0)
            .base_atk(542.0)
            .hp(20.0).cr(44.1)
    }
}

impl Timeline for PrimordialJadeGS {}

impl WeaponAttack for PrimordialJadeGS {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        self.0.modify(action_state, data, attack, state, enemy);
    }
}

pub struct PrimordialJadeVista(ProtectorsVirtue);

impl PrimordialJadeVista {
    pub fn new() -> Self {
        Self(ProtectorsVirtue::new())
    }
}

impl PrimordialJadeVista {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("PrimordialJadeVista").type_(Bow).version(99.0)
            .base_atk(542.0)
            .hp(20.0).cr(44.1)
    }
}

impl Timeline for PrimordialJadeVista {}

impl WeaponAttack for PrimordialJadeVista {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        self.0.modify(action_state, data, attack, state, enemy);
    }
}

pub struct StaffOfHoma(RecklessCinnabar);

impl StaffOfHoma {
    pub fn new() -> Self {
        Self(RecklessCinnabar::new())
    }
}

impl StaffOfHoma {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Staff of Homa").type_(Polearm).version(1.3)
            .base_atk(608.0)
            .hp(20.0).cd(66.2)
    }
}

impl Timeline for StaffOfHoma {}

impl WeaponAttack for StaffOfHoma {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        self.0.modify(action_state, data, attack, state, enemy);
    }
}

pub struct LithicSpear;

impl LithicSpear {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Lithic Spear").type_(Polearm).version(1.3)
            .base_atk(565.0)
            .atk(27.6 + 11.0).cr(0.0 + 7.0)
    }
}

impl Timeline for LithicSpear {}

impl WeaponAttack for LithicSpear {}

pub struct LithicBlade;

impl LithicBlade {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Lithic Blade").type_(Claymore).version(1.3)
            .base_atk(510.0)
            .atk(41.3 + 11.0).cr(0.0 + 7.0)
    }
}

impl Timeline for LithicBlade {}

impl WeaponAttack for LithicBlade {}
