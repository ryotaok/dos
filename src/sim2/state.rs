use crate::sim2::types::{CharacterAction, DamageType, Vision, UnstackableBuff, NOBLESSE_OBLIGE};
use crate::sim2::record::{CharacterData};

#[derive(Debug, Clone, Copy)]
pub struct ICDColumn<T> {
    pub burst: T,
    pub skill: T,
    pub na: T,
    pub ca: T,
}

impl Default for ICDColumn<f32> {
    fn default() -> Self {
        Self {
            burst: 0.0,
            skill: 0.0,
            na: 0.0,
            ca: 0.0,
        }
    }
}

impl Default for ICDColumn<u8> {
    fn default() -> Self {
        Self {
            burst: 0,
            skill: 0,
            na: 0,
            ca: 0,
        }
    }
}

// this State class is meant to hold numbers only so that
// 1) they can be merged
// 2) initialized without arguments
#[derive(Debug, Clone, Copy)]
pub struct State {
    pub base_hp: f32, pub base_def: f32, pub base_atk: f32,
    pub hp: f32, pub def: f32, pub atk: f32, pub flat_atk: f32, pub cr: f32, pub cd: f32, pub em: f32,
    pub na_dmg: f32, pub ca_dmg: f32, pub skill_dmg: f32, pub burst_dmg: f32, pub all_dmg: f32,
    pub physical_dmg: f32, pub pyro_dmg: f32, pub cryo_dmg: f32, pub hydro_dmg: f32, pub electro_dmg: f32,
    pub anemo_dmg: f32, pub geo_dmg: f32, pub dendro_dmg: f32, pub elemental_dmg: f32,
    pub infusion: bool, pub stacked_buff: UnstackableBuff, pub amplifying_bonus: f32, pub transformative_bonus: f32,
    pub na_talent: f32, pub ca_talent: f32, pub skill_talent: f32, pub burst_talent: f32, 
    pub icd_time: ICDColumn<f32>, pub icd_count: ICDColumn<u8>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            base_hp: 0.0, base_def: 0.0, base_atk: 0.0,
            hp: 0.0, def: 0.0, atk: 0.0, flat_atk: 0.0, cr: 0.0, cd: 0.0, em: 0.0,
            na_dmg: 0.0, ca_dmg: 0.0, skill_dmg: 0.0, burst_dmg: 0.0, all_dmg: 0.0,
            physical_dmg: 0.0, pyro_dmg: 0.0, cryo_dmg: 0.0, hydro_dmg: 0.0, electro_dmg: 0.0,
            anemo_dmg: 0.0, geo_dmg: 0.0, dendro_dmg: 0.0, elemental_dmg: 0.0,
            infusion: false, stacked_buff: UnstackableBuff::new(), amplifying_bonus: 0.0, transformative_bonus: 0.0,
            na_talent: 0.0, ca_talent: 0.0, skill_talent: 0.0, burst_talent: 0.0, 
            icd_time: ICDColumn::<f32>::default(), icd_count: ICDColumn::<u8>::default(),
        }
    }
}

impl State {
    pub fn init(&mut self, data: &CharacterData) -> () {
        self.base_hp = data.character.base_hp;
        self.base_def = data.character.base_def;
        self.base_atk = data.character.base_atk + data.weapon.base_atk;
        self.hp = data.character.hp + data.weapon.hp + data.artifact.hp;
        self.def = data.character.def + data.weapon.def + data.artifact.def;
        self.atk = data.character.atk + data.weapon.atk + data.artifact.atk;
        // self.flat_atk = data.character.flat_atk + data.weapon.flat_atk + data.artifact.flat_atk;
        self.cr = data.character.cr + data.weapon.cr + data.artifact.cr;
        self.cd = data.character.cd + data.weapon.cd + data.artifact.cd;
        self.em = data.character.em + data.weapon.em + data.artifact.em;
        self.na_dmg = data.weapon.na_dmg + data.artifact.na_dmg;
        self.ca_dmg = data.weapon.ca_dmg + data.artifact.ca_dmg;
        self.skill_dmg = data.weapon.skill_dmg + data.artifact.skill_dmg;
        self.burst_dmg = data.weapon.burst_dmg + data.artifact.burst_dmg;
        self.all_dmg = data.weapon.all_dmg + data.artifact.all_dmg;
        self.physical_dmg = data.character.physical_dmg + data.weapon.physical_dmg + data.artifact.physical_dmg;
        self.pyro_dmg = data.character.pyro_dmg + data.weapon.pyro_dmg + data.artifact.pyro_dmg;
        self.cryo_dmg = data.character.cryo_dmg + data.weapon.cryo_dmg + data.artifact.cryo_dmg;
        self.hydro_dmg = data.character.hydro_dmg + data.weapon.hydro_dmg + data.artifact.hydro_dmg;
        self.electro_dmg = data.character.electro_dmg + data.weapon.electro_dmg + data.artifact.electro_dmg;
        self.anemo_dmg = data.character.anemo_dmg + data.weapon.anemo_dmg + data.artifact.anemo_dmg;
        self.geo_dmg = data.character.geo_dmg + data.weapon.geo_dmg + data.artifact.geo_dmg;
        self.dendro_dmg = data.character.dendro_dmg + data.weapon.dendro_dmg + data.artifact.dendro_dmg;
        self.elemental_dmg = data.weapon.elemental_dmg + data.artifact.elemental_dmg;
        self.infusion = false;
        self.stacked_buff = UnstackableBuff::new();
        self.amplifying_bonus = 0.0;
        self.transformative_bonus = 0.0;
        self.na_talent = 0.0;
        self.ca_talent = 0.0;
        self.skill_talent = 0.0;
        self.burst_talent = 0.0;
    }

    #[allow(non_snake_case)]
    pub fn HP(&self) -> f32 {
        let flower_hp = 4780.0;
        self.base_hp * (1.0 + self.hp / 100.0) + flower_hp
    }

    #[allow(non_snake_case)]
    pub fn DEF(&self) -> f32 {
        self.base_def * (1.0 + self.def / 100.0)
    }

    #[allow(non_snake_case)]
    pub fn ATK(&self) -> f32 {
        self.base_atk * (1.0 + self.atk / 100.0) + self.flat_atk
    }

    pub fn get_talent_bonus(&self, key: &DamageType) -> f32 {
        let b = match key {
            DamageType::Na => self.na_talent,
            DamageType::Ca => self.ca_talent,
            DamageType::Skill => self.skill_talent,
            DamageType::Burst => self.burst_talent,
            _ => 0.0,
        };
        1.0 + b / 100.0
    }

    pub fn get_attack_bonus(&self, key: &DamageType) -> f32 {
        match key {
            DamageType::Na => self.na_dmg + self.all_dmg,
            DamageType::Ca => self.ca_dmg + self.all_dmg,
            DamageType::Skill => self.skill_dmg + self.all_dmg,
            DamageType::Burst => self.burst_dmg + self.all_dmg,
            _ => self.all_dmg,
        }
    }

    #[allow(non_snake_case)]
    pub fn DMG_bonus(&self, attack_type: &DamageType, infusion_element: &Vision) -> f32 {
        use Vision::*;
        let bonus = self.get_attack_bonus(attack_type) + match infusion_element {
            Pyro => self.pyro_dmg + self.elemental_dmg,
            Cryo => self.cryo_dmg + self.elemental_dmg,
            Hydro => self.hydro_dmg + self.elemental_dmg,
            Electro => self.electro_dmg + self.elemental_dmg,
            Anemo => self.anemo_dmg + self.elemental_dmg,
            Geo => self.geo_dmg + self.elemental_dmg,
            Dendro => self.dendro_dmg + self.elemental_dmg,
            Physical => self.physical_dmg,
        };
        1.0 + bonus / 100.0
    }

    #[allow(non_snake_case)]
    pub fn CRCD(&self) -> f32 {
        let cr_threshold = 80.0;
        let mut cr = self.cr;
        let mut cd = self.cd;
        if cr > cr_threshold {
            cd += (cr - cr_threshold) * 2.0;
            cr = cr_threshold;
        } else if cr < 0.0 {
            return 1.0
        }
        1.0 + cd / 100.0 * cr / 100.0
    }

    pub fn apply_aura(&mut self, time: f32, event: &CharacterAction) -> bool {
        use CharacterAction::*;
        match event {
            StandStill => false,
            Burst => {
                let b = _can_apply_aura(time, self.icd_time.burst, self.icd_count.burst);
                _count_hit(time, &mut self.icd_time.burst, &mut self.icd_count.burst);
                b
            },
            PressSkill |
            HoldSkill => {
                let b = _can_apply_aura(time, self.icd_time.skill, self.icd_count.skill);
                _count_hit(time, &mut self.icd_time.skill, &mut self.icd_count.skill);
                b
            },
            Na1(_) |
            Na2(_) |
            Na3(_) |
            Na4(_) |
            Na5(_) |
            Na6(_) => {
                let b = _can_apply_aura(time, self.icd_time.na, self.icd_count.na);
                _count_hit(time, &mut self.icd_time.na, &mut self.icd_count.na);
                b
            },
            Ca => {
                let b = _can_apply_aura(time, self.icd_time.ca, self.icd_count.ca);
                _count_hit(time, &mut self.icd_time.ca, &mut self.icd_count.ca);
                b
            },
        }
    }
}

fn _can_apply_aura(time: f32, last_time: f32, count: u8) -> bool {
    time - last_time >= 2.5 || count == 0
}

// 0 t 1
// 1 f 2
// 2 f 0
fn _count_hit(time: f32, last_time: &mut f32, count: &mut u8) -> () {
    if time - *last_time >= 2.5 {
        *last_time = time;
        *count = 1;
    } else {
        if *count >= 2 {
            *count = 0;
        } else {
            *count += 1;
        }
    }
}
