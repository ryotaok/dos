use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, WeaponAttack};
use crate::sim2::types::{DamageType, CharacterAction, WeaponType, FieldEnergy, Vision};
use crate::sim2::element::{ElementalGauge, ElementalReactionType, ElementalReaction};
use crate::sim2::record::{CharacterData, WeaponRecord, Enemy};

use WeaponType::*;
// use Vision::*;

pub struct Composed {
    time: f32,
}

impl Composed {
    pub fn new() -> Self {
        Self {
            time: -99.,
        }
    }
}

impl WeaponAttack for Composed {}

impl Timeline for Composed {
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        if state.current_time - self.time > 16. && event.is_skill() {
            state.reduce_skill = 99.;
            self.time = state.current_time;
        }
    }

    fn reset(&mut self) -> () {
        self.time = -1.;
    }
}

pub struct SacrificialSwordR5(Composed);

impl SacrificialSwordR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Sacrificial Sword").type_(Sword).version(1.0)
            .base_atk(454.0)
            .er(61.3)
    }

    pub fn new() -> Self {
        Self(Composed::new())
    }
}

impl WeaponAttack for SacrificialSwordR5 {}

impl Timeline for SacrificialSwordR5 {
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        self.0.accelerate(field_energy, event, state, data);
    }

    fn reset(&mut self) -> () {
        WeaponAttack::reset(&mut self.0);
    }
}

pub struct SacrificialGreatswordR5(Composed);

impl SacrificialGreatswordR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Sacrificial Greatsword").type_(Claymore).version(1.0)
            .base_atk(565.0)
            .er(30.6)
    }

    pub fn new() -> Self {
        Self(Composed::new())
    }
}

impl WeaponAttack for SacrificialGreatswordR5 {}

impl Timeline for SacrificialGreatswordR5 {
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        self.0.accelerate(field_energy, event, state, data);
    }

    fn reset(&mut self) -> () {
        WeaponAttack::reset(&mut self.0);
    }
}

// pub struct SacrificialLanceR5(Composed);

pub struct SacrificialBowR5(Composed);

impl SacrificialBowR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Sacrificial Bow").type_(Bow).version(1.0)
            .base_atk(565.0)
            .er(30.6)
    }

    pub fn new() -> Self {
        Self(Composed::new())
    }
}

impl WeaponAttack for SacrificialBowR5 {}

impl Timeline for SacrificialBowR5 {
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        self.0.accelerate(field_energy, event, state, data);
    }

    fn reset(&mut self) -> () {
        WeaponAttack::reset(&mut self.0);
    }
}

pub struct SacrificialFragmentsR5(Composed);

impl SacrificialFragmentsR5 {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Sacrificial Fragments").type_(Catalyst).version(1.0)
            .base_atk(454.0)
            .em(221.0)
    }

    pub fn new() -> Self {
        Self(Composed::new())
    }
}

impl WeaponAttack for SacrificialFragmentsR5 {}

impl Timeline for SacrificialFragmentsR5 {
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        self.0.accelerate(field_energy, event, state, data);
    }

    fn reset(&mut self) -> () {
        WeaponAttack::reset(&mut self.0);
    }
}
