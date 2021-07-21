use crate::state::State;
use crate::types::{WeaponType};
use crate::fc::{SpecialAbility, WeaponAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{FullCharacterTimers};

// use AttackType::*;
use WeaponType::*;
// use Vision::*;

pub struct ProtectorsVirtue;

impl SpecialAbility for ProtectorsVirtue {
    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        modifiable_state[data.idx.0].flat_atk += data.state.HP() * 0.012;
    }
}

pub struct RecklessCinnabar;

impl SpecialAbility for RecklessCinnabar {
    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        modifiable_state[data.idx.0].flat_atk += data.state.HP() * 0.018;
    }
}

pub struct PrimordialJadeCutter(ProtectorsVirtue);

impl PrimordialJadeCutter {
    pub fn new() -> Self {
        Self(ProtectorsVirtue)
    }
}

impl WeaponAbility for PrimordialJadeCutter {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Primordial Jade Cutter").type_(Sword).version(1.3)
            .base_atk(542.0)
            .hp(20.0).cr(44.1)
    }
}

impl SpecialAbility for PrimordialJadeCutter {
    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, timers, data, enemy);
    }
}

pub struct PrimordialJadeGS(ProtectorsVirtue);

impl PrimordialJadeGS {
    pub fn new() -> Self {
        Self(ProtectorsVirtue)
    }
}

impl WeaponAbility for PrimordialJadeGS {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("PrimordialJadeGS").type_(Claymore).version(99.0)
            .base_atk(542.0)
            .hp(20.0).cr(44.1)
    }
}

impl SpecialAbility for PrimordialJadeGS {
    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, timers, data, enemy);
    }
}

pub struct PrimordialJadeVista(ProtectorsVirtue);

impl PrimordialJadeVista {
    pub fn new() -> Self {
        Self(ProtectorsVirtue)
    }
}

impl WeaponAbility for PrimordialJadeVista {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("PrimordialJadeVista").type_(Bow).version(99.0)
            .base_atk(542.0)
            .hp(20.0).cr(44.1)
    }
}

impl SpecialAbility for PrimordialJadeVista {
    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, timers, data, enemy);
    }
}

pub struct StaffOfHoma(RecklessCinnabar);

impl StaffOfHoma {
    pub fn new() -> Self {
        Self(RecklessCinnabar)
    }
}

impl WeaponAbility for StaffOfHoma {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Staff of Homa").type_(Polearm).version(1.3)
            .base_atk(608.0)
            .hp(20.0).cd(66.2)
    }
}

impl SpecialAbility for StaffOfHoma {
    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, timers, data, enemy);
    }
}

pub struct LithicSpear;

impl WeaponAbility for LithicSpear {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Lithic Spear").type_(Polearm).version(1.3)
            .base_atk(565.0)
            .atk(27.6 + 11.0).cr(0.0 + 7.0)
    }
}

impl SpecialAbility for LithicSpear {}

pub struct LithicBlade;

impl WeaponAbility for LithicBlade {
    fn record(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Lithic Blade").type_(Claymore).version(1.3)
            .base_atk(510.0)
            .atk(41.3 + 11.0).cr(0.0 + 7.0)
    }
}

impl SpecialAbility for LithicBlade {}
