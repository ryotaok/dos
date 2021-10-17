use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, WeaponAttack};
use crate::sim2::types::{DamageType, CharacterAction, WeaponType, FieldEnergy, Vision};
use crate::sim2::element::{ElementalGauge, ElementalReactionType, ElementalReaction, PHYSICAL_GAUGE};
use crate::sim2::record::{CharacterData, WeaponRecord, Enemy};

use WeaponType::*;
use Vision::*;

// Increases Elemental Skill DMG by 32% and Elemental Skill CRIT Rate by 12%.
pub struct FesteringDesire {}

impl FesteringDesire {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Festering Desire").type_(Sword).version(1.2)
            .base_atk(510.0)
            .er(45.9).skill_dmg(32.0)
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Timeline for FesteringDesire {}

impl WeaponAttack for FesteringDesire {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx && attack.kind == DamageType::Skill {
            state.cr += 12.;
        }
    }
}

// Hitting an opponent with Normal and Charged Attacks has a 100% chance of
// forming and dropping an Everfrost Icicle above them, dealing 140% AoE ATK
// DMG. Opponents affected by Cryo are dealt 360% ATK DMG instead by the icicle.
// Can only occur once every 10s.
#[derive(Debug)]
pub struct FrostBurial {
    time: f32,
}

impl FrostBurial {
    pub fn new() -> Self {
        Self {
            time: -99.
        }
    }
}

impl Timeline for FrostBurial {}

impl WeaponAttack for FrostBurial {
    fn attack(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        if time - self.time > 10. &&
           (event.is_na() || event.is_ca()) {
            self.time = time;
            atk_queue.push(Attack {
                kind: DamageType::AdditionalAttack,
                multiplier: if enemy.aura.aura == Cryo {
                    360.
                } else {
                    140.
                },
                element: &PHYSICAL_GAUGE,
                aura_application: false,
                time,
                idx: data.idx,
            });
        }
    }

    fn reset(&mut self) -> () {
        self.time = -99.;
    }
}

pub struct SnowTombedStarsilver(FrostBurial);

impl SnowTombedStarsilver {
    pub fn new() -> Self {
        Self(FrostBurial::new())
    }
}

impl SnowTombedStarsilver {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Snow-Tombed Starsilver").type_(Claymore).version(1.2)
            .base_atk(565.0)
            .physical_dmg(34.5)
    }
}

impl Timeline for SnowTombedStarsilver {}

impl WeaponAttack for SnowTombedStarsilver {
    fn attack(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.0.attack(time, event, data, atk_queue, state, enemy);
    }

    fn reset(&mut self) -> () { WeaponAttack::reset(&mut self.0) }
}

pub struct DragonspineSpear(FrostBurial);

impl DragonspineSpear {
    pub fn new() -> Self {
        Self(FrostBurial::new())
    }
}

impl DragonspineSpear {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Dragonspine Spear").type_(Polearm).version(1.2)
            .base_atk(454.0)
            .physical_dmg(69.0)
    }
}

impl Timeline for DragonspineSpear {}

impl WeaponAttack for DragonspineSpear {
    fn attack(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.0.attack(time, event, data, atk_queue, state, enemy);
    }

    fn reset(&mut self) -> () { WeaponAttack::reset(&mut self.0) }
}

pub struct Frostbearer(FrostBurial);

impl Frostbearer {
    pub fn new() -> Self {
        Self(FrostBurial::new())
    }
}

impl Frostbearer {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Frostbearer").type_(Catalyst).version(1.2)
            .base_atk(510.0)
            .atk(41.3)
    }
}

impl Timeline for Frostbearer {}

impl WeaponAttack for Frostbearer {
    fn attack(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.0.attack(time, event, data, atk_queue, state, enemy);
    }

    fn reset(&mut self) -> () { WeaponAttack::reset(&mut self.0) }
}
