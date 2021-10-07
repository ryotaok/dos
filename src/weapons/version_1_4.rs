use crate::state::State;
use crate::types::{AttackType, WeaponType, FieldEnergy, MILLENNIAL_MOVEMENT_SERIES};
use crate::fc::{FieldCharacterIndex, SpecialAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{Attack, AttackEvent, ICDTimer, DurationTimer};

use AttackType::*;
use WeaponType::*;
// use Vision::*;

pub struct ElegyForTheEnd {
    idx: FieldCharacterIndex,
    timer: DurationTimer,
}

impl ElegyForTheEnd {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Elegy for the End").type_(Bow).version(1.4)
            .base_atk(608.0)
            .er(55.1).em(60.0)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx,
            timer: DurationTimer::new(12.0, &[0.2,0.2,0.2,0.2, 20.0]),
        }
    }
}

impl SpecialAbility for ElegyForTheEnd {
    fn update(&mut self, time: f32, _event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        let should_update = unsafe {
            attack.iter().any(|&a| {
                let atk = & *a;
                match atk.kind {
                    PressSkill | HoldSkill | SkillDot | Burst | BurstDot => atk.idx == data.idx,
                    _ => false,
                }
            })
        };
        self.timer.update(time, should_update);
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if self.timer.n == 5 {
            for data in modifiable_data.iter_mut() {
                if data.state.stacked_buff != MILLENNIAL_MOVEMENT_SERIES {
                    data.state.atk += 20.0;
                    data.state.em  += 100.0;
                    data.state.stacked_buff.turn_on(&MILLENNIAL_MOVEMENT_SERIES);
                }
            }
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct TheAlleyFlash;

impl TheAlleyFlash {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("The Alley Flash").type_(Sword).version(1.4)
            .base_atk(620.0)
            .em(55.0)
            .na_dmg(24.0).ca_dmg(24.0).skill_dmg(24.0).burst_dmg(24.0)
    }
}

impl SpecialAbility for TheAlleyFlash {}

pub struct AlleyHunter {
    idx: FieldCharacterIndex,
    timer: DurationTimer,
}

impl AlleyHunter {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Alley Hunter").type_(Bow).version(1.4)
            .base_atk(565.0)
            .atk(27.6)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx,
            timer: DurationTimer::new(8.0, &[4.0,4.0,4.0,4.0,4.0])
        }
    }
}

impl SpecialAbility for AlleyHunter {
    fn update(&mut self, time: f32, _event: &AttackEvent, _data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update(time, true);
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        let state = &mut modifiable_data[self.idx.0].state;
        state.all_dmg += 8.0 * (5 - self.timer.n) as f32;
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

pub struct WineAndSong;

impl WineAndSong {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Wine and Song").type_(Catalyst).version(1.4)
            .base_atk(565.0)
            .atk(0.0 + 40.0).er(30.6)
    }
}

impl SpecialAbility for WineAndSong {}

pub struct WindblumeOde {
    idx: FieldCharacterIndex,
    timer: DurationTimer,
}

impl WindblumeOde {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Windblume Ode").type_(Bow).version(1.4)
            .base_atk(510.0)
            .em(165.0)
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx,
            timer: DurationTimer::new(6.0, &[0.0])
        }
    }
}

impl SpecialAbility for WindblumeOde {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update(time, event.idx == self.idx && (event.kind == PressSkill || event.kind == HoldSkill));
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if self.timer.n > 0 {
            let state = &mut modifiable_data[self.idx.0].state;
            state.atk += 32.0;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}
