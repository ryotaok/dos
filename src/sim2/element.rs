use crate::sim2::attack::Attack;
use crate::sim2::types::Vision;

use Vision::*;
use self::ElementalReactionType::*;

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

    pub fn trigger2(&mut self, time: f32, last_time: &mut f32, other: &ElementalGauge) -> () {
        // decay over time
        if self.aura != Physical {
            self.unit -= (time - *last_time) / self.decay.decay_rate_conversion();
            *last_time = time;
            if self.unit < 0.0 {
                self.aura = Physical;
                self.unit = 0.0
            }
        }

        // reaction
        let er = ElementalReaction::new(self.aura, other.aura);
        match (&er, &other.aura) {
            // + incoming attack adds the aura
            (Equalize(_), Pyro) |
            (Equalize(_), Hydro) |
            (Equalize(_), Electro) |
            (Equalize(_), Cryo) => {
                self.aura = other.aura;
                self.unit = other.unit;
                // self.decay = other.decay;
                *last_time = time;
            },
            // = incoming attack does not change the aura
            (Neutralize(_), _) => (),
            // - incoming attack reduce the aura
            _ => {
                // TODO test this
                match &er {
                    Freeze(_) => self.aura = Cryo,
                    ElectorCharged(_) => self.aura = Hydro,
                    _ => (),
                };
                self.unit += other.unit * er.gauge_modifier();
                if self.unit < 0.0 {
                    self.aura = Physical;
                }
            },
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
    pub enemy_aura: Vision,
    pub trigger: Vision,
    pub attack: Vision,
    // reaction multiplier
    rm: f32,
}

impl ElementalReaction {
    pub fn new(enemy_aura: Vision, trigger: Vision) -> ElementalReactionType {
        match (&enemy_aura, &trigger) {
            (Physical, Physical)=> Neutralize(Self { enemy_aura, trigger, attack: trigger, rm: 0.0 }),
            (Physical, _)       => Equalize(Self { enemy_aura, trigger, attack: trigger, rm: 0.0 }),
            (Pyro, Pyro)        => Equalize(Self { enemy_aura, trigger, attack: trigger, rm: 0.0 }),
            (Pyro, Hydro)       => Vaporize(Self { enemy_aura, trigger, attack: trigger, rm: 1.0 }),
            (Pyro, Electro)     => Overloaded(Self { enemy_aura, trigger, attack: Pyro, rm: 4.0 }),
            (Pyro, Cryo)        => Melt(Self { enemy_aura, trigger, attack: trigger, rm: 0.5 }),
            (Pyro, Anemo)       => Swirl(Self { enemy_aura, trigger, attack: Pyro, rm: 1.2 }),
            (Pyro, Geo)         => Crystallize(Self { enemy_aura, trigger, attack: trigger, rm: 0.0 }),
            (Pyro, Dendro)      => Burn(Self { enemy_aura, trigger, attack: Pyro, rm: 1.0 }),
            (Pyro, Physical)    => Neutralize(Self { enemy_aura, trigger, attack: trigger, rm: 0.0 }),

            (Hydro, Pyro)       => Vaporize(Self { enemy_aura, trigger, attack: trigger, rm: 0.5 }),
            (Hydro, Hydro)      => Equalize(Self { enemy_aura, trigger, attack: trigger, rm: 0.0 }),
            (Hydro, Electro)    => ElectorCharged(Self { enemy_aura, trigger, attack: Electro, rm: 2.4 }),
            (Hydro, Cryo)       => Freeze(Self { enemy_aura, trigger, attack: trigger, rm: 0.0 }),
            (Hydro, Anemo)      => Swirl(Self { enemy_aura, trigger, attack: trigger, rm: 0.0 }), // Hydro Swirl only spreads hydro aura.
            (Hydro, Geo)        => Crystallize(Self { enemy_aura, trigger, attack: trigger, rm: 0.0 }),
            (Hydro, Dendro)     => Neutralize(Self { enemy_aura, trigger, attack: trigger, rm: 0.0 }),
            (Hydro, Physical)   => Neutralize(Self { enemy_aura, trigger, attack: trigger, rm: 0.0 }),

            (Electro, Pyro)     => Overloaded(Self { enemy_aura, trigger, attack: Pyro, rm: 4.0 }),
            (Electro, Hydro)    => ElectorCharged(Self { enemy_aura, trigger, attack: Electro, rm: 2.4 }),
            (Electro, Electro)  => Equalize(Self { enemy_aura, trigger, attack: trigger, rm: 0.0 }),
            (Electro, Cryo)     => Superconduct(Self { enemy_aura, trigger, attack: Cryo, rm: 1.0 }),
            (Electro, Anemo)    => Swirl(Self { enemy_aura, trigger, attack: Electro, rm: 1.2 }),
            (Electro, Geo)      => Crystallize(Self { enemy_aura, trigger, attack: trigger, rm: 0.0 }),
            (Electro, Dendro)   => Neutralize(Self { enemy_aura, trigger, attack: trigger, rm: 0.0 }),
            (Electro, Physical) => Neutralize(Self { enemy_aura, trigger, attack: trigger, rm: 0.0 }),

            (Cryo, Pyro)        => Melt(Self { enemy_aura, trigger, attack: trigger, rm: 1.0 }),
            (Cryo, Hydro)       => Freeze(Self { enemy_aura, trigger, attack: trigger, rm: 0.0 }),
            (Cryo, Electro)     => Superconduct(Self { enemy_aura, trigger, attack: Cryo, rm: 1.0 }),
            (Cryo, Cryo)        => Equalize(Self { enemy_aura, trigger, attack: trigger, rm: 0.0 }),
            (Cryo, Anemo)       => Swirl(Self { enemy_aura, trigger, attack: Cryo, rm: 0.0 }),
            (Cryo, Geo)         => Crystallize(Self { enemy_aura, trigger, attack: trigger, rm: 0.0 }),
            (Cryo, Dendro)      => Neutralize(Self { enemy_aura, trigger, attack: trigger, rm: 0.0 }),
            (Cryo, Physical)    => Neutralize(Self { enemy_aura, trigger, attack: trigger, rm: 0.0 }),

            _ => Neutralize(Self { enemy_aura, trigger, attack: trigger, rm: 0.0 }),
        }
    }

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
