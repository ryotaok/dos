use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, WeaponAttack};
use crate::sim2::types::{DamageType, CharacterAction, WeaponType, FieldEnergy, Vision};
use crate::sim2::element::{ElementalGauge, ElementalReactionType, ElementalReaction, PHYSICAL_GAUGE};
use crate::sim2::record::{CharacterData, WeaponRecord, Enemy};
use crate::sim2::testutil;

use WeaponType::*;
// use Vision::*;

// version 1.0

pub struct SkywardBlade {
    time: f32,
}

impl SkywardBlade {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Skyward Blade").type_(Sword).version(1.0)
            .base_atk(608.0)
            .cr(4.0).er(55.1)
    }

    pub fn new() -> Self {
        Self {
            time: -99.
        }
    }
}

impl Timeline for SkywardBlade {
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        if event.is_burst() {
            self.time = state.current_time;
        }
        if state.current_time - self.time <= 12. {
            state.atk_spd += 10.;
        }
    }

    fn reset_timeline(&mut self) -> () {
        self.time = -99.;
    }
}

impl WeaponAttack for SkywardBlade {
    fn attack(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        if event.is_burst() {
            self.time = time;
        }
        if time - self.time <= 12. &&
           (event.is_na() || event.is_ca()) {
            atk_queue.push(Attack {
                kind: DamageType::AdditionalAttack,
                multiplier: 20.,
                element: &PHYSICAL_GAUGE,
                aura_application: false,
                time,
                idx: data.idx,
            });
        }
    }

    fn reset_attack(&mut self) -> () {
        self.time = -99.;
    }
}

pub struct AquilaFavonia {
    time: f32,
}

impl AquilaFavonia {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Aquila Favonia").type_(Sword).version(1.0)
            .base_atk(674.0)
            .atk(20.0)
            .physical_dmg(41.3)
    }

    pub fn new() -> Self {
        Self {
            time: -99.
        }
    }

 }
impl Timeline for AquilaFavonia {}

impl WeaponAttack for AquilaFavonia {
    fn attack(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        if time - self.time > 15. &&
           (event.is_na() || event.is_ca()) {
            self.time = time;
            atk_queue.push(Attack {
                kind: DamageType::AdditionalAttack,
                multiplier: 200.,
                element: &PHYSICAL_GAUGE,
                aura_application: false,
                time,
                idx: data.idx,
            });
        }
    }

    fn reset_attack(&mut self) -> () {
        self.time = -99.;
    }
}

pub struct SkywardPride {
    stack: u8,
}

impl SkywardPride {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Skyward Pride").type_(Claymore).version(1.0)
            .base_atk(674.0)
            .er(36.8)
            .na_dmg(8.0).ca_dmg(8.0).skill_dmg(8.0).burst_dmg(8.0)
    }

    pub fn new() -> Self {
        Self {
            stack: 0
        }
    }
 }

impl Timeline for SkywardPride {}

impl WeaponAttack for SkywardPride {
    fn attack(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        if event.is_burst() {
            self.stack = 0;
        }
        if self.stack <= 8 &&
           (event.is_na() || event.is_ca()) {
            self.stack += 1;
            atk_queue.push(Attack {
                kind: DamageType::AdditionalAttack,
                multiplier: 80.,
                element: &PHYSICAL_GAUGE,
                aura_application: false,
                time,
                idx: data.idx,
            });
        }
    }

    fn reset_attack(&mut self) -> () {
        self.stack = 0;
    }
}

pub struct WolfsGravestone;

impl WolfsGravestone {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Wolf's Gravestone").type_(Claymore).version(1.0)
            .base_atk(608.0)
            .atk(49.6 + 20.0)
    }
}

impl Timeline for WolfsGravestone {}

impl WeaponAttack for WolfsGravestone {}

pub struct SkywardSpine {
    time: f32,
}

impl SkywardSpine {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Skyward Spine").type_(Polearm).version(1.0)
            .base_atk(674.0)
            .cr(8.0).er(36.8).atk_spd(12.0)
    }

    pub fn new() -> Self {
        Self {
            time: -99.
        }
    }
 }

impl Timeline for SkywardSpine {}

impl WeaponAttack for SkywardSpine {
    fn attack(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        if time - self.time > 2. &&
           testutil::chance() < 0.5 &&
           (event.is_na() || event.is_ca()) {
            self.time = time;
            atk_queue.push(Attack {
                kind: DamageType::AdditionalAttack,
                multiplier: 40.,
                element: &PHYSICAL_GAUGE,
                aura_application: false,
                time,
                idx: data.idx,
            });
        }
    }

    fn reset_attack(&mut self) -> () {
        self.time = -99.;
    }
}

pub struct PrimordialJadeWingedSpear {
    stack: f32,
    time: f32,
}

impl PrimordialJadeWingedSpear {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Primordial Jade Winged-Spear").type_(Polearm).version(1.0)
            .base_atk(674.0)
            .cr(22.1)
    }

    pub fn new() -> Self {
        Self {
            stack: 0.,
            time: -99.,
        }
    }
 }

impl Timeline for PrimordialJadeWingedSpear {}

impl WeaponAttack for PrimordialJadeWingedSpear {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.current_time - self.time >= 0.3 &&
           (action_state.did_na() || action_state.did_ca() || action_state.did_skill() || action_state.did_burst()) {
            self.time = action_state.current_time;
            self.stack += 1.;
            if self.stack > 7. {
                self.stack = 7.;
            }
        }
        if attack.idx == data.idx && attack.time - self.time <= 6. {
            state.atk += 3.2 * self.stack;
            if self.stack == 7. {
                state.all_dmg += 12.0;
            }
        }
    }

    fn reset_modify(&mut self) -> () {
        self.stack = 0.;
        self.time = -99.;
    }
}

pub struct SkywardHarp {
    time: f32
}

impl SkywardHarp {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Skyward Harp").type_(Bow).version(1.0)
            .base_atk(674.0)
            .cr(22.1).cd(20.0)
    }

    pub fn new() -> Self {
        Self {
            time: -99.
        }
    }
 }

impl Timeline for SkywardHarp {}

impl WeaponAttack for SkywardHarp {
    fn attack(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        if time - self.time > 4. &&
           testutil::chance() < 0.6 &&
           (event.is_na() || event.is_ca() || event.is_skill() || event.is_burst()) {
            self.time = time;
            atk_queue.push(Attack {
                kind: DamageType::AdditionalAttack,
                multiplier: 125.,
                element: &PHYSICAL_GAUGE,
                aura_application: false,
                time,
                idx: data.idx,
            });
        }
    }

    fn reset_attack(&mut self) -> () {
        self.time = -99.;
    }
}

pub struct AmosBow;

impl AmosBow {
     pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Amos' Bow").type_(Bow).version(1.0)
            .base_atk(608.0)
            .atk(49.6)
            .na_dmg(12.0 + 40.0).ca_dmg(12.0 + 40.0)
    }
}

impl Timeline for AmosBow {}

impl WeaponAttack for AmosBow {}

pub struct SkywardAtlas {
    stack: u8,
    start_time: f32,
    last_attack: f32,
}

impl SkywardAtlas {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Skyward Atlas").type_(Catalyst).version(1.0)
            .base_atk(674.0)
            .atk(33.1)
            .pyro_dmg(12.0).cryo_dmg(12.0).hydro_dmg(12.0).electro_dmg(12.0).anemo_dmg(12.0).geo_dmg(12.0).dendro_dmg(12.0)
    }

    pub fn new() -> Self {
        Self {
            stack: 0,
            start_time: -99.,
            last_attack: -99.,
        }
    }
 }

impl Timeline for SkywardAtlas {}

impl WeaponAttack for SkywardAtlas {
    fn attack(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        if time - self.start_time >= 30. && (event.is_na() || event.is_ca()) {
            self.start_time = time;
            self.stack = 0;
        }
        if self.stack <= 8 &&
           time - self.last_attack >= 2.0 {
            self.stack += 1;
            self.last_attack = time;
            atk_queue.push(Attack {
                kind: DamageType::AdditionalAttack,
                multiplier: 160.,
                element: &PHYSICAL_GAUGE,
                aura_application: false,
                time,
                idx: data.idx,
            });
        }
    }

    fn reset_attack(&mut self) -> () {
        self.start_time = -99.;
        self.last_attack = -99.;
        self.stack = 0;
    }
}

pub struct LostPrayerToTheSacredWinds {
    stack: f32,
    time: f32,
}

impl LostPrayerToTheSacredWinds {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Lost Prayer to the Sacred Winds").type_(Catalyst).version(1.0)
            .base_atk(608.0)
            .cr(33.1)
    }

    pub fn new() -> Self {
        Self {
            stack: 0.,
            time: -99.,
        }
    }
 }

impl Timeline for LostPrayerToTheSacredWinds {}

impl WeaponAttack for LostPrayerToTheSacredWinds {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        // only the attacker can activate the passive
        if data.idx.0 == 0 && action_state.current_time - self.time >= 4. {
            self.time = action_state.current_time;
            self.stack += 1.;
            if self.stack > 4. {
                self.stack = 4.;
            }
        }
        if attack.idx == data.idx && attack.time - self.time <= 8. {
            state.elemental_dmg += 8. * self.stack;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.stack = 0.;
        self.time = -99.;
    }
}
