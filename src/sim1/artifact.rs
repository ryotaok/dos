use crate::sim1::state::{State, GearScore};
use crate::sim1::fc::{FieldCharacterIndex, CharacterData, SpecialAbility, Enemy};
use crate::sim1::action::{Attack, AttackEvent, ICDTimer, NTimer, DurationTimer};
use crate::sim1::types::{AttackType, WeaponType, FieldEnergy, ElementalReaction, Preference, Vision, NOBLESSE_OBLIGE, TENACITY_OF_THE_MILLELITH, THUNDERSOOTHER, LAVAWALKER, BLIZZARDSTRAYER1, BLIZZARDSTRAYER2};

use AttackType::*;

const SCORE: GearScore = GearScore { score: 140.0 };

#[derive(Debug)]
pub struct Artifact {
    pub name: &'static str,
    pub version: f32,
    pub preference: &'static [Preference],
    pub state: State,
}

impl Default for Artifact {
    fn default() -> Self {
        Self {
            name: "",
            version: 1.0,
            preference: &[],
            state: State::new()
        }
    }
}

impl Artifact {
    pub fn infuse_goblet(&mut self, vision: &Vision) -> &mut Self {
        match &vision {
            Vision::Pyro => self.state.pyro_dmg = 46.6,
            Vision::Cryo => self.state.cryo_dmg = 46.6,
            Vision::Hydro => self.state.hydro_dmg = 46.6,
            Vision::Electro => self.state.electro_dmg = 46.6,
            Vision::Anemo => self.state.anemo_dmg = 46.6,
            Vision::Geo => self.state.geo_dmg = 46.6,
            Vision::Dendro => self.state.dendro_dmg = 46.6,
            Vision::Physical => self.state.physical_dmg = 58.3,
        };
        self
    }

    pub fn dry_goblet(&mut self) -> () {
        self.state.pyro_dmg = 0.0;
        self.state.cryo_dmg = 0.0;
        self.state.hydro_dmg = 0.0;
        self.state.electro_dmg = 0.0;
        self.state.anemo_dmg = 0.0;
        self.state.geo_dmg = 0.0;
        self.state.dendro_dmg = 0.0;
        self.state.physical_dmg = 0.0;
    }

    pub fn setup(&mut self) -> () {
        // default setup for all artifacts
        self.state.flat_atk += 311.0;
    }
}

pub fn all(idx: FieldCharacterIndex) -> Vec<(Artifact, Box<dyn SpecialAbility>)> {
    vec![
    (BloodstainedChivalry::record(), Box::new(BloodstainedChivalry)),
    (Bcpf::record(), Box::new(Bcpf)),
    (ThunderingFury::record(), Box::new(ThunderingFury::new())),
    (ViridescentVenerer::record(), Box::new(ViridescentVenerer::new())),
    (VVem::record(), Box::new(VVem::new())),
    (ArchaicPetra::record(), Box::new(ArchaicPetra::new())),
    (CrimsonWitchOfFlames::record(), Box::new(CrimsonWitchOfFlames::new(idx))),
    (CrimsonWitchOfFlamesHp::record(), Box::new(CrimsonWitchOfFlamesHp::new(idx))),
    (NoblesseOblige::record(), Box::new(NoblesseOblige::new())),
    (Gfno::record(), Box::new(Gfno)),
    (GladiatorsFinale::record(), Box::new(GladiatorsFinale::new())),
    (GladiatorsFinaleDef::record(), Box::new(GladiatorsFinaleDef::new())),
    (WanderersTroupe::record(), Box::new(WanderersTroupe)),
    (RetracingBolide::record(), Box::new(RetracingBolide)),
    (RetracingBolideDef::record(), Box::new(RetracingBolideDef)),
    (Thundersoother::record(), Box::new(Thundersoother::new(idx))),
    (Lavawalker::record(), Box::new(Lavawalker::new(idx))),
    (LavawalkerHp::record(), Box::new(LavawalkerHp::new(idx))),
    (Gfelm::record(), Box::new(Gfelm)),
    (GfelmEr::record(), Box::new(GfelmEr)),
    (GfelmEr2::record(), Box::new(GfelmEr2)),
    (GfelmHpCr::record(), Box::new(GfelmHpCr)),
    (BlizzardStrayer::record(), Box::new(BlizzardStrayer::new(idx))),
    (HeartOfDepth::record(), Box::new(HeartOfDepth::new(idx))),
    (GlacierAndSnowfield::record(), Box::new(GlacierAndSnowfield::new(idx))),
    (PaleFlame::record(), Box::new(PaleFlame::new(idx))),
    (TenacityOfTheMillelith::record(), Box::new(TenacityOfTheMillelith::new())),
    (ShimenawasReminiscence::record(), Box::new(ShimenawasReminiscence::new(idx))),
    (GfShimenawa::record(), Box::new(GfShimenawa)),
    (EmblemOfSeveredFate::record(), Box::new(EmblemOfSeveredFate::new(idx))),
    (EmblemOfSeveredFateER::record(), Box::new(EmblemOfSeveredFateER::new(idx))),
    (EmblemOfSeveredFateER2::record(), Box::new(EmblemOfSeveredFateER2::new(idx))),
    ]
}

#[derive(Debug)]
pub struct BloodstainedChivalry;

impl SpecialAbility for BloodstainedChivalry {}

impl BloodstainedChivalry {
    pub fn record() -> Artifact {
        Artifact {
            name: "Bloodstained Chivalry",
            version: 1.0,
            preference: &[Preference::Physical],
            state: State::new().physical_dmg(25.0)
                    .atk(SCORE.atk(40.0))
                    .cr(SCORE.cr(60.0))
        }
    }
}

#[derive(Debug)]
pub struct Bcpf;

impl SpecialAbility for Bcpf {}

impl Bcpf {
    pub fn record() -> Artifact {
        Artifact {
            name: "BCPF Physical 50%",
            version: 1.0,
            preference: &[Preference::Physical],
            state: State::new().physical_dmg(50.0)
                    .atk(SCORE.atk(40.0))
                    .cr(SCORE.cr(60.0))
        }
    }
}

#[derive(Debug)]
pub struct ThunderingFury {
    timer: NTimer,
}

impl ThunderingFury {
    pub fn new() -> Self {
        Self {
            timer: NTimer::new(&[0.8])
        }
    }

    pub fn record() -> Artifact {
        Artifact {
            name: "Thundering Fury",
            version: 1.0,
            preference: &[Preference::Electro],
            state: State::new().electro_dmg(15.0).transformative_bonus(40.0)
                    .atk(SCORE.atk(40.0))
                    .cr(SCORE.cr(60.0))
        }
    }
}

pub fn thundering_fury_fn(timer: &mut NTimer) -> () {
    // reduce skill CD by 1 second
    timer.update(1.0, true);
}

impl SpecialAbility for ThunderingFury {
    fn update(&mut self, time: f32, _event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], _particles: &[FieldEnergy], enemy: &Enemy) -> () {
        let mut should_update = false;
        unsafe {
            for &a in attack {
                let atk = & *a;
                if atk.idx == data.idx && ElementalReaction::new(enemy.aura.aura, atk.element.aura).is_electro() {
                    should_update = true;
                    break;
                }
            }
        }
        self.timer.update(time, should_update);
    }

    fn accelerator(&self) -> Option<fn(&mut NTimer)> {
        if self.timer.ping && self.timer.n == 1 {
            Some(thundering_fury_fn)
        } else {
            None
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

#[derive(Debug)]
pub struct ViridescentVenerer {
    timer: DurationTimer,
    element: Vision,
}

impl ViridescentVenerer {
    pub fn new() -> Self {
        Self {
            timer: DurationTimer::new(10.0, &[0.0]),
            element: Vision::Physical,
        }
    }

    pub fn record() -> Artifact {
        Artifact {
            name: "Viridescent Venerer",
            version: 1.0,
            preference: &[Preference::Anemo],
            state: State::new().anemo_dmg(15.0).transformative_bonus(60.0)
                    .atk(SCORE.atk(40.0))
                    .cr(SCORE.cr(60.0))
        }
    }
}

impl SpecialAbility for ViridescentVenerer {
    fn update(&mut self, time: f32, _event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], _particles: &[FieldEnergy], enemy: &Enemy) -> () {
        let mut should_update = false;
        unsafe {
            for &a in attack {
                let atk = & *a;
                if atk.idx == data.idx && enemy.trigger_er(&atk.element.aura).is_swirl() {
                    self.element = enemy.aura.aura;
                    should_update = true;
                    break;
                }
            }
        }
        self.timer.update(time, should_update);
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if self.timer.ping {
            match (self.timer.n, &self.element) {
                (1, Vision::Pyro) => enemy.debuff.pyro += 40.0,
                (1, Vision::Hydro) => enemy.debuff.hydro += 40.0,
                (1, Vision::Electro) => enemy.debuff.electro += 40.0,
                (1, Vision::Cryo) => enemy.debuff.cryo += 40.0,
                (0, Vision::Pyro) => enemy.debuff.pyro -= 40.0,
                (0, Vision::Hydro) => enemy.debuff.hydro -= 40.0,
                (0, Vision::Electro) => enemy.debuff.electro -= 40.0,
                (0, Vision::Cryo) => enemy.debuff.cryo -= 40.0,
                _ => (),
            }
        }
    }
}

#[derive(Debug)]
pub struct VVem(ViridescentVenerer);

impl VVem {
    pub fn new() -> Self {
        Self(ViridescentVenerer::new())
    }

    pub fn record() -> Artifact {
        Artifact {
            name: "Viridescent Venerer (EM)",
            version: 1.0,
            preference: &[Preference::Anemo],
            state: State::new().anemo_dmg(15.0).transformative_bonus(60.0)
                    .atk(SCORE.atk(10.0))
                    .cr(SCORE.cr(10.0))
                    .em(SCORE.em(80.0))
        }
    }
}

impl SpecialAbility for VVem {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.0.update(time, event, data, attack, particles, enemy);
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_data, enemy);
    }
}

#[derive(Debug)]
pub struct ArchaicPetra {
    timer: DurationTimer,
}

impl ArchaicPetra {
    pub fn new() -> Self {
        Self {
            timer: DurationTimer::new(10.0, &[0.0])
        }
    }

    pub fn record() -> Artifact {
        Artifact {
            name: "Archaic Petra",
            version: 1.0,
            preference: &[Preference::Geo],
            state: State::new().geo_dmg(15.0)
                    .atk(SCORE.atk(40.0))
                    .cr(SCORE.cr(60.0))
        }
    }
}

impl SpecialAbility for ArchaicPetra {
    fn update(&mut self, time: f32, _event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], _particles: &[FieldEnergy], enemy: &Enemy) -> () {
        let mut should_update = false;
        unsafe {
            for &a in attack {
                let atk = & *a;
                if atk.idx == data.idx && ElementalReaction::new(enemy.aura.aura, atk.element.aura).is_crystallize() {
                    should_update = true;
                    break;
                }
            }
        }
        self.timer.update(time, should_update);
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if self.timer.n == 1 {
            for data in modifiable_data.iter_mut() {
                data.state.pyro_dmg += 35.0;
                data.state.hydro_dmg += 35.0;
                data.state.electro_dmg += 35.0;
                data.state.cryo_dmg += 35.0;
            }
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

#[derive(Debug)]
pub struct CrimsonWitchOfFlames {
    idx: FieldCharacterIndex,
    timer: DurationTimer,
}

impl CrimsonWitchOfFlames {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx,
            timer: DurationTimer::new(10.0, &[0.0, 0.0, 0.0])
        }
    }

    pub fn record() -> Artifact {
        Artifact {
            name: "Crimson Witch of Flames",
            version: 1.0,
            preference: &[Preference::Pyro],
            state: State::new().pyro_dmg(15.0).amplifying_bonus(15.0).transformative_bonus(40.0)
                    .atk(SCORE.atk(40.0))
                    .cr(SCORE.cr(60.0))
        }
    }
}

impl SpecialAbility for CrimsonWitchOfFlames {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        let should_update = event.idx == self.idx && (event.kind == PressSkill || event.kind == HoldSkill);
        self.timer.update(time, should_update);
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if self.timer.n > 0 {
            modifiable_data[self.idx.0].state.pyro_dmg += 7.5 * self.timer.n as f32;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

#[derive(Debug)]
pub struct CrimsonWitchOfFlamesHp(CrimsonWitchOfFlames);

impl CrimsonWitchOfFlamesHp {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self(CrimsonWitchOfFlames::new(idx))
    }

    pub fn record() -> Artifact {
        Artifact {
            name: "Crimson Witch of Flames (HP)",
            version: 1.0,
            preference: &[Preference::Pyro],
            state: State::new().pyro_dmg(15.0).amplifying_bonus(15.0).transformative_bonus(40.0)
                    .hp(SCORE.hp(40.0))
                    .cr(SCORE.cr(60.0))
        }
    }
}

impl SpecialAbility for CrimsonWitchOfFlamesHp {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.0.update(time, event, data, attack, particles, enemy);
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_data, enemy);
    }

    fn reset(&mut self) -> () {
        self.0.reset();
    }
}

#[derive(Debug)]
pub struct NoblesseOblige {
    timer: DurationTimer
}

impl NoblesseOblige {
    pub fn new() -> Self {
        Self { timer: DurationTimer::new(12.0, &[0.0]) }
    }

    pub fn record() -> Artifact {
        Artifact {
            name: "Noblesse Oblige",
            version: 1.0,
            preference: &[Preference::Supporter],
            state: State::new().burst_dmg(20.0)
                    .atk(SCORE.atk(40.0))
                    .cr(SCORE.cr(60.0))
        }
    }
}

impl SpecialAbility for NoblesseOblige {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        let should_update = event.idx == data.idx && event.kind == Burst;
        self.timer.update(time, should_update);
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if self.timer.n == 1 {
            for data in modifiable_data.iter_mut() {
                if data.state.stacked_buff != NOBLESSE_OBLIGE {
                    data.state.atk += 20.0;
                    data.state.stacked_buff.turn_on(&NOBLESSE_OBLIGE);
                }
            }
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}


#[derive(Debug)]
pub struct Gfno;

impl SpecialAbility for Gfno {}

impl Gfno {
    pub fn record() -> Artifact {
        Artifact {
            name: "GFNO ATK 18% Burst 20%",
            version: 1.0,
            preference: &[Preference::Supporter],
            state: State::new().burst_dmg(20.0)
                    .atk(18.0 + SCORE.atk(40.0))
                    .cr(SCORE.cr(60.0))
        }
    }
}

#[derive(Debug)]
pub struct GladiatorsFinale {
    bonus: f32,
    checked: bool,
}

impl GladiatorsFinale {
    pub fn new() -> Self {
        Self { bonus: 0.0, checked: false }
    }

    pub fn record() -> Artifact {
        Artifact {
            name: "Gladiator's Finale",
            version: 1.0,
            preference: &[Preference::Melee],
            state: State::new()
                    .na_dmg(35.0)
                    .atk(18.0 + SCORE.atk(40.0))
                    .cr(SCORE.cr(60.0))
        }
    }
}

impl SpecialAbility for GladiatorsFinale {}

#[derive(Debug)]
pub struct GladiatorsFinaleDef(GladiatorsFinale);

impl GladiatorsFinaleDef {
    fn new() -> Self {
        Self(GladiatorsFinale::new())
    }

    pub fn record() -> Artifact {
        Artifact {
            name: "Gladiator's Finale (DEF)",
            version: 1.0,
            preference: &[Preference::Melee],
            state: State::new()
                    .na_dmg(35.0)
                    .atk(18.0)
                    .def(SCORE.def(40.0))
                    .cr(SCORE.cr(60.0))
        }
    }
}

impl SpecialAbility for GladiatorsFinaleDef {}

#[derive(Debug)]
pub struct WanderersTroupe;

impl SpecialAbility for WanderersTroupe {}

impl WanderersTroupe {
    pub fn record() -> Artifact {
        Artifact {
            name: "Wanderer's Troupe",
            version: 1.0,
            preference: &[Preference::Ranged],
            state: State::new().ca_dmg(35.0).em(80.0)
                    .atk(SCORE.atk(40.0))
                    .cr(SCORE.cr(60.0))
        }
    }
}

#[derive(Debug)]
pub struct RetracingBolide;

impl SpecialAbility for RetracingBolide {}

impl RetracingBolide {
    pub fn record() -> Artifact {
        Artifact {
            name: "Retracing Bolide",
            version: 1.0,
            preference: &[Preference::Attacker],
            state: State::new().na_dmg(40.0).ca_dmg(40.0)
                    .atk(SCORE.atk(40.0))
                    .cr(SCORE.cr(60.0))
        }
    }
}

#[derive(Debug)]
pub struct RetracingBolideDef;

impl SpecialAbility for RetracingBolideDef {}

impl RetracingBolideDef {
    pub fn record() -> Artifact {
        Artifact {
            name: "Retracing Bolide (DEF)",
            version: 1.0,
            preference: &[Preference::Attacker],
            state: State::new().na_dmg(40.0).ca_dmg(40.0)
                    .def(SCORE.def(40.0))
                    .cr(SCORE.cr(60.0))
        }
    }
}

#[derive(Debug)]
pub struct Thundersoother {
    idx: FieldCharacterIndex,
}

impl Thundersoother {
    pub fn record() -> Artifact {
        Artifact {
            name: "Thundersoother",
            version: 1.0,
            preference: &[Preference::Electro],
            state: State::new()
                    .atk(SCORE.atk(40.0))
                    .cr(SCORE.cr(60.0))
        }
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx
        }
    }
}

impl SpecialAbility for Thundersoother {
    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if enemy.aura.aura == Vision::Electro {
            let state = &mut modifiable_data[self.idx.0].state;
            state.all_dmg += 35.0;
        }
    }
}

#[derive(Debug)]
pub struct Lavawalker {
    idx: FieldCharacterIndex,
}

impl Lavawalker {
    pub fn record() -> Artifact {
        Artifact {
            name: "Lavawalker",
            version: 1.0,
            preference: &[Preference::Pyro],
            state: State::new()
                    .atk(SCORE.atk(40.0))
                    .cr(SCORE.cr(60.0))
        }
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx
        }
    }
}

impl SpecialAbility for Lavawalker {
    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if enemy.aura.aura == Vision::Pyro {
            let state = &mut modifiable_data[self.idx.0].state;
            state.all_dmg += 35.0;
        }
    }
}

#[derive(Debug)]
pub struct LavawalkerHp {
    idx: FieldCharacterIndex
}

impl LavawalkerHp {
    pub fn record() -> Artifact {
        Artifact {
            name: "Lavawalker (HP)",
            version: 1.0,
            preference: &[Preference::Pyro],
            state: State::new()
                    .hp(SCORE.hp(40.0))
                    .cr(SCORE.cr(60.0))
        }
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx
        }
    }
}

impl SpecialAbility for LavawalkerHp {
    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if enemy.aura.aura == Vision::Pyro {
            let state = &mut modifiable_data[self.idx.0].state;
            state.all_dmg += 35.0;
        }
    }
}

#[derive(Debug)]
pub struct Gfelm;

impl SpecialAbility for Gfelm {}

impl Gfelm {
    pub fn record() -> Artifact {
        Artifact {
            name: "GFElem ATK 18% DMG 15%",
            version: 1.0,
            preference: &[],
            state: State::new().elemental_dmg(15.0)
                    .atk(18.0 + SCORE.atk(40.0))
                    .cr(SCORE.cr(60.0))
        }
    }
}

#[derive(Debug)]
pub struct GfelmEr;

impl SpecialAbility for GfelmEr {}

impl GfelmEr {
    pub fn record() -> Artifact {
        Artifact {
            name: "GFElem ATK88% CR46% ER77% DMG 15%",
            version: 1.0,
            preference: &[],
            state: State::new().elemental_dmg(15.0)
                    .atk(18.0 + SCORE.atk(33.3333))
                    .cr(SCORE.cr(33.3333))
                    .er(SCORE.er(33.3333))
        }
    }
}

#[derive(Debug)]
pub struct GfelmEr2;

impl SpecialAbility for GfelmEr2 {}

impl GfelmEr2 {
    pub fn record() -> Artifact {
        Artifact {
            name: "GFElem ATK70% CR35% ER116% DMG 15%",
            version: 1.0,
            preference: &[],
            state: State::new().elemental_dmg(15.0)
                    .atk(18.0 + SCORE.atk(25.0))
                    .cr(SCORE.cr(25.0))
                    .er(SCORE.er(50.0))
        }
    }
}

#[derive(Debug)]
pub struct GfelmHpCr;

impl SpecialAbility for GfelmHpCr {}

impl GfelmHpCr {
    pub fn record() -> Artifact {
        Artifact {
            name: "GFE HP 105% ATK 105% DMG 15%",
            version: 1.0,
            preference: &[Preference::Hydro],
            state: State::new().elemental_dmg(15.0)
                    .hp(SCORE.hp(50.0))
                    .atk(SCORE.atk(50.0))
        }
    }
}

#[derive(Debug)]
pub struct BlizzardStrayer {
    idx: FieldCharacterIndex
}

impl BlizzardStrayer {
    pub fn record() -> Artifact {
        Artifact {
            name: "Blizzard Strayer",
            version: 1.2,
            preference: &[Preference::Cryo, Preference::Hydro],
            state: State::new().cryo_dmg(15.0)
                    .atk(SCORE.atk(40.0))
                    .cr(SCORE.cr(60.0))
        }
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx
        }
    }
}

impl SpecialAbility for BlizzardStrayer {
    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        let state = &mut modifiable_data[self.idx.0].state;
        match (enemy.isfrozen, &enemy.aura.aura) {
            (true, Vision::Cryo) => state.cr += 40.0,
            (false, Vision::Cryo) => state.cr += 20.0,
            _ => (),
        }
    }
}

#[derive(Debug)]
pub struct HeartOfDepth {
    idx: FieldCharacterIndex,
    timer: DurationTimer
}

impl HeartOfDepth {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx,
            timer: DurationTimer::new(15.0, &[0.0])
        }
    }

    pub fn record() -> Artifact {
        Artifact {
            name: "Heart of Depth",
            version: 1.2,
            preference: &[Preference::Hydro],
            state: State::new().hydro_dmg(15.0)
                    .atk(SCORE.atk(40.0))
                    .cr(SCORE.cr(60.0))
        }
    }
}

impl SpecialAbility for HeartOfDepth {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update(time, event.idx == self.idx && (event.kind == PressSkill || event.kind == HoldSkill));
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if self.timer.n == 1 {
            let state = &mut modifiable_data[self.idx.0].state;
            state.na_dmg += 30.0;
            state.ca_dmg += 30.0;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

#[derive(Debug)]
pub struct GlacierAndSnowfield {
    idx: FieldCharacterIndex,
    timer: DurationTimer
}

impl GlacierAndSnowfield {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx,
            timer: DurationTimer::new(10.0, &[0.0])
        }
    }

    pub fn record() -> Artifact {
        Artifact {
            name: "Glacier and Snowfield",
            version: 99.0,
            preference: &[Preference::Cryo],
            state: State::new().cryo_dmg(15.0).amplifying_bonus(15.0).transformative_bonus(100.0)
                    .atk(SCORE.atk(40.0))
                    .cr(SCORE.cr(60.0))
        }
    }
}

impl SpecialAbility for GlacierAndSnowfield {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update(time, event.idx == self.idx && event.kind == Burst);
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if self.timer.n == 1 {
            let state = &mut modifiable_data[self.idx.0].state;
            state.cryo_dmg += 30.0;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

#[derive(Debug)]
pub struct PaleFlame {
    idx: FieldCharacterIndex,
    timer: DurationTimer
}

impl PaleFlame {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx,
            timer: DurationTimer::new(7.0, &[0.3, 0.3])
        }
    }

    pub fn record() -> Artifact {
        Artifact {
            name: "Pale Flame",
            version: 1.5,
            preference: &[Preference::Physical],
            state: State::new().physical_dmg(25.0)
                    .atk(SCORE.atk(40.0))
                    .cr(SCORE.cr(60.0))
        }
    }
}

impl SpecialAbility for PaleFlame {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        self.timer.update(time, event.idx == self.idx && (event.kind == PressSkill || event.kind == HoldSkill || event.kind == SkillDot));
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        match (self.timer.n) {
            2 => {
                let state = &mut modifiable_data[self.idx.0].state;
                state.atk += 18.0;
                state.physical_dmg += 25.0;
            },
            1 => {
                let state = &mut modifiable_data[self.idx.0].state;
                state.atk += 9.0;
            },
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

#[derive(Debug)]
pub struct TenacityOfTheMillelith {
    timer: DurationTimer
}

impl TenacityOfTheMillelith {
    pub fn new() -> Self {
        Self { timer: DurationTimer::new(3.0, &[0.5]) }
    }

    pub fn record() -> Artifact {
        Artifact {
            name: "Tenacity of the Millelith",
            version: 1.5,
            preference: &[Preference::Supporter],
            state: State::new().hp(20.0)
                    .atk(SCORE.atk(40.0))
                    .cr(SCORE.cr(60.0))
        }
    }
}

impl SpecialAbility for TenacityOfTheMillelith {
    fn update(&mut self, time: f32, _event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        let mut should_update = false;
        unsafe {
            for &a in attack {
                let atk = & *a;
                if atk.idx == data.idx && (atk.kind == PressSkill || atk.kind == HoldSkill || atk.kind == SkillDot) {
                    should_update = true;
                    break;
                }
            }
        }
        self.timer.update(time, should_update);
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if self.timer.n == 1 {
            for data in modifiable_data.iter_mut() {
                if data.state.stacked_buff != TENACITY_OF_THE_MILLELITH {
                    data.state.atk += 20.0;
                    data.state.stacked_buff.turn_on(&TENACITY_OF_THE_MILLELITH);
                }
            }
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

#[derive(Debug)]
pub struct ShimenawasReminiscence {
    idx: FieldCharacterIndex,
    timer: NTimer,
}

// 4 Piece: When casting an Elemental Skill, if the character has 15 or more
// Energy, they lose 15 Energy and Normal/Charged/ Plunging Attack DMG is
// increased by 50% for 10s.
impl ShimenawasReminiscence {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx,
            timer: NTimer::new(&[10.0])
        }
    }

    pub fn record() -> Artifact {
        Artifact {
            name: "Shimenawa's Reminiscence",
            version: 2.0,
            preference: &[Preference::Attacker],
            state: State::new()
                    .atk(18.0 + SCORE.atk(40.0))
                    .cr(SCORE.cr(60.0))
        }
    }
}

impl SpecialAbility for ShimenawasReminiscence {
    fn update(&mut self, time: f32, event: &AttackEvent, data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
        let should_update = event.idx == self.idx && (event.kind == PressSkill || event.kind == HoldSkill) && data.state.energy >= 15.0;
        self.timer.update(time, should_update);
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        match (self.timer.ping, self.timer.n) {
            (true, 1) => {
                let state = &mut modifiable_data[self.idx.0].state;
                state.energy -= 15.0;
                state.na_dmg += 50.0;
                state.ca_dmg += 50.0;
            },
            (_, 1) => {
                let state = &mut modifiable_data[self.idx.0].state;
                state.na_dmg += 50.0;
                state.ca_dmg += 50.0;
            },
            _ => (),
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

#[derive(Debug)]
pub struct GfShimenawa;

impl SpecialAbility for GfShimenawa {}

impl GfShimenawa {
    pub fn record() -> Artifact {
        Artifact {
            name: "GFShimenawa ATK 36%",
            version: 2.0,
            preference: &[],
            state: State::new()
                    .atk(36.0 + SCORE.atk(40.0))
                    .cr(SCORE.cr(60.0))
        }
    }
}

#[derive(Debug)]
pub struct EmblemOfSeveredFate {
    idx: FieldCharacterIndex,
    once: bool,
}

// 4 Piece: Increases Elemental Burst DMG by 25% of Energy Recharge. A maximum
// 75% DMG increase can be obtained in this way.
impl EmblemOfSeveredFate {
    pub fn record() -> Artifact {
        Artifact {
            name: "Emblem of Severed Fate",
            version: 2.0,
            preference: &[],
            state: State::new().er(20.0)
                    .atk(SCORE.atk(40.0))
                    .cr(SCORE.cr(60.0))
        }
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx,
            once: true
        }
    }
}

fn emblem_of_severed_fate(data: &mut CharacterData) -> () {
    // the maximum DMG bonus is obtained if ER is 300%.
    // `State.er` does not contain base 100% of characters.
    let er = 100.0 + data.state.er;
    data.state.burst_dmg += if er > 300.0 {
        75.0
    } else {
        er * 0.25
    };
}

impl SpecialAbility for EmblemOfSeveredFate {
    fn update(&mut self, _time: f32, _event: &AttackEvent, _data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        emblem_of_severed_fate(&mut modifiable_data[self.idx.0]);
    }
}

#[derive(Debug)]
pub struct EmblemOfSeveredFateER {
    idx: FieldCharacterIndex,
    once: bool,
}

impl EmblemOfSeveredFateER {
    pub fn record() -> Artifact {
        Artifact {
            name: "EoSF ATK70% CR47% ER77%",
            version: 2.0,
            preference: &[],
            state: State::new()
                    .atk(SCORE.atk(33.3333))
                    .cr(SCORE.cr(33.3333))
                    .er(20.0 + SCORE.er(33.3333))
        }
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx,
            once: true
        }
    }
}

impl SpecialAbility for EmblemOfSeveredFateER {
    fn update(&mut self, _time: f32, _event: &AttackEvent, _data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        emblem_of_severed_fate(&mut modifiable_data[self.idx.0]);
    }
}

#[derive(Debug)]
pub struct EmblemOfSeveredFateER2 {
    idx: FieldCharacterIndex,
    once: bool,
}

impl EmblemOfSeveredFateER2 {
    pub fn record() -> Artifact {
        Artifact {
            name: "EoSF ATK52% CR35% ER136%",
            version: 2.0,
            // TODO both attacker and supporter?
            preference: &[Preference::Supporter],
            state: State::new()
                    .atk(SCORE.atk(25.0))
                    .cr(SCORE.cr(25.0))
                    .er(20.0 + SCORE.er(50.0))
        }
    }

    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            idx,
            once: true
        }
    }
}

impl SpecialAbility for EmblemOfSeveredFateER2 {
    fn update(&mut self, _time: f32, _event: &AttackEvent, _data: &CharacterData, _attack: &[*const Attack], _particles: &[FieldEnergy], _enemy: &Enemy) -> () {
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        emblem_of_severed_fate(&mut modifiable_data[self.idx.0]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim1::simulate::simulate;
    use crate::sim1::types::{Vision, ElementalGauge, ElementalGaugeDecay};
    use crate::sim1::fc::{FieldAbility};
    use crate::sim1::testutil::{TestEnvironment};

    use Vision::*;

    // #[test]
    // fn name() {
    //     println!("{:?}", Gfelm::record().state);
    //     assert!(false);
    // }

    // fc0 triggers burst, which is invariant to fc1 who equips an artifact
    // that can be triggered by own burst.
    #[test]
    fn invariance_0() {
        let mut enemy = TestEnvironment::enemy();
        let mut members: Vec<CharacterData> = Vec::new();
        let mut abilities: Vec<FieldAbility> = Vec::new();
        let mut atk_queue: Vec<*const Attack> = Vec::new();
        let mut field_energy: Vec<FieldEnergy> = Vec::new();

        let mut aa = NoblesseOblige::new();

        let mut env1 = TestEnvironment::new();
        let (data, ability) = env1.vision(FieldCharacterIndex(0), State::new(), Pyro);
        members.push(data);
        abilities.push(ability);
        let mut env2 = TestEnvironment::new();
        let (data, ability) = env2.artifact(FieldCharacterIndex(1), State::new(), Pyro, &mut aa);
        members.push(data);
        abilities.push(ability);

        let mut total_dmg = 0.0;
        members[0].state.energy = members[0].character.energy_cost;
        for _ in 0..10 {
            for data in members.iter_mut() {
                data.state.clear();
                data.init();
            }
            total_dmg += simulate(0.2, &mut members, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
        }

        // (burst skill na na na) and (skill na na na)
        let expect = (300.0 + 200.0 + 100.0 + 100.0 + 100.0)
                   + (200.0 + 100.0 + 100.0 + 100.0);
        assert_eq!(total_dmg, expect);
    }

    #[test]
    fn invariance_1() {
        let mut enemy = TestEnvironment::enemy();
        let mut members: Vec<CharacterData> = Vec::new();
        let mut abilities: Vec<FieldAbility> = Vec::new();
        let mut atk_queue: Vec<*const Attack> = Vec::new();
        let mut field_energy: Vec<FieldEnergy> = Vec::new();

        let mut aa = NoblesseOblige::new();

        let mut env1 = TestEnvironment::new();
        let (data, ability) = env1.artifact(FieldCharacterIndex(0), State::new(), Pyro, &mut aa);
        members.push(data);
        abilities.push(ability);
        let mut env2 = TestEnvironment::new();
        let (data, ability) = env2.vision(FieldCharacterIndex(1), State::new(), Pyro);
        members.push(data);
        abilities.push(ability);

        let mut total_dmg = 0.0;
        members[0].state.energy = members[0].character.energy_cost;
        for _ in 0..10 {
            for data in members.iter_mut() {
                data.state.clear();
                data.init();
            }
            total_dmg += simulate(0.2, &mut members, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
        }

        // (burst skill na na na) and (skill na na na)
        let expect = 1.2 * (300.0 + 200.0 + 100.0 + 100.0 + 100.0)
                   + 1.2 * (200.0 + 100.0 + 100.0 + 100.0);
        let differnce = (total_dmg - expect).abs();
        assert!(differnce <= 0.001);
    }

    #[test]
    fn noblesse_oblige_unstackable() {
        let mut enemy = TestEnvironment::enemy();
        let mut members: Vec<CharacterData> = Vec::new();
        let mut abilities: Vec<FieldAbility> = Vec::new();
        let mut atk_queue: Vec<*const Attack> = Vec::new();
        let mut field_energy: Vec<FieldEnergy> = Vec::new();

        let mut aa1 = NoblesseOblige::new();
        let mut aa2 = NoblesseOblige::new();

        let mut env1 = TestEnvironment::new();
        let (data, ability) = env1.artifact(FieldCharacterIndex(0), State::new(), Pyro, &mut aa1);
        members.push(data);
        abilities.push(ability);
        let mut env2 = TestEnvironment::new();
        let (data, ability) = env2.artifact(FieldCharacterIndex(1), State::new(), Pyro, &mut aa2);
        members.push(data);
        abilities.push(ability);

        let mut total_dmg = 0.0;
        members[0].state.energy = members[0].character.energy_cost;
        members[1].state.energy = members[1].character.energy_cost;
        for _ in 0..10 {
            for data in members.iter_mut() {
                data.state.clear();
                data.init();
            }
            total_dmg += simulate(0.2, &mut members, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
        }

        // twice (burst skill na na na)
        let expect = 1.2 * (300.0 + 200.0 + 3.0 * 100.0)
                   + 1.2 * (300.0 + 200.0 + 2.0 * 100.0);
        let differnce = (total_dmg - expect).abs();
        assert!(differnce <= 0.001);
    }

    // #[test]
    // fn viridescent_venerer() {
    //     let mut enemy = TestEnvironment::enemy();
    //     let mut members: Vec<CharacterData> = Vec::new();
    //     let mut abilities: Vec<FieldAbility> = Vec::new();
    //     let mut atk_queue: Vec<*const Attack> = Vec::new();
    //     let mut field_energy: Vec<FieldEnergy> = Vec::new();

    //     let mut aa = ViridescentVenerer::new();

    //     let mut env1 = TestEnvironment::new();
    //     let (data, ability) = env1.artifact(State::new().infusion(true), Anemo, &mut aa);
    //     members.push(data);
    //     abilities.push(ability);

    //     let mut total_dmg = 0.0;
    //     enemy.aura = ElementalGauge {
    //         aura: Vision::Pyro,
    //         unit: 1.0,
    //         decay: ElementalGaugeDecay::A,
    //     };
    //     for _ in 0..10 {
    //         total_dmg += simulate(0.2, &mut members, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
    //     }

    //     // let mut env = TestEnvironment::new();
    //     // let mut aa = ViridescentVenerer;
    //     // let mut members = vec![
    //     //     env.artifact(FieldCharacterIndex(0), State::new().infusion(true), Anemo, &mut aa),
    //     // ];
    //     // // members[0].fc.data.ar.state.infusion = true;
    //     // let mut enemy = TestEnvironment::enemy();
    //     // enemy.aura = ElementalGauge {
    //     //     aura: Vision::Pyro,
    //     //     unit: 1.0,
    //     //     decay: ElementalGaugeDecay::A,
    //     // };
    //     // let mut total_dmg = 0.0;
    //     // for _ in 0..10 {
    //     //     total_dmg += simulate(&mut members, &mut enemy, 0.2);
    //     // }
    //     let expect = (
    //         // skill (level multiplier * reaction multiplier * bonus * TODO resistance (* TODO bypass enemy defense))
    //           (725.36 * 1.2) * 1.2 * 2.0 + 200.0 * 1.2
    //         // na
    //         + (725.36 * 1.2) * 1.2 * 2.0 + 100.0 * 1.2
    //         // na (action multiplier * vv 2 set bonus * vv 4 set RES down)
    //         + 100.0 * 1.2
    //         // na
    //         + 100.0 * 1.2
    //         // na
    //         + 100.0 * 1.2
    //         // na
    //         + 100.0 * 1.2
    //     );
    //     let differnce = (total_dmg - expect).abs();
    //     println!("{:?} {:?}", total_dmg, expect);
    //     assert!(differnce <= 0.001);
    // }

    #[test]
    fn paleflame_1() {
        let mut enemy = TestEnvironment::enemy();
        let mut members: Vec<CharacterData> = Vec::new();
        let mut abilities: Vec<FieldAbility> = Vec::new();
        let mut atk_queue: Vec<*const Attack> = Vec::new();
        let mut field_energy: Vec<FieldEnergy> = Vec::new();

        let mut aa = PaleFlame::new(FieldCharacterIndex(0));

        let mut env1 = TestEnvironment::new();
        let (data, ability) = env1.artifact(FieldCharacterIndex(0), State::new().infusion(true), Pyro, &mut aa);
        members.push(data);
        abilities.push(ability);

        let mut total_dmg = 0.0;
        enemy.aura = ElementalGauge {
            aura: Vision::Pyro,
            unit: 1.0,
            decay: ElementalGaugeDecay::A,
        };
        for _ in 0..40 {
            for data in members.iter_mut() {
                data.state.clear();
                data.init();
            }
            total_dmg += simulate(0.2, &mut members, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
        }

        // skill 15 na, skill 5 na
        let expect = (
              1.09 * (200.0 + 16.0 * 100.0)
            + 1.18 * (200.0 + 4.0 * 100.0)
        );
        assert_eq!(total_dmg, expect);
    }

    #[test]
    fn shimenawa_1() {
        let mut enemy = TestEnvironment::enemy();
        let mut members: Vec<CharacterData> = Vec::new();
        let mut abilities: Vec<FieldAbility> = Vec::new();
        let mut atk_queue: Vec<*const Attack> = Vec::new();
        let mut field_energy: Vec<FieldEnergy> = Vec::new();

        let mut aa = ShimenawasReminiscence::new(FieldCharacterIndex(0));

        let mut env1 = TestEnvironment::new();
        let (data, ability) = env1.artifact(FieldCharacterIndex(0), State::new(), Pyro, &mut aa);
        members.push(data);
        abilities.push(ability);

        members[0].state.energy += 10.0;
        let mut total_dmg = 0.0;
        for _ in 0..20 {
            for data in members.iter_mut() {
                data.state.clear();
                data.init();
            }
            total_dmg += simulate(1.0, &mut members, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
        }
        let expect = 4.0 * 200.0       // skill
                   + 9.0 * 100.0 * 1.5 // na
                   + 9.0 * 100.0;      // na
        assert_eq!(total_dmg, expect);
    }
}
