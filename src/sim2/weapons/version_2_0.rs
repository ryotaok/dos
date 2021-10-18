use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, WeaponAttack};
use crate::sim2::types::{DamageType, CharacterAction, WeaponType, FieldEnergy, Vision, near_eq};
use crate::sim2::element::{ElementalGauge, ElementalReactionType, ElementalReaction};
use crate::sim2::record::{CharacterData, WeaponRecord, Enemy};

use WeaponType::*;
// use Vision::*;

// Gain a 12% Elemental DMG Bonus for all elements and receive the might of the
// Mistsplitter's Emblem. At stack levels 1/2/3, the Mistsplitter's Emblem
// provides a 8/16/28% Elemental DMG Bonus for the character's Elemental Type.
// The character will obtain 1 stack of Mistsplitter's Emblem in each of the
// following scenarios: Normal Attack deals Elemental DMG (stack lasts 5s),
// casting Elemental Burst (stack lasts 10s); Energy is less than 100% (stack
// disappears when Energy is full). Each stack's duration is calculated
// independently.
pub struct MistsplitterReforged {
    seal_1: f32,
    seal_2: f32,
}

impl MistsplitterReforged {
    pub fn new() -> Self {
        Self {
            seal_1: -99.,
            seal_2: -99.,
        }
    }
}

impl MistsplitterReforged {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Mistsplitter Reforged").type_(Sword).version(2.0)
            .base_atk(674.0)
            .cd(44.1)
            .pyro_dmg(12.0).cryo_dmg(12.0).hydro_dmg(12.0).electro_dmg(12.0).anemo_dmg(12.0).geo_dmg(12.0).dendro_dmg(12.0)
    }
}

impl Timeline for MistsplitterReforged {}

impl WeaponAttack for MistsplitterReforged {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx {
            let mut seal: u8 = 0;
            if attack.kind == DamageType::Na && attack.element.aura != Vision::Physical {
                self.seal_1 = attack.time;
            }
            if action_state.did_burst() {
                self.seal_2 = attack.time;
            }
            if attack.time - self.seal_1 <= 5. {
                seal += 1;
            }
            if attack.time - self.seal_2 <= 10. {
                seal += 1;
            }
            if action_state.energy / data.character.energy_cost < 1.0 {
                seal += 1;
            }
            match seal {
                3 => state.elemental_dmg += 28.,
                2 => state.elemental_dmg += 16.,
                1 => state.elemental_dmg += 8.,
                _ => (),
            }
        }
    }

    fn reset(&mut self) -> () {
        self.seal_1 = -99.;
        self.seal_2 = -99.;
    }
}

// Increases ATK by 20% and grants the might of the Thunder Emblem. At stack
// levels 1/2/3, the Thunder Emblem increases Normal Attack DMG by 12/24/40%.
// The character will obtain 1 stack of Thunder Emblem in each of the following
// scenarios: Normal Attack deals DMG (stack lasts 5s), casting Elemental Skill
// (stack lasts 10s); Energy is less than 100% (stack disappears when Energy is
// full). Each stack's duration is calculated independently.
pub struct ThunderingPulse {
    seal_1: f32,
    seal_2: f32,
}

impl ThunderingPulse {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Thundering Pulse").type_(Bow).version(2.0)
            .base_atk(608.0)
            .atk(20.0).cd(66.2)
    }

    pub fn new() -> Self {
        Self {
            seal_1: -99.,
            seal_2: -99.,
        }
    }
}

impl Timeline for ThunderingPulse {}

impl WeaponAttack for ThunderingPulse {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx {
            let mut seal: u8 = 0;
            if attack.kind == DamageType::Na {
                self.seal_1 = attack.time;
            }
            if action_state.did_skill() {
                self.seal_2 = attack.time;
            }
            if attack.time - self.seal_1 <= 5. {
                seal += 1;
            }
            if attack.time - self.seal_2 <= 10. {
                seal += 1;
            }
            if action_state.energy / data.character.energy_cost < 1.0 {
                seal += 1;
            }
            match seal {
                3 => state.na_dmg += 40.,
                2 => state.na_dmg += 24.,
                1 => state.na_dmg += 12.,
                _ => (),
            }
        }
    }

    fn reset(&mut self) -> () {
        self.seal_1 = -99.;
        self.seal_2 = -99.;
    }
}

// After casting an Elemental Skill, gain 1 Succession Seed. This effect can be
// triggered once every 5s. The Succession Seed lasts for 30s. Up to 3
// Succession Seeds may exist simultaneously. After using an Elemental Burst,
// all Succession Seeds are consumed and after 2s, the character regenerates 6
// Energy for each seed consumed.
pub struct AmenomaKageuchi {
    time: f32,
    seed: f32,
}

impl AmenomaKageuchi {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Amenoma Kageuchi").type_(Sword).version(2.0)
            .base_atk(454.0)
            .atk(55.1)
    }

    pub fn new() -> Self {
        Self {
            time: -99.,
            seed: 0.,
        }
    }
}

impl WeaponAttack for AmenomaKageuchi {
    fn reset(&mut self) -> () {
        self.seed = 0.;
        self.time = -99.;
    }
}

impl Timeline for AmenomaKageuchi {
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        if state.current_time - self.time >= 5. && event.is_skill() {
            self.time = state.current_time;
            self.seed += 1.;
            if self.seed > 3. {
                self.seed = 3.;
            }
        }
        if event.is_burst() {
            state.energy += 12. * self.seed;
            self.seed = 0.;
        }
    }
}

// Increases Elemental Skill DMG by 6%. After Elemental Skill hits an opponent,
// the character loses 3 Energy but regenerates 3 Energy every 2s for the next
// 6s. This effect can occur once every 10s. Can be triggered even when the
// character is not on the field.
pub struct KatsuragikiriNagamasa {
    time: f32
}

impl KatsuragikiriNagamasa {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Katsuragikiri Nagamasa").type_(Claymore).version(2.0)
            .base_atk(510.0)
            .er(45.9)
            .skill_dmg(12.0)
    }

    pub fn new() -> Self {
        Self {
            time: -99.
        }
    }
}

impl Timeline for KatsuragikiriNagamasa {
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        if state.current_time - self.time >= 10. && event.is_skill() {
            state.energy -= 3.;
            self.time = state.current_time;
        }
        if near_eq(state.current_time, self.time + 2.) ||
           near_eq(state.current_time, self.time + 4.) ||
           near_eq(state.current_time, self.time + 6.) {
            state.energy += 5.;
        }
    }
}

impl WeaponAttack for KatsuragikiriNagamasa {
    fn reset(&mut self) -> () {
        self.time = -99.;
    }
}

// Increases Elemental Skill DMG by 6%. After Elemental Skill hits an opponent,
// the character loses 3 Energy but regenerates 3 Energy every 2s for the next
// 6s. This effect can occur once every 10s. Can be triggered even when the
// character is not on the field.
pub struct KitainCrossSpear {
    time: f32
}

impl KitainCrossSpear {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Kitain Cross Spear").type_(Polearm).version(2.0)
            .base_atk(565.0)
            .em(110.0)
            .skill_dmg(12.0)
    }

    pub fn new() -> Self {
        Self {
            time: -99.
        }
    }
}

impl Timeline for KitainCrossSpear {
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        if state.current_time - self.time >= 10. && event.is_skill() {
            state.energy -= 3.;
            self.time = state.current_time;
        }
        if near_eq(state.current_time, self.time + 2.) ||
           near_eq(state.current_time, self.time + 4.) ||
           near_eq(state.current_time, self.time + 6.) {
            state.energy += 5.;
        }
    }
}

impl WeaponAttack for KitainCrossSpear {
    fn reset(&mut self) -> () {
        self.time = -99.;
    }
}

// Increases Normal Attack DMG by 16% and Charged Attack DMG by 12%. When the
// equipping character's Energy reaches 100%, this effect is increased by 100%.
pub struct Hamayumi {
}

impl Hamayumi {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Hamayumi").type_(Bow).version(2.0)
            .base_atk(454.0)
            .atk(55.1)
            .na_dmg(32.0).ca_dmg(24.0)
    }

    pub fn new() -> Self {
        Self {
        }
    }
}

impl Timeline for Hamayumi {}

impl WeaponAttack for Hamayumi {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx && action_state.energy / data.character.energy_cost >= 1. {
            state.na_dmg += 32.0;
            state.ca_dmg += 24.0;
        }
    }
}

// After the character equipped with this weapon triggers an Electro elemental
// reaction, nearby party members of an Elemental Type involved in the elemental
// reaction receive a 10% Elemental DMG Bonus for their element, lasting 6s.
// Elemental Bonuses gained in this way cannot be stacked.
pub struct HakushinRing {
    time: f32,
}

impl HakushinRing {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Hakushin Ring").type_(Catalyst).version(2.0)
            .base_atk(565.0)
            .er(30.6)
    }

    pub fn new() -> Self {
        Self {
            time: -99.
        }
    }
}

impl Timeline for HakushinRing {}

impl WeaponAttack for HakushinRing {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx &&
           enemy.trigger_er(&attack.element.aura).is_electro() {
            self.time = attack.time;
        }
        if attack.time - self.time <= 6. {
            state.pyro_dmg += 20.;
            state.hydro_dmg += 20.;
            state.electro_dmg += 20.;
            state.cryo_dmg += 20.;
        }
    }

    fn reset(&mut self) -> () {
        self.time = -99.;
    }
}
