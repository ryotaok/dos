use crate::state::State;
use crate::types::{AttackType, WeaponType, FieldEnergy, MILLENNIAL_MOVEMENT_SERIES};
use crate::fc::{FieldCharacterIndex, SpecialAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{Attack, AttackEvent, ICDTimer, DurationTimer};

use AttackType::*;
use WeaponType::*;
// use Vision::*;

pub struct FreedomSworn {
    idx: FieldCharacterIndex,
    timer: DurationTimer,
}

impl FreedomSworn {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Freedom-Sworn").type_(Sword).version(1.6)
            .base_atk(608.0)
            .em(198.0).na_dmg(10.0).ca_dmg(10.0).skill_dmg(10.0).burst_dmg(10.0)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx,
            timer: DurationTimer::new(12.0, &[0.5,0.5, 20.0]),
        }
    }
}

impl SpecialAbility for FreedomSworn {
    fn update(&mut self, time: f32, _event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], _particles: &[FieldEnergy], enemy: &Enemy) -> () {
        let should_update = unsafe {
            attack.iter().any(|&a| (*a).idx == self.idx && enemy.trigger_er(&(*a).element.aura).is_triggered())
        };
        self.timer.update(time, should_update);
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        match (self.timer.ping, self.timer.n) {
            (true, 2) => for data in modifiable_data.iter_mut() {
                if data.state.stacked_buff != MILLENNIAL_MOVEMENT_SERIES {
                    data.state.atk += 20.0;
                    data.state.na_dmg += 16.0;
                    data.state.ca_dmg += 16.0;
                    data.state.stacked_buff.turn_on(&MILLENNIAL_MOVEMENT_SERIES);
                }
            },
            (true, 0) => for data in modifiable_data.iter_mut() {
                if data.state.stacked_buff == MILLENNIAL_MOVEMENT_SERIES {
                    data.state.atk -= 20.0;
                    data.state.na_dmg -= 16.0;
                    data.state.ca_dmg -= 16.0;
                    data.state.stacked_buff.turn_off(&MILLENNIAL_MOVEMENT_SERIES);
                }
            },
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct MitternachtsWaltz {
    idx: FieldCharacterIndex,
    na_timer: DurationTimer,
    skill_timer: DurationTimer,
}

impl MitternachtsWaltz {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Mitternachts Waltz").type_(Bow).version(1.6)
            .base_atk(510.0)
            .physical_dmg(51.7)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx,
            na_timer: DurationTimer::new(5.0, &[0.0]),
            skill_timer: DurationTimer::new(5.0, &[0.0]),
        }
    }
}

impl SpecialAbility for MitternachtsWaltz {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        let check_idx = event.idx == self.idx;
        self.na_timer.update(time, check_idx && event.kind == Na);
        self.skill_timer.update(time, check_idx && (event.kind == PressSkill || event.kind == HoldSkill));
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        let state = &mut modifiable_data[self.idx.0].state;
        match (self.na_timer.ping, self.na_timer.n) {
            (true, 1) => state.skill_dmg += 40.0,
            (true, 0) => state.skill_dmg -= 40.0,
            _ => (),
        }
        match (self.skill_timer.ping, self.skill_timer.n) {
            (true, 1) => state.na_dmg += 40.0,
            (true, 0) => state.na_dmg -= 40.0,
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.na_timer.reset();
        self.skill_timer.reset();
    }
}

pub struct DodocoTales {
    idx: FieldCharacterIndex,
    na_timer: DurationTimer,
    ca_timer: DurationTimer,
}

impl DodocoTales {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Dodoco Tales").type_(Catalyst).version(1.6)
            .base_atk(454.0)
            .atk(55.1)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx,
            na_timer: DurationTimer::new(6.0, &[0.0]),
            ca_timer: DurationTimer::new(6.0, &[0.0]),
        }
    }
}

impl SpecialAbility for DodocoTales {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        let check_idx = event.idx == self.idx;
        self.na_timer.update(time, check_idx && event.kind == Na);
        self.ca_timer.update(time, check_idx && event.kind == Ca);
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        let state = &mut modifiable_data[self.idx.0].state;
        match (self.na_timer.ping, self.na_timer.n) {
            (true, 1) => state.ca_dmg += 32.0,
            (true, 0) => state.ca_dmg -= 32.0,
            _ => (),
        }
        match (self.ca_timer.ping, self.ca_timer.n) {
            (true, 1) => state.atk += 16.0,
            (true, 0) => state.atk -= 16.0,
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.na_timer.reset();
        self.ca_timer.reset();
    }
}
