use std::cmp::PartialEq;

use crate::action::Attack;

// use self::AttackType::*;
use self::Vision::*;
use self::ElementalReactionType::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum WeaponType {
    Sword,
    Claymore,
    Polearm,
    Bow,
    Catalyst,
}

// DOC https://doc.rust-lang.org/std/marker/trait.Copy.html
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum AttackType {
    Na,
    Ca,
    // Plunge,
    Skill,
    PressSkill,
    HoldSkill,
    SkillDot,
    Burst,
    BurstDot,
    AdditionalAttack,
    StandStill,
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

    // TODO forget about number of members
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
pub const THUNDERSOOTHER: UnstackableBuff = UnstackableBuff(1 << 3);
pub const LAVAWALKER: UnstackableBuff = UnstackableBuff(1 << 4);
pub const BLIZZARDSTRAYER1: UnstackableBuff = UnstackableBuff(1 << 5);
pub const BLIZZARDSTRAYER2: UnstackableBuff = UnstackableBuff(1 << 6);
pub const LIONSROAR: UnstackableBuff = UnstackableBuff(1 << 7);
pub const RAINSLASHER: UnstackableBuff = UnstackableBuff(1 << 8);
pub const DRAGONSBANE: UnstackableBuff = UnstackableBuff(1 << 9);

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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ElementalGaugeDecay { A, B, C, }

impl ElementalGaugeDecay {
    pub fn decay_rate_conversion(&self) -> f32 {
        match self {
            ElementalGaugeDecay::A => 9.5,
            ElementalGaugeDecay::B => 6.0,
            ElementalGaugeDecay::C => 4.25,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ElementalGauge {
    pub aura: Vision,
    pub unit: f32,
    pub decay: ElementalGaugeDecay,
}

pub const PHYSICAL_GAUGE: ElementalGauge = ElementalGauge { aura: Physical, unit: 1.0, decay: ElementalGaugeDecay::A, };

pub const PYRO_GAUGE1A: ElementalGauge = ElementalGauge { aura: Pyro, unit: 1.0, decay: ElementalGaugeDecay::A, };
pub const PYRO_GAUGE2B: ElementalGauge = ElementalGauge { aura: Pyro, unit: 2.0, decay: ElementalGaugeDecay::B, };
pub const PYRO_GAUGE4C: ElementalGauge = ElementalGauge { aura: Pyro, unit: 4.0, decay: ElementalGaugeDecay::C, };

pub const HYDRO_GAUGE1A: ElementalGauge = ElementalGauge { aura: Hydro, unit: 1.0, decay: ElementalGaugeDecay::A, };
pub const HYDRO_GAUGE2B: ElementalGauge = ElementalGauge { aura: Hydro, unit: 2.0, decay: ElementalGaugeDecay::B, };
pub const HYDRO_GAUGE4C: ElementalGauge = ElementalGauge { aura: Hydro, unit: 4.0, decay: ElementalGaugeDecay::C, };

pub const ELECTRO_GAUGE1A: ElementalGauge = ElementalGauge { aura: Electro, unit: 1.0, decay: ElementalGaugeDecay::A, };
pub const ELECTRO_GAUGE2B: ElementalGauge = ElementalGauge { aura: Electro, unit: 2.0, decay: ElementalGaugeDecay::B, };
pub const ELECTRO_GAUGE4C: ElementalGauge = ElementalGauge { aura: Electro, unit: 4.0, decay: ElementalGaugeDecay::C, };

pub const CRYO_GAUGE1A: ElementalGauge = ElementalGauge { aura: Cryo, unit: 1.0, decay: ElementalGaugeDecay::A, };
pub const CRYO_GAUGE2B: ElementalGauge = ElementalGauge { aura: Cryo, unit: 2.0, decay: ElementalGaugeDecay::B, };
pub const CRYO_GAUGE4C: ElementalGauge = ElementalGauge { aura: Cryo, unit: 4.0, decay: ElementalGaugeDecay::C, };

pub const ANEMO_GAUGE1A: ElementalGauge = ElementalGauge { aura: Anemo, unit: 1.0, decay: ElementalGaugeDecay::A, };
pub const ANEMO_GAUGE2B: ElementalGauge = ElementalGauge { aura: Anemo, unit: 2.0, decay: ElementalGaugeDecay::B, };
pub const ANEMO_GAUGE4C: ElementalGauge = ElementalGauge { aura: Anemo, unit: 4.0, decay: ElementalGaugeDecay::C, };

pub const GEO_GAUGE1A: ElementalGauge = ElementalGauge { aura: Geo, unit: 1.0, decay: ElementalGaugeDecay::A, };
pub const GEO_GAUGE2B: ElementalGauge = ElementalGauge { aura: Geo, unit: 2.0, decay: ElementalGaugeDecay::B, };
pub const GEO_GAUGE4C: ElementalGauge = ElementalGauge { aura: Geo, unit: 4.0, decay: ElementalGaugeDecay::C, };

pub const DENDRO_GAUGE1A: ElementalGauge = ElementalGauge { aura: Dendro, unit: 1.0, decay: ElementalGaugeDecay::A, };
pub const DENDRO_GAUGE2B: ElementalGauge = ElementalGauge { aura: Dendro, unit: 2.0, decay: ElementalGaugeDecay::B, };
pub const DENDRO_GAUGE4C: ElementalGauge = ElementalGauge { aura: Dendro, unit: 4.0, decay: ElementalGaugeDecay::C, };

impl ElementalGauge {
    pub fn new(aura: Vision, unit: f32, decay: ElementalGaugeDecay) -> Self {
        Self {
            aura,
            unit,
            decay
        }
    }

    pub fn trigger(&mut self, attack: &Attack) -> () {
        if attack.icd_cleared() {
            let other = &attack.element;
            let er = ElementalReaction::new(self.aura, other.aura);
            let before_negative = self.unit <= 0.0;
            let unit = er.gauge_modifier() * other.unit;
            if unit > 0.0 {
                // unit can be up to 4GU.
                if self.unit < unit {
                    self.unit = unit;
                    self.decay = other.decay;
                }
            } else {
                self.unit += unit;
            }
            let after_negative = self.unit <= 0.0;

            if after_negative {
                self.aura = Physical;
            } else if before_negative && !after_negative {
                // no aura was applied on the enemy
                self.aura = other.aura;
                self.unit = other.unit;
                self.decay = other.decay;
            }
            // TODO test this
            match er {
                Freeze(_) => self.aura = Cryo,
                ElectorCharged(_) => self.aura = Hydro,
                _ => (),
            };
        }
    }

    pub fn update(&mut self, time: f32) -> () {
        self.unit -= time / self.decay.decay_rate_conversion();
        if self.unit < 0.0 {
            self.aura = Physical;
            self.unit = 0.0
        }
    }
}

impl Default for ElementalGauge {
    fn default() -> Self {
        Self {
            aura: Physical,
            unit: 0.0,
            decay: ElementalGaugeDecay::A,
        }
    }
}

#[derive(Debug)]
pub enum ElementalReactionType {
    Overloaded(ElementalReaction),
    Shatter(ElementalReaction),
    ElectorCharged(ElementalReaction),
    Swirl(ElementalReaction),
    Superconduct(ElementalReaction),
    Vaporize(ElementalReaction),
    Melt(ElementalReaction),
    Burn(ElementalReaction),
    Freeze(ElementalReaction),
    Crystallize(ElementalReaction),
    Equalize(ElementalReaction),
    Neutralize(ElementalReaction),
}

impl ElementalReactionType {
    pub fn gauge_modifier(&self) -> f32 {
        match self {
            Overloaded(_) => -1.25,
            Shatter(_) => -1.0,
            ElectorCharged(_) => -0.4,
            Swirl(_) => -0.625,
            Superconduct(_) => -1.25,
            Vaporize(er) if er.enemy_aura == Pyro => -2.5,
            Vaporize(_) => -0.625,
            Melt(er) if er.enemy_aura == Cryo => -2.5,
            Melt(_) => -0.625,
            Crystallize(_) => -0.625,
            Freeze(_) => 0.0,
            Burn(_) => -0.4,
            Equalize(_) => 1.0,
            Neutralize(_) => 0.0,
        }
    }

    pub fn is_triggered(&self) -> bool {
        match self {
            Equalize(_) | Neutralize(_) => false,
            _ => true,
        }
    }

    // pub fn is_pyro(&self) -> bool {
    //     match self {
    //         Overloaded(_) | Vaporize(_) | Melt(_) => true,
    //         Swirl(er) | Crystallize(er) => er.enemy_aura == Pyro,
    //         _ => false,
    //     }
    // }

    // pub fn is_hydro(&self) -> bool {
    //     match self {
    //         ElectorCharged(_) | Vaporize(_) | Freeze(_) => true,
    //         Swirl(er) | Crystallize(er) => er.enemy_aura == Hydro,
    //         _ => false,
    //     }
    // }

    pub fn is_electro(&self) -> bool {
        match self {
            Overloaded(_) | ElectorCharged(_) | Superconduct(_) => true,
            Swirl(er) | Crystallize(er) => er.enemy_aura == Electro,
            _ => false,
        }
    }

    // pub fn is_cryo(&self) -> bool {
    //     match self {
    //         Melt(_) | Freeze(_) | ElectorCharged(_) => true,
    //         Swirl(er) | Crystallize(er) => er.enemy_aura == Cryo,
    //         _ => false,
    //     }
    // }

    pub fn is_swirl(&self) -> bool {
        match self {
            Swirl(_) => true,
            _ => false,
        }
    }

    pub fn is_crystallize(&self) -> bool {
        match self {
            Crystallize(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct ElementalReaction {
    enemy_aura: Vision,
    trigger: Vision,
    // reaction multiplier
    rm: f32,
}

impl ElementalReaction {
    pub fn new(enemy_aura: Vision, trigger: Vision) -> ElementalReactionType {
        match (&enemy_aura, &trigger) {
            (Physical, Physical)=> Neutralize(Self { enemy_aura, trigger, rm: 0.0 }),
            (Physical, _)       => Equalize(Self { enemy_aura, trigger, rm: 0.0 }),
            (Pyro, Pyro)        => Equalize(Self { enemy_aura, trigger, rm: 0.0 }),
            (Pyro, Hydro)       => Vaporize(Self { enemy_aura, trigger, rm: 2.0 }),
            (Pyro, Electro)     => Overloaded(Self { enemy_aura, trigger, rm: 4.0 }),
            (Pyro, Cryo)        => Melt(Self { enemy_aura, trigger, rm: 1.5 }),
            (Pyro, Anemo)       => Swirl(Self { enemy_aura, trigger, rm: 1.2 }),
            (Pyro, Geo)         => Crystallize(Self { enemy_aura, trigger, rm: 0.0 }),
            (Pyro, Dendro)      => Burn(Self { enemy_aura, trigger, rm: 1.0 }),
            (Pyro, Physical)    => Neutralize(Self { enemy_aura, trigger, rm: 0.0 }),

            (Hydro, Pyro)       => Vaporize(Self { enemy_aura, trigger, rm: 1.5 }),
            (Hydro, Hydro)      => Equalize(Self { enemy_aura, trigger, rm: 0.0 }),
            (Hydro, Electro)    => ElectorCharged(Self { enemy_aura, trigger, rm: 2.4 }),
            (Hydro, Cryo)       => Freeze(Self { enemy_aura, trigger, rm: 0.0 }),
            (Hydro, Anemo)      => Swirl(Self { enemy_aura, trigger, rm: 1.2 }),
            (Hydro, Geo)        => Crystallize(Self { enemy_aura, trigger, rm: 0.0 }),
            (Hydro, Dendro)     => Neutralize(Self { enemy_aura, trigger, rm: 0.0 }),
            (Hydro, Physical)   => Neutralize(Self { enemy_aura, trigger, rm: 0.0 }),

            (Electro, Pyro)     => Overloaded(Self { enemy_aura, trigger, rm: 4.0 }),
            (Electro, Hydro)    => ElectorCharged(Self { enemy_aura, trigger, rm: 2.4 }),
            (Electro, Electro)  => Equalize(Self { enemy_aura, trigger, rm: 0.0 }),
            (Electro, Cryo)     => Superconduct(Self { enemy_aura, trigger, rm: 1.0 }),
            (Electro, Anemo)    => Swirl(Self { enemy_aura, trigger, rm: 1.2 }),
            (Electro, Geo)      => Crystallize(Self { enemy_aura, trigger, rm: 0.0 }),
            (Electro, Dendro)   => Neutralize(Self { enemy_aura, trigger, rm: 0.0 }),
            (Electro, Physical) => Neutralize(Self { enemy_aura, trigger, rm: 0.0 }),

            (Cryo, Pyro)        => Melt(Self { enemy_aura, trigger, rm: 2.0 }),
            (Cryo, Hydro)       => Freeze(Self { enemy_aura, trigger, rm: 0.0 }),
            (Cryo, Electro)     => Superconduct(Self { enemy_aura, trigger, rm: 1.0 }),
            (Cryo, Cryo)        => Equalize(Self { enemy_aura, trigger, rm: 0.0 }),
            (Cryo, Anemo)       => Swirl(Self { enemy_aura, trigger, rm: 0.0 }),
            (Cryo, Geo)         => Crystallize(Self { enemy_aura, trigger, rm: 0.0 }),
            (Cryo, Dendro)      => Neutralize(Self { enemy_aura, trigger, rm: 0.0 }),
            (Cryo, Physical)    => Neutralize(Self { enemy_aura, trigger, rm: 0.0 }),

            _ => Neutralize(Self { enemy_aura, trigger, rm: 0.0 }),
        }
    }

    // TODO level is fixed to 90
    pub fn transformative_reaction(&self, em: f32, bonus: f32) -> f32 {
        let bonus = 1.0 + (16.0 * em) / (2000.0 + em) + bonus / 100.0;
        let level_multiplier = 725.36;
        self.rm * bonus * level_multiplier
    }

    pub fn amplifying_reaction(&self, em: f32, bonus: f32) -> f32 {
        let bonus = 1.0 + (2.78 * em) / (1400.0 + em) + bonus / 100.0;
        self.rm * bonus
    }
}

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
