use crate::state::State;
use crate::types::{AttackType, WeaponType, FieldEnergy, MILLENNIAL_MOVEMENT_SERIES};
use crate::fc::{SpecialAbility, CharacterData, WeaponRecord, Enemy};
use crate::action::{Attack, AttackEvent, DurationTimer};

use AttackType::*;
use WeaponType::*;
// use Vision::*;

pub struct ElegyForTheEnd {
    timer: DurationTimer,
}

impl ElegyForTheEnd {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Elegy for the End").type_(Bow).version(1.4)
            .base_atk(608.0)
            .er(55.1).em(60.0)
    }

    pub fn new() -> Self {
        Self {
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

    fn modify(&self, modifiable_state: &mut [State], _data: &CharacterData, _enemy: &mut Enemy) -> () {
        match (self.timer.ping, self.timer.n) {
            (true, 4) => for s in modifiable_state.iter_mut() {
                if s.stacked_buff != MILLENNIAL_MOVEMENT_SERIES {
                    s.atk += 20.0;
                    s.em  += 100.0;
                    s.stacked_buff.turn_on(&MILLENNIAL_MOVEMENT_SERIES);
                }
            },
            (true, 0) => for s in modifiable_state.iter_mut() {
                if s.stacked_buff == MILLENNIAL_MOVEMENT_SERIES {
                    s.atk -= 20.0;
                    s.em  -= 100.0;
                    s.stacked_buff.turn_off(&MILLENNIAL_MOVEMENT_SERIES);
                }
            },
            _ => (),
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
    timer: DurationTimer,
}

impl AlleyHunter {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Alley Hunter").type_(Bow).version(1.4)
            .base_atk(565.0)
            .atk(27.6)
    }

    pub fn new() -> Self {
        Self { timer: DurationTimer::new(8.0, &[4.0,4.0,4.0,4.0,4.0]) }
    }
}

impl SpecialAbility for AlleyHunter {
    fn update(&mut self, time: f32, _event: &AttackEvent, _data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update(time, true);
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, _enemy: &mut Enemy) -> () {
        let state = &mut modifiable_state[data.idx.0];
        match (self.timer.ping, self.timer.n) {
            (false, 0) => state.all_dmg += 40.0,
            (true, 0) => (),
            (true, _) => state.all_dmg -= 8.0,
            _ => (),
        }
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
    timer: DurationTimer,
}

impl WindblumeOde {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Windblume Ode").type_(Bow).version(1.4)
            .base_atk(510.0)
            .em(165.0)
    }

    pub fn new() -> Self {
        Self { timer: DurationTimer::new(6.0, &[0.0]) }
    }
}

impl SpecialAbility for WindblumeOde {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update(time, event.idx == data.idx && (event.kind == PressSkill || event.kind == HoldSkill));
    }

    fn modify(&self, modifiable_state: &mut [State], data: &CharacterData, _enemy: &mut Enemy) -> () {
        let state = &mut modifiable_state[data.idx.0];
        match (self.timer.ping, self.timer.n > 0) {
            (true, true) => state.atk += 32.0,
            (true, false) => state.atk -= 32.0,
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}
