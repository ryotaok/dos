use std::ops::{Add, AddAssign};
use std::cmp::PartialEq;

use crate::fc::{FieldCharacter};
use crate::action::Attack;

use self::AttackType::*;
use self::Vision::*;
use self::ElementalReactionType::*;

// DOC https://doc.rust-lang.org/std/marker/trait.Copy.html
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum AttackType {
    Na,
    Ca,
    // Plunge,
    PressSkill,
    HoldSkill,
    SkillDot,
    Burst,
    BurstDot,
    AdditionalAttack,
    StandStill,
}

#[derive(Debug, PartialEq)]
pub struct Energy(pub f32);

#[derive(Debug, PartialEq)]
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

impl PartialEq<String> for Preference {
    fn eq(&self, other: &String) -> bool {
        match (&self, other.as_str()) {
            (Preference::Melee, "Sword") => true,
            (Preference::Melee, "Claymore") => true,
            (Preference::Melee, "Polearm") => true,
            (Preference::Ranged, "Bow") => true,
            (Preference::Ranged, "Catalyst") => true,
            _ => false,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct UnstackableBuff(usize);

impl UnstackableBuff {
    pub fn new() -> Self {
        Self(0)
    }

    #[allow(non_snake_case)]
    pub fn NoblesseOblige() -> Self {
        Self(1)
    }

    #[allow(non_snake_case)]
    pub fn TenacityOfTheMillelith() -> Self {
        Self(10)
    }

    #[allow(non_snake_case)]
    pub fn MillennialMovementSeries() -> Self {
        Self(100)
    }
}

impl Add for UnstackableBuff {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl AddAssign for UnstackableBuff {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
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

impl ElementalGauge {
    pub fn new(aura: Vision, unit: f32, decay: ElementalGaugeDecay) -> Self {
        Self {
            aura,
            unit,
            decay
        }
    }

    pub fn physical() -> Self {
        Self {
            aura: Physical,
            unit: 1.0,
            decay: ElementalGaugeDecay::A,
        }
    }

    pub fn na(fc: &FieldCharacter) -> Self {
        Self {
            aura: if fc.state.infusion { fc.vision } else { Physical },
            unit: fc.cr.na_unit,
            decay: fc.cr.na_decay,
        }
    }

    pub fn ca(fc: &FieldCharacter) -> Self {
        let element = if fc.state.infusion
                      || fc.cr.weapon == "Bow" {
            fc.vision
        } else {
            Physical
        };
        Self {
            aura: element,
            unit: fc.cr.ca_unit,
            decay: fc.cr.ca_decay,
        }
    }

    pub fn skill(fc: &FieldCharacter) -> Self {
        Self {
            aura: fc.vision,
            unit: fc.cr.skill_unit,
            decay: fc.cr.skill_decay,
        }
    }

    pub fn skilldot(fc: &FieldCharacter) -> Self {
        Self {
            aura: fc.vision,
            unit: fc.cr.skilldot_unit,
            decay: fc.cr.skilldot_decay,
        }
    }

    pub fn burst(fc: &FieldCharacter) -> Self {
        Self {
            aura: fc.vision,
            unit: fc.cr.burst_unit,
            decay: fc.cr.burst_decay,
        }
    }

    pub fn burstdot(fc: &FieldCharacter) -> Self {
        Self {
            aura: fc.vision,
            unit: fc.cr.burstdot_unit,
            decay: fc.cr.burstdot_decay,
        }
    }

    pub fn trigger(&mut self, attack: &Attack, attack_element: &Vision) -> () {
        if attack.icd_cleared() {
            let other = &attack.element;
            let er = ElementalReaction::new(self.aura, *attack_element);
            let before_negative = self.unit <= 0.0;
            self.unit += er.gauge_modifier() * other.unit;
            let after_negative = self.unit <= 0.0;

            if after_negative {
                self.aura = Physical;
            } else if before_negative && !after_negative {
                // no aura was applied on the enemy
                self.aura = *attack_element;
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
            (Physical, _)       => Neutralize(Self { enemy_aura, trigger, rm: 0.0 }),
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
        assert_eq!(UnstackableBuff::NoblesseOblige(), UnstackableBuff(1));
    }

    #[test]
    fn usb_add() {
        let no = UnstackableBuff::NoblesseOblige();
        let tm = UnstackableBuff::TenacityOfTheMillelith();
        let mm = UnstackableBuff::MillennialMovementSeries();
        assert_eq!(no + tm, UnstackableBuff(11));
        assert_eq!(tm + mm, UnstackableBuff(110));
        assert_eq!(mm + no, UnstackableBuff(101));
        assert_eq!(no + tm + mm, UnstackableBuff(111));
    }
}
