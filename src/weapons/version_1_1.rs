use crate::state::State;
use crate::types::{AttackType, WeaponType, FieldEnergy};
use crate::fc::{FieldCharacterIndex, SpecialAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{Attack, AttackEvent, ICDTimer, DurationTimer};

// use AttackType::*;
use WeaponType::*;
// use Vision::*;

pub struct GoldenMajesty {
    idx: FieldCharacterIndex,
    timer: DurationTimer,
}

impl GoldenMajesty {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .version(1.1)
            .base_atk(608.0)
            .atk(49.6)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx,
            timer: DurationTimer::new(8.0, &[0.3,0.3,0.3,0.3,0.3]),
        }
    }
}

impl SpecialAbility for GoldenMajesty {
    fn update(&mut self, time: f32, event: &AttackEvent, _data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update(time, event.idx == self.idx);
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, _enemy: &mut Enemy) -> () {
        let state = &mut modifiable_state[data.idx.0];
        match (self.timer.ping, self.timer.n > 0) {
            (true, true) => state.atk += 8.0,
            (true, false) => state.atk -= 8.0 * self.timer.previous_n as f32,
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct TheUnforged(GoldenMajesty);

impl TheUnforged {
    pub fn record() -> WeaponRecord {
        GoldenMajesty::record().name("The Unforged").type_(Claymore)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self(GoldenMajesty::new(idx))
    }
}

impl SpecialAbility for TheUnforged {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.0.update(time, event, data, attack, particles, enemy);
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, data, enemy);
    }

    fn reset(&mut self) -> () { self.0.reset() }
}

pub struct SummitShaper(GoldenMajesty);

impl SummitShaper {
    pub fn record() -> WeaponRecord {
        GoldenMajesty::record().name("Summit shaper").type_(Sword)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self(GoldenMajesty::new(idx))
    }
}

impl SpecialAbility for SummitShaper {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.0.update(time, event, data, attack, particles, enemy);
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, data, enemy);
    }

    fn reset(&mut self) -> () { self.0.reset() }
}

pub struct VortexVanquisher(GoldenMajesty);

impl VortexVanquisher {
    pub fn record() -> WeaponRecord {
        GoldenMajesty::record().name("Vortex Vanquisher").type_(Polearm)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self(GoldenMajesty::new(idx))
    }
}

impl SpecialAbility for VortexVanquisher {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.0.update(time, event, data, attack, particles, enemy);
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, data, enemy);
    }

    fn reset(&mut self) -> () { self.0.reset() }
}

pub struct MemoryOfDust(GoldenMajesty);

impl MemoryOfDust {
    pub fn record() -> WeaponRecord {
        GoldenMajesty::record().name("Memory of Dust").type_(Catalyst)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self(GoldenMajesty::new(idx))
    }
}

impl SpecialAbility for MemoryOfDust {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.0.update(time, event, data, attack, particles, enemy);
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, data, enemy);
    }

    fn reset(&mut self) -> () { self.0.reset() }
}
