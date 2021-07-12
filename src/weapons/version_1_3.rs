use crate::state::State;
// use crate::types::{AttackType, Vision};
use crate::fc::{SpecialAbility, FieldCharacter, WeaponRecord, Enemy};

// use AttackType::*;
// use Vision::*;

pub struct ProtectorsVirtue;

impl SpecialAbility for ProtectorsVirtue {
    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        modifiable_state[owner_fc.idx.0].flat_atk += owner_fc.state.HP() * 0.012;
    }
}

pub struct RecklessCinnabar;

impl SpecialAbility for RecklessCinnabar {
    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, _enemy: &mut Enemy) -> () {
        modifiable_state[owner_fc.idx.0].flat_atk += owner_fc.state.HP() * 0.018;
    }
}

pub struct PrimordialJadeCutter(ProtectorsVirtue);

impl PrimordialJadeCutter {
    pub fn new() -> Self {
        Self(ProtectorsVirtue)
    }
}

impl SpecialAbility for PrimordialJadeCutter {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Primordial Jade Cutter").type_("Sword").version(1.3)
            .base_atk(542.0)
            .hp(20.0).cr(44.1)
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, owner_fc, enemy);
    }
}

pub struct PrimordialJadeGS(ProtectorsVirtue);

impl PrimordialJadeGS {
    pub fn new() -> Self {
        Self(ProtectorsVirtue)
    }
}

impl SpecialAbility for PrimordialJadeGS {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("PrimordialJadeGS").type_("Claymore").version(99.0)
            .base_atk(542.0)
            .hp(20.0).cr(44.1)
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, owner_fc, enemy);
    }
}

pub struct PrimordialJadeVista(ProtectorsVirtue);

impl PrimordialJadeVista {
    pub fn new() -> Self {
        Self(ProtectorsVirtue)
    }
}

impl SpecialAbility for PrimordialJadeVista {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("PrimordialJadeVista").type_("Bow").version(99.0)
            .base_atk(542.0)
            .hp(20.0).cr(44.1)
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, owner_fc, enemy);
    }
}

pub struct StaffOfHoma(RecklessCinnabar);

impl StaffOfHoma {
    pub fn new() -> Self {
        Self(RecklessCinnabar)
    }
}

impl SpecialAbility for StaffOfHoma {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Staff of Homa").type_("Polearm").version(1.3)
            .base_atk(608.0)
            .hp(20.0).cd(66.2)
    }

    fn modify(&self, modifiable_state: &mut [State], owner_fc: &FieldCharacter, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, owner_fc, enemy);
    }
}

pub struct LithicSpear;

impl SpecialAbility for LithicSpear {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Lithic Spear").type_("Polearm").version(1.3)
            .base_atk(565.0)
            .atk(27.6 + 11.0).cr(0.0 + 7.0)
    }
}

pub struct LithicBlade;

impl SpecialAbility for LithicBlade {
    fn weapon(&self) -> WeaponRecord {
        WeaponRecord::default()
            .name("Lithic Blade").type_("Claymore").version(1.3)
            .base_atk(510.0)
            .atk(41.3 + 11.0).cr(0.0 + 7.0)
    }
}
