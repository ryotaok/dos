use crate::sim1::types::{AttackType, UnstackableBuff, Vision, NOBLESSE_OBLIGE};

// this State class is meant to hold numbers only so that
// 1) they can be merged
// 2) initialized without arguments
#[derive(Debug)]
pub struct State {
    pub base_hp: f32, pub base_def: f32, pub base_atk: f32,
    pub hp: f32, pub def: f32, pub atk: f32, pub flat_atk: f32, pub cr: f32, pub cd: f32, pub er: f32, pub em: f32, pub atk_spd: f32,
    pub energy: f32,
    pub na_dmg: f32, pub ca_dmg: f32, pub skill_dmg: f32, pub burst_dmg: f32, pub all_dmg: f32,
    pub pyro_dmg: f32, pub cryo_dmg: f32, pub hydro_dmg: f32, pub electro_dmg: f32,
    pub physical_dmg: f32, pub anemo_dmg: f32, pub geo_dmg: f32, pub dendro_dmg: f32, pub elemental_dmg: f32,
    pub infusion: bool, pub stacked_buff: UnstackableBuff, pub amplifying_bonus: f32, pub transformative_bonus: f32,
    pub na_talent: f32, pub ca_talent: f32, pub skill_talent: f32, pub burst_talent: f32, 
}

impl Default for State {
    fn default() -> Self {
        State::new()
    }
}

// impl Clone for State {
//     fn clone(&self) -> Self {
//         Self {
//             base_hp: self.base_hp, base_def: self.base_def, base_atk: self.base_atk,
//             hp: self.hp, def: self.def, atk: self.atk, flat_atk: self.flat_atk, cr: self.cr, cd: self.cd, er: self.er, em: self.em, atk_spd: self.atk_spd,
//             energy: Energy(self.energy), energy_cost: self.energy_cost,
//             na_dmg: self.na_dmg, ca_dmg: self.ca_dmg, skill_dmg: self.skill_dmg, burst_dmg: self.burst_dmg, all_dmg: self.all_dmg,
//             pyro_dmg: self.pyro_dmg, cryo_dmg: self.cryo_dmg, hydro_dmg: self.hydro_dmg, electro_dmg: self.electro_dmg,
//             physical_dmg: self.physical_dmg, anemo_dmg: self.anemo_dmg, geo_dmg: self.geo_dmg, dendro_dmg: self.dendro_dmg, elemental_dmg: self.elemental_dmg,
//             infusion: self.infusion
//         }
//     }
// }

#[allow(dead_code)]
impl State {
    pub fn new() -> State {
        State {
            base_hp: 0.0, base_def: 0.0, base_atk: 0.0,
            hp: 0.0, def: 0.0, atk: 0.0, flat_atk: 0.0, cr: 0.0, cd: 0.0, er: 0.0, em: 0.0, atk_spd: 0.0,
            energy: 0.0,
            na_dmg: 0.0, ca_dmg: 0.0, skill_dmg: 0.0, burst_dmg: 0.0, all_dmg: 0.0,
            pyro_dmg: 0.0, cryo_dmg: 0.0, hydro_dmg: 0.0, electro_dmg: 0.0,
            physical_dmg: 0.0, anemo_dmg: 0.0, geo_dmg: 0.0, dendro_dmg: 0.0, elemental_dmg: 0.0,
            infusion: false, stacked_buff: UnstackableBuff::new(), amplifying_bonus: 0.0, transformative_bonus: 0.0,
            na_talent: 0.0, ca_talent: 0.0, skill_talent: 0.0, burst_talent: 0.0, 
        }
    }

    pub fn clear(&mut self) -> () {
        self.base_hp = 0.0;
        self.base_def = 0.0;
        self.base_atk = 0.0;
        self.hp = 0.0;
        self.def = 0.0;
        self.atk = 0.0;
        self.flat_atk = 0.0;
        self.cr = 0.0;
        self.cd = 0.0;
        self.er = 0.0;
        self.em = 0.0;
        self.atk_spd = 0.0;
        // self.energy = 0.0;
        self.na_dmg = 0.0;
        self.ca_dmg = 0.0;
        self.skill_dmg = 0.0;
        self.burst_dmg = 0.0;
        self.all_dmg = 0.0;
        self.physical_dmg = 0.0;
        self.pyro_dmg = 0.0;
        self.cryo_dmg = 0.0;
        self.hydro_dmg = 0.0;
        self.electro_dmg = 0.0;
        self.anemo_dmg = 0.0;
        self.geo_dmg = 0.0;
        self.dendro_dmg = 0.0;
        self.elemental_dmg = 0.0;
        self.infusion = false;
        self.stacked_buff = UnstackableBuff::new();
        self.amplifying_bonus = 0.0;
        self.transformative_bonus = 0.0;
        self.na_talent = 0.0;
        self.ca_talent = 0.0;
        self.skill_talent = 0.0;
        self.burst_talent = 0.0;
    }

    // should be used as one-liner
    // DOC https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
    // pub fn base_hp(mut self, base_hp: f32) -> Self { self.base_hp = base_hp; self }
    // pub fn base_def(mut self, base_def: f32) -> Self { self.base_def = base_def; self }
    // pub fn base_atk(mut self, base_atk: f32) -> Self { self.base_atk = base_atk; self }
    pub fn hp(mut self, hp: f32) -> Self { self.hp = hp; self }
    pub fn def(mut self, def: f32) -> Self { self.def = def; self }
    pub fn atk(mut self, atk: f32) -> Self { self.atk = atk; self }
    // pub fn flat_atk(mut self, flat_atk: f32) -> Self { self.flat_atk = flat_atk; self }
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
    pub fn pyro_dmg(mut self, pyro_dmg: f32) -> Self { self.pyro_dmg = pyro_dmg; self }
    pub fn cryo_dmg(mut self, cryo_dmg: f32) -> Self { self.cryo_dmg = cryo_dmg; self }
    pub fn hydro_dmg(mut self, hydro_dmg: f32) -> Self { self.hydro_dmg = hydro_dmg; self }
    pub fn electro_dmg(mut self, electro_dmg: f32) -> Self { self.electro_dmg = electro_dmg; self }
    pub fn anemo_dmg(mut self, anemo_dmg: f32) -> Self { self.anemo_dmg = anemo_dmg; self }
    pub fn geo_dmg(mut self, geo_dmg: f32) -> Self { self.geo_dmg = geo_dmg; self }
    pub fn dendro_dmg(mut self, dendro_dmg: f32) -> Self { self.dendro_dmg = dendro_dmg; self }
    pub fn elemental_dmg(mut self, elemental_dmg: f32) -> Self { self.elemental_dmg = elemental_dmg; self }
    pub fn infusion(mut self, infusion: bool) -> Self { self.infusion = infusion; self }
    pub fn amplifying_bonus(mut self, amplifying_bonus: f32) -> Self { self.amplifying_bonus = amplifying_bonus; self }
    pub fn transformative_bonus(mut self, transformative_bonus: f32) -> Self { self.transformative_bonus = transformative_bonus; self }
    pub fn na_talent(mut self, na_talent: f32) -> Self { self.na_talent = na_talent; self }
    pub fn ca_talent(mut self, ca_talent: f32) -> Self { self.ca_talent = ca_talent; self }
    pub fn skill_talent(mut self, skill_talent: f32) -> Self { self.skill_talent = skill_talent; self }
    pub fn burst_talent(mut self, burst_talent: f32) -> Self { self.burst_talent = burst_talent; self }

    // pub fn merge(&mut self, other: &State) -> &mut Self {
    pub fn merge(&mut self, other: &State) -> () {
        self.base_hp += other.base_hp;
        self.base_def += other.base_def;
        self.base_atk += other.base_atk;
        self.hp += other.hp;
        self.def += other.def;
        self.atk += other.atk;
        self.flat_atk += other.flat_atk;
        self.cr += other.cr;
        self.cd += other.cd;
        self.er += other.er;
        self.em += other.em;
        self.atk_spd += other.atk_spd;
        self.energy += other.energy;
        self.na_dmg += other.na_dmg;
        self.ca_dmg += other.ca_dmg;
        self.skill_dmg += other.skill_dmg;
        self.burst_dmg += other.burst_dmg;
        self.all_dmg += other.all_dmg;
        self.physical_dmg += other.physical_dmg;
        self.pyro_dmg += other.pyro_dmg;
        self.cryo_dmg += other.cryo_dmg;
        self.hydro_dmg += other.hydro_dmg;
        self.electro_dmg += other.electro_dmg;
        self.anemo_dmg += other.anemo_dmg;
        self.geo_dmg += other.geo_dmg;
        self.dendro_dmg += other.dendro_dmg;
        self.elemental_dmg += other.elemental_dmg;
        self.infusion |= other.infusion;
        self.stacked_buff.turn_on(&other.stacked_buff);
        self.amplifying_bonus += other.amplifying_bonus;
        self.transformative_bonus += other.transformative_bonus;
        self.na_talent += other.na_talent;
        self.ca_talent += other.ca_talent;
        self.skill_talent += other.skill_talent;
        self.burst_talent += other.burst_talent;
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
        let plume_atk = self.flat_atk;
        self.base_atk * (1.0 + self.atk / 100.0) + plume_atk
    }

    pub fn get_talent_bonus(&self, key: &AttackType) -> f32 {
        let b = match key {
            AttackType::Na => self.na_talent,
            AttackType::Ca => self.ca_talent,
            AttackType::PressSkill |
            AttackType::HoldSkill |
            AttackType::SkillDot => self.skill_talent,
            AttackType::Burst |
            AttackType::BurstDot => self.burst_talent,
            _ => 0.0,
        };
        1.0 + b / 100.0
    }

    pub fn get_attack_bonus(&self, key: &AttackType) -> f32 {
        match key {
            AttackType::Na => self.na_dmg + self.all_dmg,
            AttackType::Ca => self.ca_dmg + self.all_dmg,
            AttackType::PressSkill |
            AttackType::HoldSkill |
            AttackType::SkillDot => self.skill_dmg + self.all_dmg,
            AttackType::Burst |
            AttackType::BurstDot => self.burst_dmg + self.all_dmg,
            _ => self.all_dmg,
        }
    }

    #[allow(non_snake_case)]
    pub fn DMG_bonus(&self, attack_type: &AttackType, infusion_element: &Vision) -> f32 {
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

    #[allow(non_snake_case)]
    pub fn ER(&self) -> f32 {
        1.0 + self.er / 100.0
    }
}

#[derive(Debug)]
pub struct GearScore {
    pub score: f32
}

impl GearScore {
    const HP: f32 = 5.83 / 3.89;
    const ATK: f32 = 5.83 / 3.89;
    const DEF: f32 = 7.29 / 3.89;
    // const CR: f32 = 1.0; // 3.89 / 3.89
    const CD: f32 = 7.77 / 3.89;
    const ER: f32 = 6.48 / 3.89;
    const EM: f32 = 23.310 / 3.89;

    pub fn new(score: f32) -> Self {
        Self {
            score
        }
    }

    pub fn hp(&self, r: f32) -> f32 {
        self.score * r / 100.0 * Self::HP
    }

    pub fn atk(&self, r: f32) -> f32 {
        self.score * r / 100.0 * Self::ATK
    }

    pub fn def(&self, r: f32) -> f32 {
        self.score * r / 100.0 * Self::DEF
    }

    pub fn cr(&self, r: f32) -> f32 {
        self.score * r / 100.0
    }

    pub fn cd(&self, r: f32) -> f32 {
        self.score * r / 100.0 * Self::CD
    }

    pub fn er(&self, r: f32) -> f32 {
        self.score * r / 100.0 * Self::ER
    }

    pub fn em(&self, r: f32) -> f32 {
        self.score * r / 100.0 * Self::EM
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge() {
        let mut a = State::new();
        let b = State {
            base_hp: 1.0,
            base_def: 1.0,
            base_atk: 1.0,
            hp: 1.0,
            def: 1.0,
            atk: 1.0,
            flat_atk: 1.0,
            cr: 1.0,
            cd: 1.0,
            er: 1.0,
            em: 1.0,
            atk_spd: 1.0,
            energy: 1.0,
            na_dmg: 1.0,
            ca_dmg: 1.0,
            skill_dmg: 1.0,
            burst_dmg: 1.0,
            all_dmg: 1.0,
            pyro_dmg: 1.0,
            cryo_dmg: 1.0,
            hydro_dmg: 1.0,
            electro_dmg: 1.0,
            physical_dmg: 1.0,
            anemo_dmg: 1.0,
            geo_dmg: 1.0,
            dendro_dmg: 1.0,
            elemental_dmg: 1.0,
            infusion: true,
            stacked_buff: NOBLESSE_OBLIGE,
            amplifying_bonus: 1.0,
            transformative_bonus: 1.0,
            na_talent: 1.0,
            ca_talent: 1.0,
            skill_talent: 1.0,
            burst_talent: 1.0,
        };
        a.merge(&b);
        assert_eq!(a.base_hp, 1.0);
        assert_eq!(a.base_def, 1.0);
        assert_eq!(a.base_atk, 1.0);
        assert_eq!(a.hp, 1.0);
        assert_eq!(a.def, 1.0);
        assert_eq!(a.atk, 1.0);
        assert_eq!(a.flat_atk, 1.0);
        assert_eq!(a.cr, 1.0);
        assert_eq!(a.cd, 1.0);
        assert_eq!(a.er, 1.0);
        assert_eq!(a.em, 1.0);
        assert_eq!(a.atk_spd, 1.0);
        assert_eq!(a.energy, 1.0);
        assert_eq!(a.na_dmg, 1.0);
        assert_eq!(a.ca_dmg, 1.0);
        assert_eq!(a.skill_dmg, 1.0);
        assert_eq!(a.burst_dmg, 1.0);
        assert_eq!(a.all_dmg, 1.0);
        assert_eq!(a.pyro_dmg, 1.0);
        assert_eq!(a.cryo_dmg, 1.0);
        assert_eq!(a.hydro_dmg, 1.0);
        assert_eq!(a.electro_dmg, 1.0);
        assert_eq!(a.physical_dmg, 1.0);
        assert_eq!(a.anemo_dmg, 1.0);
        assert_eq!(a.geo_dmg, 1.0);
        assert_eq!(a.dendro_dmg, 1.0);
        assert_eq!(a.elemental_dmg, 1.0);
        assert_eq!(a.infusion, true);
        assert_eq!(a.stacked_buff, NOBLESSE_OBLIGE);
        assert_eq!(a.amplifying_bonus, 1.0);
        assert_eq!(a.transformative_bonus, 1.0);
        assert_eq!(a.na_talent, 1.0);
        assert_eq!(a.ca_talent, 1.0);
        assert_eq!(a.skill_talent, 1.0);
        assert_eq!(a.burst_talent, 1.0);
    }
}
