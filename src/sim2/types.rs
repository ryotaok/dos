use std::cmp::PartialEq;

use crate::sim2::attack::Attack;
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
    Melee, Ranged, Attacker, Supporter,
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
}
