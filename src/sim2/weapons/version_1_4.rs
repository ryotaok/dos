use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, WeaponAttack};
use crate::sim2::types::{DamageType, CharacterAction, WeaponType, FieldEnergy, Vision, MILLENNIAL_MOVEMENT_SERIES};
use crate::sim2::element::{ElementalGauge, ElementalReactionType, ElementalReaction};
use crate::sim2::record::{CharacterData, WeaponRecord, Enemy};

use WeaponType::*;
// use Vision::*;

pub struct ElegyForTheEnd {
    sigil: u8,
    time: f32,
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
            sigil: 0,
            time: -99.,
        }
    }
}

impl Timeline for ElegyForTheEnd {}

impl WeaponAttack for ElegyForTheEnd {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx &&
           (attack.kind == DamageType::Burst || attack.kind == DamageType::Skill) &&
           attack.time - self.time > 20. {
            self.sigil += 1;
            if self.sigil == 4 {
                self.time = attack.time;
                self.sigil = 0;
            }
        }
        if state.stacked_buff != MILLENNIAL_MOVEMENT_SERIES && attack.time - self.time <= 12. {
            state.atk += 20.;
            state.em  += 100.;
            state.stacked_buff.turn_on(&MILLENNIAL_MOVEMENT_SERIES);
        }
    }

    fn reset_modify(&mut self) -> () {
        self.sigil = 0;
        self.time = -99.;
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

impl Timeline for TheAlleyFlash {}

impl WeaponAttack for TheAlleyFlash {}

pub struct AlleyHunter {
    stack: f32,
    time: f32,
}

impl AlleyHunter {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Alley Hunter").type_(Bow).version(1.4)
            .base_atk(565.0)
            .atk(27.6)
    }

    pub fn new() -> Self {
        Self {
            stack: 0.,
            time: 0.,
        }
    }
}

impl Timeline for AlleyHunter {}

impl WeaponAttack for AlleyHunter {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.current_time - self.time > 4. {
            self.stack += 1.;
            self.time = action_state.current_time;
            if self.stack > 5. {
                self.stack = 5.;
            }
        }
        if attack.idx == data.idx {
            state.all_dmg += 8.0 * (5. - self.stack);
        }
    }

    fn reset_modify(&mut self) -> () {
        self.stack = 0.;
        self.time = 0.;
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

impl Timeline for WineAndSong {}

impl WeaponAttack for WineAndSong {}

pub struct WindblumeOde {
    time: f32,
}

impl WindblumeOde {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Windblume Ode").type_(Bow).version(1.4)
            .base_atk(510.0)
            .em(165.0)
    }

    pub fn new() -> Self {
        Self {
            time: -99.
        }
    }
}

impl Timeline for WindblumeOde {}

impl WeaponAttack for WindblumeOde {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_skill() {
            self.time = action_state.current_time;
        }
        if attack.idx == data.idx && attack.time - self.time <= 6. {
            state.atk += 32.0;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.time = -99.
    }
}
