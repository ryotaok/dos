use crate::state::State;
use crate::types::{WeaponType, FieldEnergy};
use crate::fc::{SpecialAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{Attack, AttackEvent, ICDTimer, ElementalAbsorption, NTimer, DurationTimer, ICDTimers};

// use AttackType::*;
use WeaponType::*;
// use Vision::*;

pub struct ProtectorsVirtue(bool);

impl ProtectorsVirtue {
    pub fn new() -> Self {
        Self(true)
    }
}

impl SpecialAbility for ProtectorsVirtue {
    fn update(&mut self, _time: f32, _event: &AttackEvent, _data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        if self.0 {
            self.0 = false;
        }
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.0 {
            modifiable_state[data.idx.0].flat_atk += data.state().HP() * 0.012;
        }
    }

    fn reset(&mut self) -> () {
        self.0 = true;
    }
}

pub struct RecklessCinnabar(bool);

impl RecklessCinnabar {
    pub fn new() -> Self {
        Self(true)
    }
}

impl SpecialAbility for RecklessCinnabar {
    fn update(&mut self, _time: f32, _event: &AttackEvent, _data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        if self.0 {
            self.0 = false;
        }
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.0 {
            modifiable_state[data.idx.0].flat_atk += data.state().HP() * 0.018;
        }
    }

    fn reset(&mut self) -> () {
        self.0 = true;
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

impl SpecialAbility for PrimordialJadeCutter {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.0.update(time, event, data, attack, particles, enemy);
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, data, enemy);
    }

    fn reset(&mut self) -> () {
        self.0.reset();
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

impl SpecialAbility for PrimordialJadeGS {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.0.update(time, event, data, attack, particles, enemy);
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, data, enemy);
    }

    fn reset(&mut self) -> () {
        self.0.reset();
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

impl SpecialAbility for PrimordialJadeVista {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.0.update(time, event, data, attack, particles, enemy);
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, data, enemy);
    }

    fn reset(&mut self) -> () {
        self.0.reset();
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

impl SpecialAbility for StaffOfHoma {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.0.update(time, event, data, attack, particles, enemy);
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, data, enemy);
    }

    fn reset(&mut self) -> () {
        self.0.reset();
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

impl SpecialAbility for LithicSpear {}

pub struct LithicBlade;

impl LithicBlade {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Lithic Blade").type_(Claymore).version(1.3)
            .base_atk(510.0)
            .atk(41.3 + 11.0).cr(0.0 + 7.0)
    }
}

impl SpecialAbility for LithicBlade {}
