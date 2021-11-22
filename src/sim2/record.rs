use crate::sim2::types::{CharacterAction, DamageType, Vision, FieldCharacterIndex, FieldEnergy, WeaponType, Preference};
use crate::sim2::element::{ElementalGauge, ElementalReactionType, ElementalReaction, PYRO_GAUGE1A, HYDRO_GAUGE1A, ELECTRO_GAUGE1A, CRYO_GAUGE1A, PHYSICAL_GAUGE};
use crate::sim2::state::State;
use crate::sim2::timeline::Timeline;
use crate::sim2::attack::{Attack, CharacterAttack, WeaponAttack};

#[derive(Debug)]
pub struct CharacterRecord {
    pub name: &'static str,
    pub vision: Vision,
    pub weapon: WeaponType,
    pub version: f32,
    pub energy_cost: f32,
    pub base_hp: f32,
    pub base_atk: f32,
    pub base_def: f32,
    pub hp: f32,
    pub atk: f32,
    pub def: f32,
    pub cr: f32,
    pub cd: f32,
    pub er: f32,
    pub em: f32,
    pub physical_dmg: f32,
    pub pyro_dmg: f32,
    pub cryo_dmg: f32,
    pub hydro_dmg: f32,
    pub electro_dmg: f32,
    pub anemo_dmg: f32,
    pub geo_dmg: f32,
    pub dendro_dmg: f32,
}

impl Default for CharacterRecord {
    fn default() -> Self {
        Self {
            name: "Amber",
            vision: Vision::Pyro,
            weapon: WeaponType::Bow,
            version: 1.0,
            energy_cost: 40.0,
            base_hp: 200.0,
            base_atk: 200.0,
            base_def: 200.0,
            hp: 0.0,
            atk: 0.0,
            def: 0.0,
            cr: 5.0,
            cd: 50.0,
            er: 0.0,
            em: 0.0,
            physical_dmg: 0.0,
            pyro_dmg: 0.0,
            cryo_dmg: 0.0,
            hydro_dmg: 0.0,
            electro_dmg: 0.0,
            anemo_dmg: 0.0,
            geo_dmg: 0.0,
            dendro_dmg: 0.0,
        }
    }
}

impl Timeline for CharacterRecord {}
impl CharacterAttack for CharacterRecord {}

#[allow(dead_code)]
impl CharacterRecord {
    pub fn name(mut self, name: &'static str) -> Self { self.name = name ; self }
    pub fn version(mut self, version: f32) -> Self { self.version = version ; self }
    pub fn vision(mut self, vision: Vision) -> Self { self.vision = vision ; self }
    pub fn weapon(mut self, weapon: WeaponType) -> Self { self.weapon = weapon ; self }
    pub fn energy_cost(mut self, energy_cost: f32) -> Self { self.energy_cost = energy_cost ; self }
    pub fn base_hp(mut self, base_hp: f32) -> Self { self.base_hp = base_hp ; self }
    pub fn base_atk(mut self, base_atk: f32) -> Self { self.base_atk = base_atk ; self }
    pub fn base_def(mut self, base_def: f32) -> Self { self.base_def = base_def ; self }
    pub fn hp(mut self, hp: f32) -> Self { self.hp = hp ; self }
    pub fn atk(mut self, atk: f32) -> Self { self.atk = atk ; self }
    pub fn def(mut self, def: f32) -> Self { self.def = def ; self }
    pub fn cr(mut self, cr: f32) -> Self { self.cr = cr ; self }
    pub fn cd(mut self, cd: f32) -> Self { self.cd = cd ; self }
    pub fn er(mut self, er: f32) -> Self { self.er = er ; self }
    pub fn em(mut self, em: f32) -> Self { self.em = em ; self }
    pub fn physical_dmg(mut self, physical_dmg: f32) -> Self { self.physical_dmg = physical_dmg ; self }
    pub fn pyro_dmg(mut self, pyro_dmg: f32) -> Self { self.pyro_dmg = pyro_dmg ; self }
    pub fn cryo_dmg(mut self, cryo_dmg: f32) -> Self { self.cryo_dmg = cryo_dmg ; self }
    pub fn hydro_dmg(mut self, hydro_dmg: f32) -> Self { self.hydro_dmg = hydro_dmg ; self }
    pub fn electro_dmg(mut self, electro_dmg: f32) -> Self { self.electro_dmg = electro_dmg ; self }
    pub fn anemo_dmg(mut self, anemo_dmg: f32) -> Self { self.anemo_dmg = anemo_dmg ; self }
    pub fn geo_dmg(mut self, geo_dmg: f32) -> Self { self.geo_dmg = geo_dmg ; self }
    pub fn dendro_dmg(mut self, dendro_dmg: f32) -> Self { self.dendro_dmg = dendro_dmg ; self }
}

#[derive(Debug)]
pub struct WeaponRecord {
    pub name: &'static str,
    pub type_: WeaponType,
    pub version: f32,
    pub base_atk: f32,
    pub hp: f32,
    pub atk: f32,
    pub def: f32,
    pub cr: f32,
    pub cd: f32,
    pub er: f32,
    pub em: f32,
    pub atk_spd: f32,
    pub na_dmg: f32,
    pub ca_dmg: f32,
    pub skill_dmg: f32,
    pub burst_dmg: f32,
    pub all_dmg: f32,
    pub physical_dmg: f32,
    pub elemental_dmg: f32,
    pub pyro_dmg: f32,
    pub cryo_dmg: f32,
    pub hydro_dmg: f32,
    pub electro_dmg: f32,
    pub anemo_dmg: f32,
    pub geo_dmg: f32,
    pub dendro_dmg: f32,
}

impl Default for WeaponRecord {
    fn default() -> Self {
        Self {
            name: "",
            type_: WeaponType::Sword,
            version: 0.0,
            base_atk: 0.0,
            hp: 0.0,
            atk: 0.0,
            def: 0.0,
            cr: 0.0,
            cd: 0.0,
            er: 0.0,
            em: 0.0,
            atk_spd: 0.0,
            na_dmg: 0.0,
            ca_dmg: 0.0,
            skill_dmg: 0.0,
            burst_dmg: 0.0,
            all_dmg: 0.0,
            physical_dmg: 0.0,
            elemental_dmg: 0.0,
            pyro_dmg: 0.0,
            cryo_dmg: 0.0,
            hydro_dmg: 0.0,
            electro_dmg: 0.0,
            anemo_dmg: 0.0,
            geo_dmg: 0.0,
            dendro_dmg: 0.0,
        }
    }
}

impl Timeline for WeaponRecord {}
impl WeaponAttack for WeaponRecord {}

#[allow(dead_code)]
impl WeaponRecord {
    pub fn name(mut self, name: &'static str) -> Self { self.name = name; self }
    pub fn type_(mut self, type_: WeaponType) -> Self { self.type_ = type_; self }
    pub fn version(mut self, version: f32) -> Self { self.version = version; self }
    pub fn base_atk(mut self, base_atk: f32) -> Self { self.base_atk = base_atk; self }
    pub fn hp(mut self, hp: f32) -> Self { self.hp = hp; self }
    pub fn atk(mut self, atk: f32) -> Self { self.atk = atk; self }
    pub fn def(mut self, def: f32) -> Self { self.def = def; self }
    pub fn cr(mut self, cr: f32) -> Self { self.cr = cr; self }
    pub fn cd(mut self, cd: f32) -> Self { self.cd = cd; self }
    pub fn er(mut self, er: f32) -> Self { self.er = er; self }
    pub fn em(mut self, em: f32) -> Self { self.em = em; self }
    pub fn atk_spd(mut self, atk_spd: f32) -> Self { self.atk_spd = atk_spd; self }
    pub fn na_dmg(mut self, na_dmg: f32) -> Self { self.na_dmg = na_dmg; self }
    pub fn ca_dmg(mut self, ca_dmg: f32) -> Self { self.ca_dmg = ca_dmg; self }
    pub fn skill_dmg(mut self, skill_dmg: f32) -> Self { self.skill_dmg = skill_dmg; self }
    pub fn burst_dmg(mut self, burst_dmg: f32) -> Self { self.burst_dmg = burst_dmg; self }
    pub fn all_dmg(mut self, all_dmg: f32) -> Self { self.all_dmg = all_dmg; self }
    pub fn physical_dmg(mut self, physical_dmg: f32) -> Self { self.physical_dmg = physical_dmg; self }
    pub fn elemental_dmg(mut self, elemental_dmg: f32) -> Self { self.elemental_dmg = elemental_dmg; self }
    pub fn pyro_dmg(mut self, pyro_dmg: f32) -> Self { self.pyro_dmg = pyro_dmg; self }
    pub fn cryo_dmg(mut self, cryo_dmg: f32) -> Self { self.cryo_dmg = cryo_dmg; self }
    pub fn hydro_dmg(mut self, hydro_dmg: f32) -> Self { self.hydro_dmg = hydro_dmg; self }
    pub fn electro_dmg(mut self, electro_dmg: f32) -> Self { self.electro_dmg = electro_dmg; self }
    pub fn anemo_dmg(mut self, anemo_dmg: f32) -> Self { self.anemo_dmg = anemo_dmg; self }
    pub fn geo_dmg(mut self, geo_dmg: f32) -> Self { self.geo_dmg = geo_dmg; self }
    pub fn dendro_dmg(mut self, dendro_dmg: f32) -> Self { self.dendro_dmg = dendro_dmg; self }
}

#[derive(Debug)]
pub struct Artifact {
    pub name: &'static str,
    pub version: f32,
    pub preference: &'static [Preference],
    pub hp: f32,
    pub atk: f32,
    pub def: f32,
    pub flat_atk: f32,
    pub cr: f32,
    pub cd: f32,
    pub er: f32,
    pub em: f32,
    pub atk_spd: f32,
    pub na_dmg: f32,
    pub ca_dmg: f32,
    pub skill_dmg: f32,
    pub burst_dmg: f32,
    pub all_dmg: f32,
    pub physical_dmg: f32,
    pub elemental_dmg: f32,
    pub pyro_dmg: f32,
    pub cryo_dmg: f32,
    pub hydro_dmg: f32,
    pub electro_dmg: f32,
    pub anemo_dmg: f32,
    pub geo_dmg: f32,
    pub dendro_dmg: f32,
    pub amplifying_bonus: f32,
    pub transformative_bonus: f32,
}

impl Default for Artifact {
    fn default() -> Self {
        Self {
            name: "",
            version: 1.0,
            preference: &[],
            hp: 0.0,
            atk: 0.0,
            def: 0.0,
            flat_atk: 0.0,
            cr: 0.0,
            cd: 0.0,
            er: 0.0,
            em: 0.0,
            atk_spd: 0.0,
            na_dmg: 0.0,
            ca_dmg: 0.0,
            skill_dmg: 0.0,
            burst_dmg: 0.0,
            all_dmg: 0.0,
            physical_dmg: 0.0,
            elemental_dmg: 0.0,
            pyro_dmg: 0.0,
            cryo_dmg: 0.0,
            hydro_dmg: 0.0,
            electro_dmg: 0.0,
            anemo_dmg: 0.0,
            geo_dmg: 0.0,
            dendro_dmg: 0.0,
            amplifying_bonus: 0.0,
            transformative_bonus: 0.0,
        }
    }
}

impl Timeline for Artifact {}
impl WeaponAttack for Artifact {}

#[allow(dead_code)]
impl Artifact {
    pub fn name(mut self, name: &'static str) -> Self { self.name = name; self }
    pub fn version(mut self, version: f32) -> Self { self.version = version; self }
    pub fn preference(mut self, preference: &'static [Preference]) -> Self { self.preference = preference; self }
    pub fn hp(mut self, hp: f32) -> Self { self.hp = hp; self }
    pub fn atk(mut self, atk: f32) -> Self { self.atk = atk; self }
    pub fn def(mut self, def: f32) -> Self { self.def = def; self }
    pub fn flat_atk(mut self, flat_atk: f32) -> Self { self.flat_atk = flat_atk; self }
    pub fn cr(mut self, cr: f32) -> Self { self.cr = cr; self }
    pub fn cd(mut self, cd: f32) -> Self { self.cd = cd; self }
    pub fn er(mut self, er: f32) -> Self { self.er = er; self }
    pub fn em(mut self, em: f32) -> Self { self.em = em; self }
    pub fn atk_spd(mut self, atk_spd: f32) -> Self { self.atk_spd = atk_spd; self }
    pub fn na_dmg(mut self, na_dmg: f32) -> Self { self.na_dmg = na_dmg; self }
    pub fn ca_dmg(mut self, ca_dmg: f32) -> Self { self.ca_dmg = ca_dmg; self }
    pub fn skill_dmg(mut self, skill_dmg: f32) -> Self { self.skill_dmg = skill_dmg; self }
    pub fn burst_dmg(mut self, burst_dmg: f32) -> Self { self.burst_dmg = burst_dmg; self }
    pub fn all_dmg(mut self, all_dmg: f32) -> Self { self.all_dmg = all_dmg; self }
    pub fn physical_dmg(mut self, physical_dmg: f32) -> Self { self.physical_dmg = physical_dmg; self }
    pub fn elemental_dmg(mut self, elemental_dmg: f32) -> Self { self.elemental_dmg = elemental_dmg; self }
    pub fn pyro_dmg(mut self, pyro_dmg: f32) -> Self { self.pyro_dmg = pyro_dmg; self }
    pub fn cryo_dmg(mut self, cryo_dmg: f32) -> Self { self.cryo_dmg = cryo_dmg; self }
    pub fn hydro_dmg(mut self, hydro_dmg: f32) -> Self { self.hydro_dmg = hydro_dmg; self }
    pub fn electro_dmg(mut self, electro_dmg: f32) -> Self { self.electro_dmg = electro_dmg; self }
    pub fn anemo_dmg(mut self, anemo_dmg: f32) -> Self { self.anemo_dmg = anemo_dmg; self }
    pub fn geo_dmg(mut self, geo_dmg: f32) -> Self { self.geo_dmg = geo_dmg; self }
    pub fn dendro_dmg(mut self, dendro_dmg: f32) -> Self { self.dendro_dmg = dendro_dmg; self }
    pub fn amplifying_bonus(mut self, amplifying_bonus: f32) -> Self { self.amplifying_bonus = amplifying_bonus; self }
    pub fn transformative_bonus(mut self, transformative_bonus: f32) -> Self { self.transformative_bonus = transformative_bonus; self }

    pub fn is_physical_goblet_user(&self, name: &str) -> bool {
        match name {
            "Eula" |
            "Razor" => true,
            _ => false,
        }
    }

    pub fn infuse_goblet(&mut self, vision: &Vision, name: &str) -> &mut Self {
        if self.is_physical_goblet_user(name) {
            self.physical_dmg = 58.3;
            return self;
        };
        match &vision {
            Vision::Pyro => self.pyro_dmg = 46.6,
            Vision::Cryo => self.cryo_dmg = 46.6,
            Vision::Hydro => self.hydro_dmg = 46.6,
            Vision::Electro => self.electro_dmg = 46.6,
            Vision::Anemo => self.anemo_dmg = 46.6,
            Vision::Geo => self.geo_dmg = 46.6,
            Vision::Dendro => self.dendro_dmg = 46.6,
            Vision::Physical => self.physical_dmg = 58.3,
        };
        self
    }

    pub fn dry_goblet(&mut self) -> () {
        self.pyro_dmg = 0.0;
        self.cryo_dmg = 0.0;
        self.hydro_dmg = 0.0;
        self.electro_dmg = 0.0;
        self.anemo_dmg = 0.0;
        self.geo_dmg = 0.0;
        self.dendro_dmg = 0.0;
        self.physical_dmg = 0.0;
    }
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

    pub aura_time: f32,
    pub aura: ElementalGauge,
    pub isfrozen: bool,

    pub debuff: Resistance,
    pub def_down: f32,
    // indicates the time when superconduct is applied
    // superconduct is not applied if `superconduct_time` is a large negative value
    pub superconduct_time: f32,
}

impl Enemy {
    pub fn hilichurl() -> Self {
        Self {
            level: 90.0,
            default: Resistance::normal(),
            aura_time: 0.0,
            aura: ElementalGauge::default(),
            isfrozen: false,
            debuff: Resistance::zero(),
            def_down: 0.0,
            superconduct_time: -99.0,
        }
    }

    pub fn simple() -> Self {
        Self {
            level: 90.0,
            default: Resistance::zero(),
            aura_time: 0.0,
            aura: ElementalGauge::default(),
            isfrozen: false,
            debuff: Resistance::zero(),
            def_down: 0.0,
            superconduct_time: -99.0,
        }
    }

    pub fn trigger_er(&self, e: &Vision) -> ElementalReactionType {
        ElementalReaction::new(self.aura.aura, *e)
    }

    pub fn absorb_element(&self) -> &'static ElementalGauge {
        match &self.aura.aura {
            Vision::Pyro => &PYRO_GAUGE1A,
            Vision::Hydro => &HYDRO_GAUGE1A,
            Vision::Electro => &ELECTRO_GAUGE1A,
            Vision::Cryo => &CRYO_GAUGE1A,
            _ => &PHYSICAL_GAUGE,
        }
    }

    pub fn undergo_reaction(&mut self, attack: &Attack, elemental_reaction: &ElementalReactionType) -> () {
        use ElementalReactionType::*;
        self.aura.trigger2(attack.time, &mut self.aura_time, &attack.element);
        match &elemental_reaction {
            Freeze(_) => self.isfrozen = true,
            Superconduct(_) => self.superconduct_time = attack.time,
            _ => (),
        }
    }

    pub fn resistance(&self, current_time: f32, element: &Vision) -> f32 {
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
                debuff = self.debuff.physical + if current_time - self.superconduct_time <= 12.0 {
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
pub struct TimelineMember<'a> {
    pub character: &'a mut dyn Timeline,
    pub weapon: &'a mut dyn Timeline,
    pub artifact: &'a mut dyn Timeline,
}

pub struct FieldMember<'a> {
    pub character: &'a mut dyn CharacterAttack,
    pub weapon: &'a mut dyn WeaponAttack,
    pub artifact: &'a mut dyn WeaponAttack,
}

pub struct CharacterData<'a> {
    pub idx: FieldCharacterIndex,
    pub na_idx: usize,
    pub character: &'a CharacterRecord,
    pub weapon: &'a WeaponRecord,
    pub artifact: &'a Artifact,
}

impl<'a> CharacterData<'a> {
    pub fn new(idx: usize, character: &'a CharacterRecord, weapon: &'a WeaponRecord, artifact: &'a Artifact) -> Self {
        Self {
            idx: FieldCharacterIndex(idx),
            na_idx: 1,
            character,
            weapon,
            artifact,
        }
    }

    pub fn reset_na(&mut self, event: &CharacterAction) -> () {
        match event {
            CharacterAction::Burst |
            CharacterAction::PressSkill |
            CharacterAction::HoldSkill |
            CharacterAction::Ca(_) => self.na_idx = 1,
            _ => (),
        };
    }
}
