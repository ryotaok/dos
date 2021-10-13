use crate::sim1::state::State;
use crate::sim1::artifact::Artifact;
use crate::sim1::action::{Attack, AttackEvent, ICDTimer, NTimer, ICDTimers, DurationTimer};
use crate::sim1::types::{Vision, WeaponType, FieldEnergy, ElementalGauge, ElementalReaction, ElementalReactionType};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct FieldCharacterIndex(pub usize);

#[allow(unused_variables)]
pub trait SpecialAbility {
    // the variable is named `owner_fc` because `FieldCharacter` will own this
    // `SpecialAbility`.

    fn maybe_attack(&self, data: &CharacterData) -> Option<AttackEvent> { None }

    // Synchronize own timers to the emulator so that cool down times and
    // passive effect duration are up to date. The three methods of
    // `additional_attack`, `modify` and `accelerate` will depend on the data
    // mutated by this self.
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {}

    // `Vec::push` additional attacks created. The `atk_queue` is `Vec` because
    // some character (Eula's hold skill) deals many additional attacks of
    // different elements at once. Similarly, Kazuha (Chihayaburu) deals
    // additional attacks of different kinds.
    // `AdditionalAttack` created by various effects of characters and weapons
    // are another entities who take part in the battle. These entities
    // can also attack an enemy, so we need to know how strong their attacks
    // are, i.e. additional attack DMG (or `Attack.multiplier`).
    fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {}

    // Apply own passive effects to `State`. This is the primary method that
    // "returns" passive effects to `FieldCharacter`. For example, suppose this
    // `SpecialAbility` is about Sucrose A1, "Catalyst Conversion" and she has
    // `FieldCharacterIndex(1)`. Then this method modifies the vector like:
    // 
    //     modifiable_data[0, 2 and 3].em += 50
    // 
    // The `Enemy` is mutable because some passive debuffs an enemy (e.g. DEF
    // down by Lisa).
    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {}

    // This method can change the `State` of `Attack`. For example, Some abilities
    // increase CR of a specific action: Amber A1 (Every Arrow Finds Its
    // Target), Ganyu A1 (Undivided Heart), Festering Desire.
    fn intensify(&self, attack: &Attack) -> Option<State> { None }

    fn accelerator(&self) -> Option<fn(&mut NTimer)> { None }

    fn reset(&mut self) -> () {}
}

#[derive(Debug)]
pub struct NoopAbility;

impl SpecialAbility for NoopAbility {}

#[derive(Debug)]
pub struct NoopSkillAbility;

impl SpecialAbility for NoopSkillAbility {}

impl SkillAbility for NoopSkillAbility {
    fn accelerate(&mut self, _f: fn(&mut NTimer)) -> () {}
}

pub trait CharacterAbility : SpecialAbility {
    fn na_mut(&mut self) -> &mut dyn SpecialAbility;
    fn ca_mut(&mut self) -> &mut dyn SpecialAbility;
    fn skill_mut(&mut self) -> &mut dyn SkillAbility;
    fn burst_mut(&mut self) -> &mut dyn SpecialAbility;
    fn na_ref(&self) -> & dyn SpecialAbility;
    fn ca_ref(&self) -> & dyn SpecialAbility;
    fn skill_ref(&self) -> & dyn SkillAbility;
    fn burst_ref(&self) -> & dyn SpecialAbility;
    // passive -> Self
}

pub trait SkillAbility : SpecialAbility {
    fn accelerate(&mut self, f: fn(&mut NTimer)) -> ();
}

// #[derive(Debug)]
pub struct FieldAbility<'a> {
    pub timers: &'a mut ICDTimers,
    pub character: &'a mut dyn CharacterAbility,
    pub weapon: &'a mut dyn SpecialAbility,
    pub artifact: &'a mut dyn SpecialAbility,
}

impl<'a> FieldAbility<'a> {
    pub fn additional_attack(&self, atk_queue: &mut Vec<*const Attack>, particles: &mut Vec<FieldEnergy>, data: &CharacterData) -> () {
        self.character.na_ref().additional_attack(atk_queue, particles, data);
        self.character.ca_ref().additional_attack(atk_queue, particles, data);
        self.character.skill_ref().additional_attack(atk_queue, particles, data);
        self.character.burst_ref().additional_attack(atk_queue, particles, data);
        self.character.additional_attack(atk_queue, particles, data);
        self.weapon.additional_attack(atk_queue, particles, data);
    }

    pub fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        self.character.modify(modifiable_data, enemy);
        self.weapon.modify(modifiable_data, enemy);
        self.artifact.modify(modifiable_data, enemy);
    }

    pub fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        let speedup_time = time * (1.0 + data.state.atk_spd / 100.0);
        self.timers.update(time);
        self.character.na_mut().update(speedup_time, event, data, attack, particles, enemy);
        self.character.ca_mut().update(speedup_time, event, data, attack, particles, enemy);
        self.character.skill_mut().update(time, event, data, attack, particles, enemy);
        self.character.burst_mut().update(time, event, data, attack, particles, enemy);
        self.character.update(time, event, data, attack, particles, enemy);
        self.weapon.update(time, event, data, attack, particles, enemy);
        self.artifact.update(time, event, data, attack, particles, enemy);
    }

    pub fn accelerate(&mut self) -> () {
        if let Some(f) = self.character.accelerator() {
            self.character.skill_mut().accelerate(f);
        }
        if let Some(f) = self.weapon.accelerator() {
            self.character.skill_mut().accelerate(f);
        }
    }

    pub fn intensify(&self, attack: &Attack) -> Option<State> {
        let mut result: Option<State> = None;
        let xs = &mut [
            self.character.intensify(attack),
            self.weapon.intensify(attack),
        ];
        for some_state in xs.iter_mut() {
            match (&mut result, some_state) {
                (Some(state), Some(s)) => state.merge(&s),
                (Some(_), None) => (),
                (None, state @ Some(_)) => result = state.take(),
                (None, None) => (),
            };
        }
        result
    }

    pub fn reset(&mut self) -> () {
        self.character.na_mut().reset();
        self.character.ca_mut().reset();
        self.character.skill_mut().reset();
        self.character.burst_mut().reset();
        self.character.reset();
        self.weapon.reset();
        self.artifact.reset();
    }
}

#[derive(Debug)]
pub struct CharacterRecord {
    pub name: &'static str,
    pub vision: Vision,
    pub weapon: WeaponType,
    pub release_date: &'static str,
    pub version: f32,
    pub energy_cost: f32,
    pub state: State,
}

impl Default for CharacterRecord {
    fn default() -> Self {
        Self {
            name: "Amber",
            vision: Vision::Pyro,
            weapon: WeaponType::Bow,
            release_date: "2020-09-28",
            version: 1.0,
            energy_cost: 40.0,
            state: State::new().cr(5.0).cd(50.0)
        }
    }
}

#[allow(dead_code)]
impl CharacterRecord {
    pub fn name(mut self, name: &'static str) -> Self { self.name = name ; self }
    pub fn release_date(mut self, release_date: &'static str) -> Self { self.release_date = release_date ; self }
    pub fn version(mut self, version: f32) -> Self { self.version = version ; self }
    pub fn vision(mut self, vision: Vision) -> Self { self.vision = vision ; self }
    pub fn weapon(mut self, weapon: WeaponType) -> Self { self.weapon = weapon ; self }
    pub fn energy_cost(mut self, energy_cost: f32) -> Self { self.energy_cost = energy_cost ; self }
    pub fn base_hp(mut self, base_hp: f32) -> Self { self.state.base_hp = base_hp ; self }
    pub fn base_atk(mut self, base_atk: f32) -> Self { self.state.base_atk = base_atk ; self }
    pub fn base_def(mut self, base_def: f32) -> Self { self.state.base_def = base_def ; self }
    pub fn hp(mut self, hp: f32) -> Self { self.state.hp = hp ; self }
    pub fn atk(mut self, atk: f32) -> Self { self.state.atk = atk ; self }
    pub fn def(mut self, def: f32) -> Self { self.state.def = def ; self }
    pub fn cr(mut self, cr: f32) -> Self { self.state.cr = cr ; self }
    pub fn cd(mut self, cd: f32) -> Self { self.state.cd = cd ; self }
    pub fn er(mut self, er: f32) -> Self { self.state.er = er ; self }
    pub fn em(mut self, em: f32) -> Self { self.state.em = em ; self }
    pub fn na_dmg(mut self, na_dmg: f32) -> Self { self.state.na_dmg = na_dmg ; self }
    pub fn ca_dmg(mut self, ca_dmg: f32) -> Self { self.state.ca_dmg = ca_dmg ; self }
    pub fn skill_dmg(mut self, skill_dmg: f32) -> Self { self.state.skill_dmg = skill_dmg ; self }
    pub fn burst_dmg(mut self, burst_dmg: f32) -> Self { self.state.burst_dmg = burst_dmg ; self }
    pub fn physical_dmg(mut self, physical_dmg: f32) -> Self { self.state.physical_dmg = physical_dmg ; self }
    pub fn pyro_dmg(mut self, pyro_dmg: f32) -> Self { self.state.pyro_dmg = pyro_dmg ; self }
    pub fn cryo_dmg(mut self, cryo_dmg: f32) -> Self { self.state.cryo_dmg = cryo_dmg ; self }
    pub fn hydro_dmg(mut self, hydro_dmg: f32) -> Self { self.state.hydro_dmg = hydro_dmg ; self }
    pub fn electro_dmg(mut self, electro_dmg: f32) -> Self { self.state.electro_dmg = electro_dmg ; self }
    pub fn anemo_dmg(mut self, anemo_dmg: f32) -> Self { self.state.anemo_dmg = anemo_dmg ; self }
    pub fn geo_dmg(mut self, geo_dmg: f32) -> Self { self.state.geo_dmg = geo_dmg ; self }
    pub fn dendro_dmg(mut self, dendro_dmg: f32) -> Self { self.state.dendro_dmg = dendro_dmg ; self }
    pub fn infusion(mut self, infusion: bool) -> Self { self.state.infusion = infusion ; self }
}

#[derive(Debug)]
pub struct WeaponRecord {
    pub name: &'static str,
    pub type_: WeaponType,
    pub version: f32,
    pub state: State,
}

impl Default for WeaponRecord {
    fn default() -> Self {
        Self {
            name: "",
            type_: WeaponType::Sword,
            version: 0.0,
            state: State::new()
        }
    }
}

#[allow(dead_code)]
impl WeaponRecord {
    pub fn name(mut self, name: &'static str) -> Self { self.name = name; self }
    pub fn type_(mut self, type_: WeaponType) -> Self { self.type_ = type_; self }
    pub fn version(mut self, version: f32) -> Self { self.version = version; self }
    pub fn base_atk(mut self, base_atk: f32) -> Self { self.state.base_atk = base_atk; self }
    pub fn hp(mut self, hp: f32) -> Self { self.state.hp = hp; self }
    pub fn atk(mut self, atk: f32) -> Self { self.state.atk = atk; self }
    pub fn def(mut self, def: f32) -> Self { self.state.def = def; self }
    pub fn cr(mut self, cr: f32) -> Self { self.state.cr = cr; self }
    pub fn cd(mut self, cd: f32) -> Self { self.state.cd = cd; self }
    pub fn er(mut self, er: f32) -> Self { self.state.er = er; self }
    pub fn em(mut self, em: f32) -> Self { self.state.em = em; self }
    pub fn atk_spd(mut self, atk_spd: f32) -> Self { self.state.atk_spd = atk_spd; self }
    pub fn na_dmg(mut self, na_dmg: f32) -> Self { self.state.na_dmg = na_dmg; self }
    pub fn ca_dmg(mut self, ca_dmg: f32) -> Self { self.state.ca_dmg = ca_dmg; self }
    pub fn skill_dmg(mut self, skill_dmg: f32) -> Self { self.state.skill_dmg = skill_dmg; self }
    pub fn burst_dmg(mut self, burst_dmg: f32) -> Self { self.state.burst_dmg = burst_dmg; self }
    pub fn physical_dmg(mut self, physical_dmg: f32) -> Self { self.state.physical_dmg = physical_dmg; self }
    pub fn pyro_dmg(mut self, pyro_dmg: f32) -> Self { self.state.pyro_dmg = pyro_dmg; self }
    pub fn cryo_dmg(mut self, cryo_dmg: f32) -> Self { self.state.cryo_dmg = cryo_dmg; self }
    pub fn hydro_dmg(mut self, hydro_dmg: f32) -> Self { self.state.hydro_dmg = hydro_dmg; self }
    pub fn electro_dmg(mut self, electro_dmg: f32) -> Self { self.state.electro_dmg = electro_dmg; self }
    pub fn anemo_dmg(mut self, anemo_dmg: f32) -> Self { self.state.anemo_dmg = anemo_dmg; self }
    pub fn geo_dmg(mut self, geo_dmg: f32) -> Self { self.state.geo_dmg = geo_dmg; self }
    pub fn dendro_dmg(mut self, dendro_dmg: f32) -> Self { self.state.dendro_dmg = dendro_dmg; self }
}

#[derive(Debug)]
pub struct Resistance {
    pub pyro: f32,
    pub hydro: f32,
    pub electro: f32,
    pub cryo: f32,
    pub anemo: f32,
    pub geo: f32,
    pub dendro: f32,
    pub physical: f32,
}

impl Resistance {
    pub fn normal() -> Self {
        Self {
            pyro: 10.0,
            hydro: 10.0,
            electro: 10.0,
            cryo: 10.0,
            anemo: 10.0,
            geo: 10.0,
            dendro: 10.0,
            physical: 10.0,
        }
    }

    pub fn zero() -> Self {
        Self {
            pyro: 0.0,
            hydro: 0.0,
            electro: 0.0,
            cryo: 0.0,
            anemo: 0.0,
            geo: 0.0,
            dendro: 0.0,
            physical: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct Enemy {
    pub level: f32,
    pub default: Resistance,

    pub aura: ElementalGauge,
    pub isfrozen: bool,

    pub debuff: Resistance,
    pub def_down: f32,
    pub superconduct: DurationTimer,
}

impl Enemy {
    pub fn hilichurl() -> Self {
        Self {
            level: 90.0,
            default: Resistance::normal(),
            aura: ElementalGauge::default(),
            isfrozen: false,
            debuff: Resistance::zero(),
            def_down: 0.0,
            superconduct: DurationTimer::new(12.0, &[0.0]),
        }
    }

    pub fn simple() -> Self {
        Self {
            level: 90.0,
            default: Resistance::zero(),
            aura: ElementalGauge::default(),
            isfrozen: false,
            debuff: Resistance::zero(),
            def_down: 0.0,
            superconduct: DurationTimer::new(12.0, &[0.0]),
        }
    }

    pub fn trigger_er(&self, e: &Vision) -> ElementalReactionType {
        ElementalReaction::new(self.aura.aura, *e)
    }

    pub fn update(&mut self, time: f32) -> () {
        self.aura.update(time);
        self.superconduct.update(time, false);
        if self.isfrozen && self.aura.aura == Vision::Physical {
            self.isfrozen = false;
        }
    }

    pub fn resistance(&self, element: &Vision) -> f32 {
        let resistance: f32;
        let debuff: f32;
        match element {
            Vision::Pyro => {
                resistance = self.default.pyro;
                debuff = self.debuff.pyro;
            },
            Vision::Hydro => {
                resistance = self.default.hydro;
                debuff = self.debuff.hydro;
            },
            Vision::Electro => {
                resistance = self.default.electro;
                debuff = self.debuff.electro;
            },
            Vision::Cryo => {
                resistance = self.default.cryo;
                debuff = self.debuff.cryo;
            },
            Vision::Anemo => {
                resistance = self.default.anemo;
                debuff = self.debuff.anemo;
            },
            Vision::Geo => {
                resistance = self.default.geo;
                debuff = self.debuff.geo;
            },
            Vision::Dendro => {
                resistance = self.default.dendro;
                debuff = self.debuff.dendro;
            },
            Vision::Physical => {
                resistance = self.default.physical;
                debuff = self.debuff.physical + if self.superconduct.n == 1 {
                    40.0
                } else {
                    0.0
                };
            },
        }
        let res = if debuff > resistance {
            -0.5 * (debuff - resistance)
        } else {
            resistance - debuff
        };
        (100.0 - res) / 100.0
    }
}

// #[derive(Debug)]
pub struct CharacterData<'a> {
    pub idx: FieldCharacterIndex,
    pub character: &'a CharacterRecord,
    pub weapon: &'a WeaponRecord,
    pub artifact: &'a Artifact,
    pub state: State,
}

impl<'a> CharacterData<'a> {
    pub fn new(idx: FieldCharacterIndex, character: &'a CharacterRecord, weapon: &'a WeaponRecord, artifact: &'a Artifact) -> Self {
        Self {
            idx,
            character,
            weapon,
            artifact,
            state: State::new(),
        }
    }

    pub fn init(&mut self) -> () {
        self.state.merge(&self.character.state);
        self.state.merge(&self.weapon.state);
        self.state.merge(&self.artifact.state);
    }

    pub fn can_burst(&self) -> bool {
        self.state.energy >= self.character.energy_cost
    }
}
