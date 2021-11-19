use std::cmp::PartialEq;

use crate::sim2::attack::Attack;
    use crate::sim2::cli::Args;
use crate::sim2::record::{CharacterRecord, WeaponRecord, Artifact};
use crate::sim2::element::{ElementalGauge, ElementalReactionType, ElementalReaction, PYRO_GAUGE1A, CRYO_GAUGE1A, HYDRO_GAUGE1A, ELECTRO_GAUGE1A, ANEMO_GAUGE1A, GEO_GAUGE1A, DENDRO_GAUGE1A, PHYSICAL_GAUGE,};

// use self::AttackType::*;
use self::Vision::*;
use self::ElementalReactionType::*;

pub fn approx_equal(a: f32, b: f32, decimal_places: u8) -> bool {
    let factor = 10.0f32.powi(decimal_places as i32);
    let a = (a * factor).trunc();
    let b = (b * factor).trunc();
    a == b
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct FieldCharacterIndex(pub usize);

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum WeaponType {
    Sword,
    Claymore,
    Polearm,
    Bow,
    Catalyst,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum DamageType {
    Na,
    Ca,
    Skill,
    Burst,
    AdditionalAttack,
}

// Each NA action may have a `carryover` time, which is obtained by
// 
//      rel_time.na - CD
// 
// where CD is the cooldown of NA animation. This value reduces the cooldown of
// succeeding NAs.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum CharacterAction {
    Na1(f32), Na2(f32), Na3(f32), Na4(f32), Na5(f32), Na6(f32),
    Ca(f32),
    // Plunge,
    PressSkill,
    HoldSkill,
    Burst,
    StandStill,
}

impl CharacterAction {
    pub fn is_na(&self) -> bool {
        match self {
            CharacterAction::Na1(_) |
            CharacterAction::Na2(_) |
            CharacterAction::Na3(_) |
            CharacterAction::Na4(_) |
            CharacterAction::Na5(_) |
            CharacterAction::Na6(_) => true,
            _ => false,
        }
    }

    pub fn is_ca(&self) -> bool {
        // *self == CharacterAction::Ca(_)
        if let CharacterAction::Ca(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_skill(&self) -> bool {
        *self == CharacterAction::PressSkill || *self == CharacterAction::HoldSkill
    }

    pub fn is_burst(&self) -> bool {
        *self == CharacterAction::Burst
    }
}

pub trait ToNaAction {
    fn to_na(&mut self, n: usize, carryover: f32) -> CharacterAction;
}

impl ToNaAction for usize {
    fn to_na(&mut self, n: usize, carryover: f32) -> CharacterAction {
        let result = match self {
            1 => CharacterAction::Na1(carryover),
            2 => CharacterAction::Na2(carryover),
            3 => CharacterAction::Na3(carryover),
            4 => CharacterAction::Na4(carryover),
            5 => CharacterAction::Na5(carryover),
            6 => CharacterAction::Na6(carryover),
            _ => unimplemented!(),
        };
        *self += 1;
        if *self > n {
            *self = 1;
        }
        result
    }
}

pub trait VecFieldEnergy {
    fn has_particles(&self) -> bool;
    fn push_p(&mut self, p: Particle) -> ();
    fn push_e(&mut self, e: f32) -> ();
}

impl VecFieldEnergy for Vec<FieldEnergy> {
    fn has_particles(&self) -> bool {
        for x in self.iter() {
            match &x {
                FieldEnergy::Particle(_) => return true,
                _ => (),
            }
        };
        false
    }

    fn push_p(&mut self, p: Particle) -> () {
        self.push(FieldEnergy::Particle(p));
    }

    fn push_e(&mut self, e: f32) -> () {
        self.push(FieldEnergy::Energy(e));
    }
}

impl VecFieldEnergy for [FieldEnergy] {
    fn has_particles(&self) -> bool {
        for x in self.iter() {
            match &x {
                FieldEnergy::Particle(_) => return true,
                _ => (),
            }
        };
        false
    }

    fn push_p(&mut self, _p: Particle) -> () {}
    fn push_e(&mut self, _e: f32) -> () {}
}

#[derive(Debug)]
pub enum FieldEnergy {
    Energy(f32),
    Particle(Particle),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Particle {
    element: Vision,
    n: f32,
}

impl Particle {
    pub fn new(element: Vision, n: f32) -> Self {
        Self {
            element,
            n,
        }
    }

    pub fn neutral(n: f32) -> Self {
        Self {
            element: Physical,
            n
        }
    }

    // do not consider the number of members
    pub fn on_field_energy(&self, reciver_element: &Vision) -> f32 {
        self.n * if self.element == *reciver_element {
            3.0
        // physical particles mean neutral energy
        } else if self.element == Physical {
            2.0
        } else {
            1.0
        }
    }

    pub fn off_field_energy(&self, reciver_element: &Vision) -> f32 {
        self.n * if self.element == *reciver_element {
            1.8
        } else if self.element == Physical {
            1.2
        } else {
            0.6
        }
    }
}

// The `Preference` enum means a preferable role in a battle for artifacts,
// weapons and characters. Implementations should allow some of them to prefer
// one or more roles (should be implemented like `Vec<Preference>`). If such
// `Preference` is empty, it will mean they prefer "everything", both `Attacker`
// and `Supporter`. The `Attacker` and `Supporter` roles may be determined by
// the position of the field members (see `simulate`).
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Preference {
    Pyro, Hydro, Electro, Cryo,
    Anemo, Geo, Dendro, Physical,
    Melee, Ranged,
    // pyro
    Amber,
    Bennett,
    Xiangling,
    Diluc,
    Klee,
    // hydro
    Barbara,
    Xingqiu,
    Mona,
    // electro
    Beidou,
    Fischl,
    Lisa,
    Razor,
    Keqing,
    // cryo
    Chongyun,
    Kaeya,
    Qiqi,
    // anemo
    Sucrose,
    TravelerAnemo,
    Jean,
    Venti,
    // geo
    Ningguang,
    Noelle,
    TravelerGeo,
    // version_1_1
    Tartaglia,
    Diona,
    Zhongli,
    Xinyan,
    // version_1_2
    Albedo,
    Ganyu,
    // version_1_3
    Xiao,
    HuTao,
    // version_1_4
    Rosaria,
    // version_1_5
    Yanfei,
    Eula,
    // version_1_6
    Kazuha,
    // version_2_0
    Ayaka,
    Yoimiya,
    Sayu,
    TravelerElectro,
    // version_2_1
    RaidenShogun,
    KujouSara,
    Aloy,
    SangonomiyaKokomi,
    // version_2_2
    Thoma,
    // version_2_3
    AratakiItto,
    Gorou,
}

impl PartialEq<Vision> for Preference {
    fn eq(&self, other: &Vision) -> bool {
        match (&self, &other) {
            (Preference::Pyro, Vision::Pyro) => true,
            (Preference::Hydro, Vision::Hydro) => true,
            (Preference::Electro, Vision::Electro) => true,
            (Preference::Cryo, Vision::Cryo) => true,
            (Preference::Anemo, Vision::Anemo) => true,
            (Preference::Geo, Vision::Geo) => true,
            (Preference::Dendro, Vision::Dendro) => true,
            (Preference::Physical, Vision::Physical) => true,
            _ => false,
        }
    }
}

impl PartialEq<WeaponType> for Preference {
    fn eq(&self, other: &WeaponType) -> bool {
        match (&self, &other) {
            (Preference::Melee, WeaponType::Sword) => true,
            (Preference::Melee, WeaponType::Claymore) => true,
            (Preference::Melee, WeaponType::Polearm) => true,
            (Preference::Ranged, WeaponType::Bow) => true,
            (Preference::Ranged, WeaponType::Catalyst) => true,
            _ => false,
        }
    }
}

impl PartialEq<str> for Preference {
    fn eq(&self, other: &str) -> bool {
        match (&self, other) {
            // pyro
            (Preference::Amber, "Amber") => true,
            (Preference::Bennett, "Bennett") => true,
            (Preference::Xiangling, "Xiangling") => true,
            (Preference::Diluc, "Diluc") => true,
            (Preference::Klee, "Klee") => true,
            // hydro
            (Preference::Barbara, "Barbara") => true,
            (Preference::Xingqiu, "Xingqiu") => true,
            (Preference::Mona, "Mona") => true,
            // electro
            (Preference::Beidou, "Beidou") => true,
            (Preference::Fischl, "Fischl") => true,
            (Preference::Lisa, "Lisa") => true,
            (Preference::Razor, "Razor") => true,
            (Preference::Keqing, "Keqing") => true,
            // cryo
            (Preference::Chongyun, "Chongyun") => true,
            (Preference::Kaeya, "Kaeya") => true,
            (Preference::Qiqi, "Qiqi") => true,
            // anemo
            (Preference::Sucrose, "Sucrose") => true,
            (Preference::TravelerAnemo, "Traveler (Anemo)") => true,
            (Preference::Jean, "Jean") => true,
            (Preference::Venti, "Venti") => true,
            // geo
            (Preference::Ningguang, "Ningguang") => true,
            (Preference::Noelle, "Noelle") => true,
            (Preference::Noelle, "Noelle (C6)") => true,
            (Preference::TravelerGeo, "Traveler (Geo)") => true,
            // version_1_1
            (Preference::Tartaglia, "Tartaglia") => true,
            (Preference::Diona, "Diona") => true,
            (Preference::Zhongli, "Zhongli") => true,
            (Preference::Xinyan, "Xinyan") => true,
            // version_1_2
            (Preference::Albedo, "Albedo") => true,
            (Preference::Ganyu, "Ganyu") => true,
            // version_1_3
            (Preference::Xiao, "Xiao") => true,
            (Preference::HuTao, "Hu Tao") => true,
            // version_1_4
            (Preference::Rosaria, "Rosaria") => true,
            // version_1_5
            (Preference::Yanfei, "Yanfei") => true,
            (Preference::Eula, "Eula") => true,
            // version_1_6
            (Preference::Kazuha, "Kazuha") => true,
            // version_2_0
            (Preference::Ayaka, "Ayaka") => true,
            (Preference::Yoimiya, "Yoimiya") => true,
            (Preference::Sayu, "Sayu") => true,
            (Preference::TravelerElectro, "Traveler (Electro)") => true,
            // version_2_1
            (Preference::RaidenShogun, "Raiden Shogun") => true,
            (Preference::KujouSara, "Kujou Sara") => true,
            (Preference::Aloy, "Aloy") => true,
            (Preference::SangonomiyaKokomi, "Sangonomiya Kokomi") => true,
            // version_2_2
            (Preference::Thoma, "Thoma") => true,
            // version_2_3
            (Preference::AratakiItto, "Arataki Itto") => true,
            (Preference::Gorou, "Gorou") => true,
            _ => false,
        }
    }
}

pub fn combination_filter(cr: &CharacterRecord, wr: &WeaponRecord, ar: &Artifact, args: &Args) -> bool {
    if cr.version > args.character_version ||
       wr.version > args.weapon_version ||
       ar.version > args.artifact_version {
        return false;
    }

    // check weapon
    if cr.weapon != wr.type_ {
        return false;
    }

    // check artifact
    let physical_attack = ar.is_physical_goblet_user(&cr.name);
    let mut result = if ar.preference.len() == 0 {
        true
    } else {
        false
    };
    for p in ar.preference.iter() {
        if p == &cr.vision
        || (physical_attack && p == &Preference::Physical)
        || p == &cr.weapon
        || p == cr.name {
            result = true;
            break;
        }
    }

    result
}


#[derive(Debug, Copy, Clone)]
pub struct UnstackableBuff(usize);

pub const NOBLESSE_OBLIGE: UnstackableBuff = UnstackableBuff(1 << 0);
pub const TENACITY_OF_THE_MILLELITH: UnstackableBuff = UnstackableBuff(1 << 1);
pub const MILLENNIAL_MOVEMENT_SERIES: UnstackableBuff = UnstackableBuff(1 << 2);

impl UnstackableBuff {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn turn_on(&mut self, other: &Self) -> &mut Self {
        self.0 |= other.0;
        self
    }

    pub fn turn_off(&mut self, other: &Self) -> &mut Self {
        self.0 &= !other.0;
        self
    }
}

impl PartialEq for UnstackableBuff {
    fn eq(&self, other: &UnstackableBuff) -> bool {
        (self.0 & other.0) != 0
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Vision {
    Pyro,
    Hydro,
    Electro,
    Cryo,
    Anemo,
    Geo,
    Dendro,
    Physical,
}

impl Vision {
    pub fn to_gauge(&self) -> &'static ElementalGauge {
        match self {
            Pyro => &PYRO_GAUGE1A,
            Cryo => &CRYO_GAUGE1A,
            Hydro => &HYDRO_GAUGE1A,
            Electro => &ELECTRO_GAUGE1A,
            Anemo => &ANEMO_GAUGE1A,
            Geo => &GEO_GAUGE1A,
            Dendro => &DENDRO_GAUGE1A,
            Physical => &PHYSICAL_GAUGE,
        }
    }
}

impl From<String> for Vision {
    fn from(key: String) -> Self {
        match key.as_str() {
            "Pyro" => Pyro,
            "Cryo" => Cryo,
            "Hydro" => Hydro,
            "Electro" => Electro,
            "Anemo" => Anemo,
            "Geo" => Geo,
            "Dendro" => Dendro,
            "Physical" => Physical,
            _ => panic!("invalid vision string: {:?}", key),
        }
    }
}

impl From<&String> for Vision {
    fn from(key: &String) -> Self {
        match key.as_str() {
            "Pyro" => Pyro,
            "Cryo" => Cryo,
            "Hydro" => Hydro,
            "Electro" => Electro,
            "Anemo" => Anemo,
            "Geo" => Geo,
            "Dendro" => Dendro,
            "Physical" => Physical,
            _ => panic!("invalid vision string: {:?}", key),
        }
    }
}

#[derive(Debug)]
pub struct PeriodicStack {
    start_time: f32,
    time: f32,
    interval: f32,
    duration: f32,
}

impl PeriodicStack {
    pub fn new(time: f32, interval: f32, duration: f32) -> Self {
        Self {
            start_time: time,
            time,
            interval,
            duration,
        }
    }

    pub fn disable() -> Self {
        Self {
            start_time: 9999.,
            time: 9999.,
            interval: 0.,
            duration: 0.,
        }
    }

    pub fn grant(&mut self, time: f32) -> u8 {
        if time >= self.time {
            if self.is_duration_valid() {
                self.time += self.interval;
            } else {
                // stop granting seals
                self.time = 9999.;
            }
            1
        } else {
            0
        }
    }

    pub fn is_duration_valid(&self) -> bool {
        self.time + self.interval - self.start_time <= self.duration
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

pub const SCORE: GearScore = GearScore { score: 140.0 };

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim2::characters;
    use crate::sim2::weapons;
    use crate::sim2::artifact;

    #[test]
    fn usb_eq() {
        assert_eq!(NOBLESSE_OBLIGE, UnstackableBuff(1));
    }

    #[test]
    fn usb_add() {
        let mut no = NOBLESSE_OBLIGE;
        let tm = TENACITY_OF_THE_MILLELITH;
        assert_eq!(*no.turn_on(&tm), UnstackableBuff(3));

        let mut tm = TENACITY_OF_THE_MILLELITH;
        let mm = MILLENNIAL_MOVEMENT_SERIES;
        assert_eq!(*tm.turn_on(&mm), UnstackableBuff(6));

        let mut mm = MILLENNIAL_MOVEMENT_SERIES;
        let no = NOBLESSE_OBLIGE;
        assert_eq!(*mm.turn_on(&no), UnstackableBuff(5));

        let mut no = NOBLESSE_OBLIGE;
        let tm = TENACITY_OF_THE_MILLELITH;
        let mm = MILLENNIAL_MOVEMENT_SERIES;
        assert_eq!(*no.turn_on(&tm).turn_on(&mm), UnstackableBuff(7));
    }

    #[test]
    fn filter_1() {
        let c = characters::pyro::Diluc::record();
        let w = weapons::claymore_4star::RainslasherR5::record();
        let a = artifact::ViridescentVenerer::record();
        let args = Args::default();
        assert!(!combination_filter(&c, &w, &a, &args));
    }

    #[test]
    fn filter_2() {
        let c = characters::pyro::Diluc::record();
        let w = weapons::claymore_4star::RainslasherR5::record();
        let a = artifact::GladiatorsFinale::record();
        let args = Args::default();
        assert!(combination_filter(&c, &w, &a, &args));
    }

    #[test]
    fn filter_3() {
        let c = characters::electro::Razor::record();
        let w = weapons::claymore_4star::RainslasherR5::record();
        let a = artifact::PaleFlame::record();
        let args = Args::default();
        assert!(combination_filter(&c, &w, &a, &args));
        // assert!(combination_filter_supporter(&c, &w, &a, &args));
    }

    #[test]
    fn filter_4() {
        let c = characters::electro::Razor::record();
        let w = weapons::claymore_4star::RainslasherR5::record();
        let a = artifact::ThunderingFury::record();
        let args = Args::default();
        assert!(combination_filter(&c, &w, &a, &args));
    }

    #[test]
    fn filter_5() {
        let c = characters::hydro::Xingqiu::record();
        let w = weapons::sword_4star::PrototypeRancourR5::record();
        let a = artifact::BlizzardStrayer::record();
        let args = Args::default();
        assert!(combination_filter(&c, &w, &a, &args));
    }

    #[test]
    fn filter_6() {
        let c = characters::cryo::Kaeya::record();
        let w = weapons::sword_4star::PrototypeRancourR5::record();
        let a = artifact::BlizzardStrayer::record();
        let args = Args::default();
        assert!(combination_filter(&c, &w, &a, &args));
    }
}
