use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, WeaponAttack};
use crate::sim2::types::{DamageType, CharacterAction, WeaponType, FieldEnergy, Vision, Particle, VecFieldEnergy};
use crate::sim2::element::{ElementalGauge, ElementalReactionType, ElementalReaction};
use crate::sim2::record::{CharacterData, WeaponRecord, Enemy};

use WeaponType::*;
// use Vision::*;

// CRIT hits have a 100% chance to generate a small amount of Elemental
// Particles, which will regenerate 6 Energy for the character. Can only occur
// once every 6s.
pub struct Windfall {
    time: f32,
}

impl Windfall {
    pub fn new() -> Self {
        Self {
            time: -99.
        }
    }
}

impl Timeline for Windfall {
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        if *event != CharacterAction::StandStill && state.current_time - self.time >= 6. {
            self.time = state.current_time;
            field_energy.push_p(Particle::neutral(3.));
        }
    }

    fn reset_timeline(&mut self) -> () {
        self.time = -99.;
    }
}

impl WeaponAttack for Windfall {}

pub struct FavoniusGreatswordR5(Windfall);

impl FavoniusGreatswordR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Favonius Greatsword").type_(Claymore).version(1.0)
            .base_atk(454.0)
            .er(61.3)
    }

    pub fn new() -> Self {
        Self(Windfall::new())
    }
}

impl Timeline for FavoniusGreatswordR5 {
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        self.0.accelerate(field_energy, event, state, data);
    }

    fn reset_timeline(&mut self) -> () { self.0.reset_timeline(); }
}

impl WeaponAttack for FavoniusGreatswordR5 {}

pub struct FavoniusSwordR5(Windfall);

impl FavoniusSwordR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Favonius Sword").type_(Sword).version(1.0)
            .base_atk(454.0)
            .er(61.3)
    }

    pub fn new() -> Self {
        Self(Windfall::new())
    }
}

impl Timeline for FavoniusSwordR5 {
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        self.0.accelerate(field_energy, event, state, data);
    }

    fn reset_timeline(&mut self) -> () { self.0.reset_timeline(); }
}

impl WeaponAttack for FavoniusSwordR5 {}

pub struct FavoniusLanceR5(Windfall);

impl FavoniusLanceR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Favonius Lance").type_(Polearm).version(1.0)
            .base_atk(565.0)
            .er(30.6)
    }

    pub fn new() -> Self {
        Self(Windfall::new())
    }
}

impl Timeline for FavoniusLanceR5 {
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        self.0.accelerate(field_energy, event, state, data);
    }

    fn reset_timeline(&mut self) -> () { self.0.reset_timeline(); }
}

impl WeaponAttack for FavoniusLanceR5 {}

pub struct FavoniusWarbowR5(Windfall);

impl FavoniusWarbowR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Favonius Warbow").type_(Bow).version(1.0)
            .base_atk(454.0)
            .er(61.3)
    }

    pub fn new() -> Self {
        Self(Windfall::new())
    }
}

impl Timeline for FavoniusWarbowR5 {
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        self.0.accelerate(field_energy, event, state, data);
    }

    fn reset_timeline(&mut self) -> () { self.0.reset_timeline(); }
}

impl WeaponAttack for FavoniusWarbowR5 {}

pub struct FavoniusCodexR5(Windfall);

impl FavoniusCodexR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Favonius Codex").type_(Catalyst).version(1.0)
            .base_atk(510.0)
            .er(45.9)
    }

    pub fn new() -> Self {
        Self(Windfall::new())
    }
}

impl Timeline for FavoniusCodexR5 {
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        self.0.accelerate(field_energy, event, state, data);
    }

    fn reset_timeline(&mut self) -> () { self.0.reset_timeline(); }
}

impl WeaponAttack for FavoniusCodexR5 {}
