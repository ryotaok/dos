use crate::sim2::types::{CharacterAction, DamageType, Vision, FieldCharacterIndex, FieldEnergy, WeaponType, Preference};
use crate::sim2::element::{ElementalGauge, ElementalReactionType, ElementalReaction};
use crate::sim2::state::State;
use crate::sim2::timeline::Timeline;
use crate::sim2::attack::{CharacterAttack, WeaponAttack};

#[derive(Debug)]
pub struct CharacterRecord {
    pub name: &'static str,
    pub vision: Vision,
    pub weapon: WeaponType,
    pub release_date: &'static str,
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
            release_date: "2020-09-28",
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
    pub fn release_date(mut self, release_date: &'static str) -> Self { self.release_date = release_date ; self }
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

impl Default for Artifact {
    fn default() -> Self {
        Self {
            name: "",
            version: 1.0,
            preference: &[],
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

    // pub fn update(&mut self, time: f32) -> () {
    //     self.aura.update(time);
    //     self.superconduct.update(time, false);
    //     if self.isfrozen && self.aura.aura == Vision::Physical {
    //         self.isfrozen = false;
    //     }
    // }

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
}

/*
impl Timeline for HuTao {
    fn decide_action(&mut self, state: &ActionState) -> CharacterAction {
        // is burst CD off and has enough energy
        if state.rel_time.burst >= 15.0 && state.energy >= 60.0 {
            Burst
        // check if skill can be used
        } else if state.rel_time.skill >= 16.0 {
            PressSkill
        // check if normal attacks can be used (both animations are ended)
        } else if state.rel_time.na >= 0.4875 && state.rel_time.ca >= 0.915 {
            if state.rel_time.skill <= 9.0 {
                Ca
            } else {
                let na = self.na_idx.to_action();
                self.na_idx += 1;
                if self.na_idx > 6 {
                    self.na_idx = 1;
                }
                na
            }
        } else {
            StandStill
        }
    }

    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState) -> () {
        match event {
            CharacterAction::PressSkill => field_energy.push_p(Particle::new(Pyro, 3.0)),
            _ => (),
        }
    }
}
*/

/*
impl CharacterAttack for HuTao {
    fn burst(&self, time: f32, event: &CharacterAction, atk_queue: &mut Vec<Attack2>, state: &mut State2) -> () {
        atk_queue.push(Attack2 {
            kind: DamageType::Burst,
            multiplier: 617.44,
            element: &PYRO_GAUGE2B,
            aura_application: state.apply_aura(time, event),
            time,
            idx: self.idx,
        });
    }

    fn press(&self, time: f32, event: &CharacterAction, atk_queue: &mut Vec<Attack2>, state: &mut State2) -> () {
        atk_queue.push(Attack2 {
            kind: DamageType::Skill,
            multiplier: 115.2,
            element: &PYRO_GAUGE1A,
            aura_application: state.apply_aura(time, event),
            time,
            idx: self.idx,
        });
        atk_queue.push(Attack2 {
            kind: DamageType::Skill,
            multiplier: 115.2,
            element: &PYRO_GAUGE1A,
            aura_application: state.apply_aura(time + 4.0, event),
            time: time + 4.0,
            idx: self.idx,
        });
    }

    // self FieldCharacterIndex
    // TODO own attack and others?
    // `ActionState` is the state of this character
    // `Attack2` and `State2` can be owned by this character or the others
    fn modify(&self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack2, state: &mut State2, enemy: &mut Enemy) -> () {
        // skill duration
        let oneself = self.idx == attack.idx;
        let dr = attack.time - action_state.abs_time.press;
        if oneself && dr <= 9.0 {
            state.flat_atk += state.HP() * 0.0626;
            match &attack.kind {
                DamageType::Ca |
                DamageType::Na => attack.element = &PYRO_GAUGE1A,
                _ => (),
            };
        // a1
        } else if !oneself && 9.0 < dr && dr <= 17.0 {
            state.cr += 12.0;
        }
    }
}
*/
