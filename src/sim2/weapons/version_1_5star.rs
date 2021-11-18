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
    multiplier: f32,
}

impl SkywardBlade {
    pub fn record(refinement: usize) -> WeaponRecord {
        let (cr, name) = match refinement {
            1 => (4., "Skyward Blade"),
            2 => (5., "Skyward Blade (R2)"),
            3 => (6., "Skyward Blade (R3)"),
            4 => (7., "Skyward Blade (R4)"),
            5 => (8., "Skyward Blade (R5)"),
            _ => unreachable!(),
        };
        WeaponRecord::default()
            .name(name).type_(Sword).version(1.0)
            .base_atk(608.0)
            .cr(cr).er(55.1)
    }

    pub fn new(refinement: usize) -> Self {
        Self {
            time: -99.,
            multiplier: match refinement {
                1 => 20.,
                2 => 25.,
                3 => 30.,
                4 => 35.,
                5 => 40.,
                _ => unreachable!(),
            }
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
                multiplier: self.multiplier,
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
    multiplier: f32,
}

impl AquilaFavonia {
    pub fn record(refinement: usize) -> WeaponRecord {
        let (atk, name) = match refinement {
            1 => (20., "Aquila Favonia"),
            2 => (25., "Aquila Favonia (R2)"),
            3 => (30., "Aquila Favonia (R3)"),
            4 => (35., "Aquila Favonia (R4)"),
            5 => (40., "Aquila Favonia (R5)"),
            _ => unreachable!(),
        };
        WeaponRecord::default()
            .name(name).type_(Sword).version(1.0)
            .base_atk(674.0)
            .atk(atk)
            .physical_dmg(41.3)
    }

    pub fn new(refinement: usize) -> Self {
        Self {
            time: -99.,
            multiplier: match refinement {
                1 => 200.,
                2 => 230.,
                3 => 260.,
                4 => 290.,
                5 => 320.,
                _ => unreachable!(),
            }
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
                multiplier: self.multiplier,
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
    multiplier: f32,
}

impl SkywardPride {
    pub fn record(refinement: usize) -> WeaponRecord {
        let (dmg, name) = match refinement {
            1 => (8., "Skyward Pride"),
            2 => (10., "Skyward Pride (R2)"),
            3 => (12., "Skyward Pride (R3)"),
            4 => (14., "Skyward Pride (R4)"),
            5 => (16., "Skyward Pride (R5)"),
            _ => unreachable!(),
        };
        WeaponRecord::default()
            .name(name).type_(Claymore).version(1.0)
            .base_atk(674.0)
            .er(36.8)
            .na_dmg(dmg).ca_dmg(dmg).skill_dmg(dmg).burst_dmg(dmg)
    }

    pub fn new(refinement: usize) -> Self {
        Self {
            stack: 0,
            multiplier: match refinement {
                1 => 80.,
                2 => 100.,
                3 => 120.,
                4 => 140.,
                5 => 160.,
                _ => unreachable!(),
            }
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
                multiplier: self.multiplier,
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
    pub fn record(refinement: usize) -> WeaponRecord {
        let (atk, name) = match refinement {
            1 => (20., "Wolf's Gravestone"),
            2 => (25., "Wolf's Gravestone (R2)"),
            3 => (30., "Wolf's Gravestone (R3)"),
            4 => (35., "Wolf's Gravestone (R4)"),
            5 => (40., "Wolf's Gravestone (R5)"),
            _ => unreachable!(),
        };
        WeaponRecord::default()
            .name(name).type_(Claymore).version(1.0)
            .base_atk(608.0)
            .atk(49.6 + atk)
    }
}

impl Timeline for WolfsGravestone {}

impl WeaponAttack for WolfsGravestone {}

pub struct SkywardSpine {
    time: f32,
    multiplier: f32,
}

impl SkywardSpine {
    pub fn record(refinement: usize) -> WeaponRecord {
        let (cr, name) = match refinement {
            1 => (8., "Skyward Spine"),
            2 => (10., "Skyward Spine (R2)"),
            3 => (12., "Skyward Spine (R3)"),
            4 => (14., "Skyward Spine (R4)"),
            5 => (16., "Skyward Spine (R5)"),
            _ => unreachable!(),
        };
        WeaponRecord::default()
            .name(name).type_(Polearm).version(1.0)
            .base_atk(674.0)
            .cr(cr).er(36.8).atk_spd(12.0)
    }

    pub fn new(refinement: usize) -> Self {
        Self {
            time: -99.,
            multiplier: match refinement {
                1 => 40.,
                2 => 55.,
                3 => 70.,
                4 => 85.,
                5 => 100.,
                _ => unreachable!(),
            }
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
                multiplier: self.multiplier,
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
    atk: f32,
    dmg: f32,
}

impl PrimordialJadeWingedSpear {
    pub fn record(refinement: usize) -> WeaponRecord {
        let name = match refinement {
            1 => "Primordial Jade Winged-Spear",
            2 => "Primordial Jade Winged-Spear (R2)",
            3 => "Primordial Jade Winged-Spear (R3)",
            4 => "Primordial Jade Winged-Spear (R4)",
            5 => "Primordial Jade Winged-Spear (R5)",
            _ => unreachable!(),
        };
        WeaponRecord::default()
            .name(name).type_(Polearm).version(1.0)
            .base_atk(674.0)
            .cr(22.1)
    }

    pub fn new(refinement: usize) -> Self {
        Self {
            stack: 0.,
            time: -99.,
            atk: match refinement {
                1 => 3.2,
                2 => 3.9,
                3 => 4.6,
                4 => 5.3,
                5 => 6.0,
                _ => unreachable!(),
            },
            dmg: match refinement {
                1 => 12.,
                2 => 15.,
                3 => 18.,
                4 => 21.,
                5 => 24.,
                _ => unreachable!(),
            },
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
            state.atk += self.atk * self.stack;
            if self.stack == 7. {
                state.all_dmg += self.dmg;
            }
        }
    }

    fn reset_modify(&mut self) -> () {
        self.stack = 0.;
        self.time = -99.;
    }
}

pub struct SkywardHarp {
    time: f32,
    chance: f32,
    frequency: f32,
}

impl SkywardHarp {
    pub fn record(refinement: usize) -> WeaponRecord {
        let (cd, name) = match refinement {
            1 => (20., "Skyward Harp"),
            2 => (25., "Skyward Harp (R2)"),
            3 => (30., "Skyward Harp (R3)"),
            4 => (35., "Skyward Harp (R4)"),
            5 => (40., "Skyward Harp (R5)"),
            _ => unreachable!(),
        };
        WeaponRecord::default()
            .name(name).type_(Bow).version(1.0)
            .base_atk(674.0)
            .cr(22.1).cd(cd)
    }

    pub fn new(refinement: usize) -> Self {
        Self {
            time: -99.,
            chance: match refinement {
                1 => 60.,
                2 => 70.,
                3 => 80.,
                4 => 90.,
                5 => 100.,
                _ => unreachable!(),
            },
            frequency: match refinement {
                1 => 4.,
                2 => 3.5,
                3 => 3.,
                4 => 2.5,
                5 => 2.,
                _ => unreachable!(),
            },
        }
    }
 }

impl Timeline for SkywardHarp {}

impl WeaponAttack for SkywardHarp {
    fn attack(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        if time - self.time > self.frequency &&
           testutil::chance() < self.chance &&
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
     pub fn record(refinement: usize) -> WeaponRecord {
        let (dmg, name) = match refinement {
            1 => (12. + 8. * 5., "Amos' Bow"),
            2 => (15. + 10. * 5., "Amos' Bow (R2)"),
            3 => (18. + 12. * 5., "Amos' Bow (R3)"),
            4 => (21. + 14. * 5., "Amos' Bow (R4)"),
            5 => (24. + 16. * 5., "Amos' Bow (R5)"),
            _ => unreachable!(),
        };
        WeaponRecord::default()
            .name(name).type_(Bow).version(1.0)
            .base_atk(608.0)
            .atk(49.6)
            .na_dmg(dmg).ca_dmg(dmg)
    }
}

impl Timeline for AmosBow {}

impl WeaponAttack for AmosBow {}

pub struct SkywardAtlas {
    stack: u8,
    start_time: f32,
    last_attack: f32,
    multiplier: f32,
}

impl SkywardAtlas {
    pub fn record(refinement: usize) -> WeaponRecord {
        let (dmg, name) = match refinement {
            1 => (12., "Skyward Atlas"),
            2 => (15., "Skyward Atlas (R2)"),
            3 => (18., "Skyward Atlas (R3)"),
            4 => (21., "Skyward Atlas (R4)"),
            5 => (24., "Skyward Atlas (R5)"),
            _ => unreachable!(),
        };
        WeaponRecord::default()
            .name(name).type_(Catalyst).version(1.0)
            .base_atk(674.0)
            .atk(33.1)
            .pyro_dmg(dmg).cryo_dmg(dmg).hydro_dmg(dmg).electro_dmg(dmg).anemo_dmg(dmg).geo_dmg(dmg).dendro_dmg(dmg)
    }

    pub fn new(refinement: usize) -> Self {
        Self {
            stack: 0,
            start_time: -99.,
            last_attack: -99.,
            multiplier: match refinement {
                1 => 160.,
                2 => 200.,
                3 => 240.,
                4 => 280.,
                5 => 320.,
                _ => unreachable!(),
            }
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
                multiplier: self.multiplier,
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
    dmg: f32,
}

impl LostPrayerToTheSacredWinds {
    pub fn record(refinement: usize) -> WeaponRecord {
        let name = match refinement {
            1 => "Lost Prayer to the Sacred Winds",
            2 => "Lost Prayer to the Sacred Winds (R2)",
            3 => "Lost Prayer to the Sacred Winds (R3)",
            4 => "Lost Prayer to the Sacred Winds (R4)",
            5 => "Lost Prayer to the Sacred Winds (R5)",
            _ => unreachable!(),
        };
        WeaponRecord::default()
            .name(name).type_(Catalyst).version(1.0)
            .base_atk(608.0)
            .cr(33.1)
    }

    pub fn new(refinement: usize) -> Self {
        Self {
            stack: 0.,
            time: -99.,
            dmg: match refinement {
                1 => 8.,
                2 => 10.,
                3 => 12.,
                4 => 14.,
                5 => 16.,
                _ => unreachable!(),
            }
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
            state.elemental_dmg += self.dmg * self.stack;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.stack = 0.;
        self.time = -99.;
    }
}
