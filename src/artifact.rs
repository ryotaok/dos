use crate::state::{State, GearScore};
use crate::fc::{FieldCharacterIndex, CharacterData, SpecialAbility, FieldAbilityBuilder, Enemy, Debuff};
use crate::action::{Attack, AttackEvent, ICDTimer, NTimer, DurationTimer};
use crate::types::{AttackType, WeaponType, FieldEnergy, ElementalReaction, Preference, Vision, NOBLESSE_OBLIGE, TENACITY_OF_THE_MILLELITH, THUNDERSOOTHER, LAVAWALKER, BLIZZARDSTRAYER1, BLIZZARDSTRAYER2};

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

pub struct AllArtifacts {
    bloodstainedchivalry: (Artifact, BloodstainedChivalry),
    bcpf: (Artifact, Bcpf),
    thunderingfury: (Artifact, ThunderingFury),
    viridescentvenerer: (Artifact, ViridescentVenerer),
    vvem: (Artifact, VVem),
    archaicpetra: (Artifact, ArchaicPetra),
    crimsonwitchofflames: (Artifact, CrimsonWitchOfFlames),
    crimsonwitchofflameshp: (Artifact, CrimsonWitchOfFlamesHp),
    noblesseoblige: (Artifact, NoblesseOblige),
    gfno: (Artifact, Gfno),
    gladiatorsfinale: (Artifact, GladiatorsFinale),
    gladiatorsfinaledef: (Artifact, GladiatorsFinaleDef),
    wandererstroupe: (Artifact, WanderersTroupe),
    retracingbolide: (Artifact, RetracingBolide),
    retracingbolidedef: (Artifact, RetracingBolideDef),
    thundersoother: (Artifact, Thundersoother),
    lavawalker: (Artifact, Lavawalker),
    lavawalkerhp: (Artifact, LavawalkerHp),
    gfelm: (Artifact, Gfelm),
    gfelmer: (Artifact, GfelmEr),
    gfelmer2: (Artifact, GfelmEr2),
    gfelmhp: (Artifact, GfelmHpCr),
    blizzardstrayer: (Artifact, BlizzardStrayer),
    heartofdepth: (Artifact, HeartOfDepth),
    glacierandsnowfield: (Artifact, GlacierAndSnowfield),
    paleflame: (Artifact, PaleFlame),
    tenacityofthemillelith: (Artifact, TenacityOfTheMillelith),
    shimenawasreminiscence: (Artifact, ShimenawasReminiscence),
    gfshimenawa: (Artifact, GfShimenawa),
    emblemofseveredfate: (Artifact, EmblemOfSeveredFate),
    emblemofseveredfateer: (Artifact, EmblemOfSeveredFateER),
    emblemofseveredfateer2: (Artifact, EmblemOfSeveredFateER2),
}

impl AllArtifacts {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            bloodstainedchivalry: (BloodstainedChivalry::record(), BloodstainedChivalry),
            bcpf: (Bcpf::record(), Bcpf),
            thunderingfury: (ThunderingFury::record(), ThunderingFury::new()),
            viridescentvenerer: (ViridescentVenerer::record(), ViridescentVenerer::new()),
            vvem: (VVem::record(), VVem::new()),
            archaicpetra: (ArchaicPetra::record(), ArchaicPetra::new()),
            crimsonwitchofflames: (CrimsonWitchOfFlames::record(), CrimsonWitchOfFlames::new(idx)),
            crimsonwitchofflameshp: (CrimsonWitchOfFlamesHp::record(), CrimsonWitchOfFlamesHp::new(idx)),
            noblesseoblige: (NoblesseOblige::record(), NoblesseOblige::new()),
            gfno: (Gfno::record(), Gfno),
            gladiatorsfinale: (GladiatorsFinale::record(), GladiatorsFinale::new()),
            gladiatorsfinaledef: (GladiatorsFinaleDef::record(), GladiatorsFinaleDef::new()),
            wandererstroupe: (WanderersTroupe::record(), WanderersTroupe),
            retracingbolide: (RetracingBolide::record(), RetracingBolide),
            retracingbolidedef: (RetracingBolideDef::record(), RetracingBolideDef),
            thundersoother: (Thundersoother::record(), Thundersoother::new(idx)),
            lavawalker: (Lavawalker::record(), Lavawalker::new(idx)),
            lavawalkerhp: (LavawalkerHp::record(), LavawalkerHp::new(idx)),
            gfelm: (Gfelm::record(), Gfelm),
            gfelmer: (GfelmEr::record(), GfelmEr),
            gfelmer2: (GfelmEr2::record(), GfelmEr2),
            gfelmhp: (GfelmHpCr::record(), GfelmHpCr),
            blizzardstrayer: (BlizzardStrayer::record(), BlizzardStrayer::new(idx)),
            heartofdepth: (HeartOfDepth::record(), HeartOfDepth::new(idx)),
            glacierandsnowfield: (GlacierAndSnowfield::record(), GlacierAndSnowfield::new(idx)),
            paleflame: (PaleFlame::record(), PaleFlame::new(idx)),
            tenacityofthemillelith: (TenacityOfTheMillelith::record(), TenacityOfTheMillelith::new()),
            shimenawasreminiscence: (ShimenawasReminiscence::record(), ShimenawasReminiscence::new(idx)),
            gfshimenawa: (GfShimenawa::record(), GfShimenawa),
            emblemofseveredfate: (EmblemOfSeveredFate::record(), EmblemOfSeveredFate::new(idx)),
            emblemofseveredfateer: (EmblemOfSeveredFateER::record(), EmblemOfSeveredFateER::new(idx)),
            emblemofseveredfateer2: (EmblemOfSeveredFateER2::record(), EmblemOfSeveredFateER2::new(idx)),
        }
    }

    pub fn find<'a>(&'a mut self, name: &ArtifactName, builder: &mut FieldAbilityBuilder) -> &'a mut (Artifact, dyn SpecialAbility + 'a) {
        use ArtifactName::*;
        match name {
            BloodstainedChivalry => { builder.artifact(&mut self.bloodstainedchivalry.1); &mut self.bloodstainedchivalry },
            Bcpf => { builder.artifact(&mut self.bcpf.1); &mut self.bcpf },
            ThunderingFury => { builder.artifact(&mut self.thunderingfury.1); &mut self.thunderingfury },
            ViridescentVenerer => { builder.artifact(&mut self.viridescentvenerer.1); &mut self.viridescentvenerer },
            VVem => { builder.artifact(&mut self.vvem.1); &mut self.vvem },
            ArchaicPetra => { builder.artifact(&mut self.archaicpetra.1); &mut self.archaicpetra },
            CrimsonWitchOfFlames => { builder.artifact(&mut self.crimsonwitchofflames.1); &mut self.crimsonwitchofflames },
            CrimsonWitchOfFlamesHp => { builder.artifact(&mut self.crimsonwitchofflameshp.1); &mut self.crimsonwitchofflameshp },
            NoblesseOblige => { builder.artifact(&mut self.noblesseoblige.1); &mut self.noblesseoblige },
            Gfno => { builder.artifact(&mut self.gfno.1); &mut self.gfno },
            GladiatorsFinale => { builder.artifact(&mut self.gladiatorsfinale.1); &mut self.gladiatorsfinale },
            GladiatorsFinaleDef => { builder.artifact(&mut self.gladiatorsfinaledef.1); &mut self.gladiatorsfinaledef },
            WanderersTroupe => { builder.artifact(&mut self.wandererstroupe.1); &mut self.wandererstroupe },
            RetracingBolide => { builder.artifact(&mut self.retracingbolide.1); &mut self.retracingbolide },
            RetracingBolideDef => { builder.artifact(&mut self.retracingbolidedef.1); &mut self.retracingbolidedef },
            Thundersoother => { builder.artifact(&mut self.thundersoother.1); &mut self.thundersoother },
            Lavawalker => { builder.artifact(&mut self.lavawalker.1); &mut self.lavawalker },
            LavawalkerHp => { builder.artifact(&mut self.lavawalkerhp.1); &mut self.lavawalkerhp },
            Gfelm => { builder.artifact(&mut self.gfelm.1); &mut self.gfelm },
            GfelmEr => { builder.artifact(&mut self.gfelmer.1); &mut self.gfelmer },
            GfelmEr2 => { builder.artifact(&mut self.gfelmer2.1); &mut self.gfelmer2 },
            GfelmHpCr => { builder.artifact(&mut self.gfelmhp.1); &mut self.gfelmhp },
            BlizzardStrayer => { builder.artifact(&mut self.blizzardstrayer.1); &mut self.blizzardstrayer },
            HeartOfDepth => { builder.artifact(&mut self.heartofdepth.1); &mut self.heartofdepth },
            GlacierAndSnowfield => { builder.artifact(&mut self.glacierandsnowfield.1); &mut self.glacierandsnowfield },
            PaleFlame => { builder.artifact(&mut self.paleflame.1); &mut self.paleflame },
            TenacityOfTheMillelith => { builder.artifact(&mut self.tenacityofthemillelith.1); &mut self.tenacityofthemillelith },
            ShimenawasReminiscence => { builder.artifact(&mut self.shimenawasreminiscence.1); &mut self.shimenawasreminiscence },
            GfShimenawa => { builder.artifact(&mut self.gfshimenawa.1); &mut self.gfshimenawa },
            EmblemOfSeveredFate => { builder.artifact(&mut self.emblemofseveredfate.1); &mut self.emblemofseveredfate },
            EmblemOfSeveredFateER => { builder.artifact(&mut self.emblemofseveredfateer.1); &mut self.emblemofseveredfateer },
            EmblemOfSeveredFateER2 => { builder.artifact(&mut self.emblemofseveredfateer2.1); &mut self.emblemofseveredfateer2 },
        }
    }
}

#[derive(Debug)]
pub enum ArtifactName {
    BloodstainedChivalry,
    Bcpf,
    ThunderingFury,
    ViridescentVenerer,
    VVem,
    ArchaicPetra,
    CrimsonWitchOfFlames,
    CrimsonWitchOfFlamesHp,
    NoblesseOblige,
    Gfno,
    GladiatorsFinale,
    GladiatorsFinaleDef,
    WanderersTroupe,
    RetracingBolide,
    RetracingBolideDef,
    Thundersoother,
    Lavawalker,
    LavawalkerHp,
    Gfelm,
    GfelmEr,
    GfelmEr2,
    GfelmHpCr,
    BlizzardStrayer,
    HeartOfDepth,
    GlacierAndSnowfield,
    PaleFlame,
    TenacityOfTheMillelith,
    ShimenawasReminiscence,
    GfShimenawa,
    EmblemOfSeveredFate,
    EmblemOfSeveredFateER,
    EmblemOfSeveredFateER2,
}

impl ArtifactName {
    pub fn vec() -> Vec<ArtifactName> {
        use ArtifactName::*;
        vec![
            BloodstainedChivalry,
            Bcpf,
            ThunderingFury,
            ViridescentVenerer,
            VVem,
            ArchaicPetra,
            CrimsonWitchOfFlames,
            CrimsonWitchOfFlamesHp,
            NoblesseOblige,
            Gfno,
            GladiatorsFinale,
            GladiatorsFinaleDef,
            WanderersTroupe,
            RetracingBolide,
            RetracingBolideDef,
            Thundersoother,
            Lavawalker,
            LavawalkerHp,
            Gfelm,
            GfelmEr,
            GfelmEr2,
            GfelmHpCr,
            BlizzardStrayer,
            HeartOfDepth,
            GlacierAndSnowfield,
            PaleFlame,
            TenacityOfTheMillelith,
            ShimenawasReminiscence,
            GfShimenawa,
            EmblemOfSeveredFate,
            EmblemOfSeveredFateER,
            EmblemOfSeveredFateER2,
        ]
    }
}

impl<'a> From<&'a str> for ArtifactName {
    fn from(name: &'a str) -> Self {
        use ArtifactName::*;
        match name {
            "BloodstainedChivalry" => BloodstainedChivalry,
            "Bcpf" => Bcpf,
            "ThunderingFury" => ThunderingFury,
            "ViridescentVenerer" => ViridescentVenerer,
            "VVem" => VVem,
            "ArchaicPetra" => ArchaicPetra,
            "CrimsonWitchOfFlames" => CrimsonWitchOfFlames,
            "CrimsonWitchOfFlamesHp" => CrimsonWitchOfFlamesHp,
            "NoblesseOblige" => NoblesseOblige,
            "Gfno" => Gfno,
            "GladiatorsFinale" => GladiatorsFinale,
            "GladiatorsFinaleDef" => GladiatorsFinaleDef,
            "WanderersTroupe" => WanderersTroupe,
            "RetracingBolide" => RetracingBolide,
            "RetracingBolideDef" => RetracingBolideDef,
            "Thundersoother" => Thundersoother,
            "Lavawalker" => Lavawalker,
            "LavawalkerHp" => LavawalkerHp,
            "Gfelm" => Gfelm,
            "GfelmEr" => GfelmEr,
            "GfelmEr2" => GfelmEr2,
            "GfelmHpCr" => GfelmHpCr,
            "BlizzardStrayer" => BlizzardStrayer,
            "HeartOfDepth" => HeartOfDepth,
            "GlacierAndSnowfield" => GlacierAndSnowfield,
            "PaleFlame" => PaleFlame,
            "TenacityOfTheMillelith" => TenacityOfTheMillelith,
            "ShimenawasReminiscence" => ShimenawasReminiscence,
            "GfShimenawa" => GfShimenawa,
            "EmblemOfSeveredFate" => EmblemOfSeveredFate,
            "EmblemOfSeveredFateER" => EmblemOfSeveredFateER,
            "EmblemOfSeveredFateER2" => EmblemOfSeveredFateER2,
            _ => unimplemented!(),
        }
    }
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
pub struct ViridescentVenerer(pub bool);

impl ViridescentVenerer {
    pub fn new() -> Self {
        Self(false)
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

    fn update(&mut self, _time: f32, _event: &AttackEvent, data: &CharacterData, attack: &[*const Attack], _particles: &[FieldEnergy], enemy: &Enemy) -> () {
        self.0 = false;
        unsafe {
            for &a in attack {
                let atk = & *a;
                if atk.idx == data.idx && ElementalReaction::new(enemy.aura.aura, atk.element.aura).is_swirl() {
                    self.0 = true;
                    break;
                }
            }
        }
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if self.0 {
            enemy.element_res_debuff.push(Debuff::viridescent_venerer());
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
        match (self.timer.ping, self.timer.n) {
            (true, 1) => for data in modifiable_data.iter_mut() {
                data.state.pyro_dmg += 35.0;
                data.state.hydro_dmg += 35.0;
                data.state.electro_dmg += 35.0;
                data.state.cryo_dmg += 35.0;
            },
            (true, 0) => for data in modifiable_data.iter_mut() {
                data.state.pyro_dmg -= 35.0;
                data.state.hydro_dmg -= 35.0;
                data.state.electro_dmg -= 35.0;
                data.state.cryo_dmg -= 35.0;
            },
            _ => (),
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
        match (self.timer.ping, self.timer.n > 0) {
            (true, true) => modifiable_data[self.idx.0].state.pyro_dmg += 7.5,
            (true, false) => modifiable_data[self.idx.0].state.pyro_dmg -= 7.5 * self.timer.previous_n as f32,
            _ => (),
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
        match (self.timer.ping, self.timer.n) {
            (true, 1) => for data in modifiable_data.iter_mut() {
                if data.state.stacked_buff != NOBLESSE_OBLIGE {
                    data.state.atk += 20.0;
                    data.state.stacked_buff.turn_on(&NOBLESSE_OBLIGE);
                }
            },
            (true, 0) => for data in modifiable_data.iter_mut() {
                data.state.atk -= 20.0;
                data.state.stacked_buff.turn_off(&NOBLESSE_OBLIGE);
            },
            _ => (),
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
        let state = &mut modifiable_data[self.idx.0].state;
        match (&enemy.aura.aura, state.stacked_buff != THUNDERSOOTHER) {
            (Vision::Electro, true) => {
                state.all_dmg += 35.0;
                state.stacked_buff.turn_on(&THUNDERSOOTHER);
            },
            (Vision::Electro, false) => (),
            (_, false) => {
                state.all_dmg -= 35.0;
                state.stacked_buff.turn_off(&THUNDERSOOTHER);
            },
            _ => (),
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
        let state = &mut modifiable_data[self.idx.0].state;
        match (&enemy.aura.aura, state.stacked_buff != LAVAWALKER) {
            (Vision::Pyro, true) => {
                state.all_dmg += 35.0;
                state.stacked_buff.turn_on(&LAVAWALKER);
            },
            (Vision::Pyro, false) => (),
            (_, false) => {
                state.all_dmg -= 35.0;
                state.stacked_buff.turn_off(&LAVAWALKER);
            },
            _ => (),
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
        let state = &mut modifiable_data[self.idx.0].state;
        match (&enemy.aura.aura, state.stacked_buff != LAVAWALKER) {
            (Vision::Pyro, true) => {
                state.all_dmg += 35.0;
                state.stacked_buff.turn_on(&LAVAWALKER);
            },
            (Vision::Pyro, false) => (),
            (_, false) => {
                state.all_dmg -= 35.0;
                state.stacked_buff.turn_off(&LAVAWALKER);
            },
            _ => (),
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
        match (enemy.isfrozen, &enemy.aura.aura, state.stacked_buff != BLIZZARDSTRAYER1, state.stacked_buff != BLIZZARDSTRAYER2) {
            // instantly frozen
            (true, Vision::Cryo, true, true) => {
                state.cr += 40.0;
                state.stacked_buff.turn_on(&BLIZZARDSTRAYER1).turn_on(&BLIZZARDSTRAYER2);
            },
            // apply cryo, then hydro
            (true, Vision::Cryo, false, true) => {
                state.cr += 20.0;
                state.stacked_buff.turn_on(&BLIZZARDSTRAYER2);
            },
            // apply cryo
            (false, Vision::Cryo, true, false) => {
                state.cr += 20.0;
                state.stacked_buff.turn_on(&BLIZZARDSTRAYER1);
            },
            // frozen state ended
            (false, Vision::Cryo, false, false) => {
                state.cr -= 20.0;
                state.stacked_buff.turn_off(&BLIZZARDSTRAYER2);
            },
            // frozen state ended
            (false, _, false, false) => {
                state.cr -= 40.0;
                state.stacked_buff.turn_off(&BLIZZARDSTRAYER1).turn_off(&BLIZZARDSTRAYER2);
            },
            // cryo state ended
            (false, _, false, true) => {
                state.cr -= 20.0;
                state.stacked_buff.turn_off(&BLIZZARDSTRAYER1);
            },
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
        let state = &mut modifiable_data[self.idx.0].state;
        match (self.timer.ping, self.timer.n) {
            (true, 1) => {
                state.na_dmg += 30.0;
                state.ca_dmg += 30.0;
            },
            (true, 0) => {
                state.na_dmg -= 30.0;
                state.ca_dmg -= 30.0;
            },
            _ => (),
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
        let state = &mut modifiable_data[self.idx.0].state;
        match (self.timer.ping, self.timer.n) {
            (true, 1) => {
                state.cryo_dmg += 30.0;
            },
            (true, 0) => {
                state.cryo_dmg -= 30.0;
            },
            _ => (),
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
        let state = &mut modifiable_data[self.idx.0].state;
        match (self.timer.ping, self.timer.n) {
            (true, 2) => {
                state.atk += 9.0;
                state.physical_dmg += 25.0;
            },
            (true, 1) => {
                state.atk += 9.0;
            },
            (true, 0) => {
                state.atk -= 9.0 * self.timer.previous_n as f32;
                state.physical_dmg -= 25.0 * (self.timer.previous_n - 1) as f32;
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
        match (self.timer.ping, self.timer.n) {
            (true, 1) => for data in modifiable_data.iter_mut() {
                if data.state.stacked_buff != TENACITY_OF_THE_MILLELITH {
                    data.state.atk += 20.0;
                    data.state.stacked_buff.turn_on(&TENACITY_OF_THE_MILLELITH);
                }
            },
            (true, 0) => for data in modifiable_data.iter_mut() {
                data.state.atk -= 20.0;
                data.state.stacked_buff.turn_off(&TENACITY_OF_THE_MILLELITH);
            },
            _ => (),
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
        let state = &mut modifiable_data[self.idx.0].state;
        match (self.timer.ping, self.timer.n) {
            (true, 1) => {
                state.energy -= 15.0;
                state.na_dmg += 50.0;
                state.ca_dmg += 50.0;
            },
            (true, 0) => {
                state.na_dmg -= 50.0;
                state.ca_dmg -= 50.0;
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
        if self.once {
            self.once = false;
        }
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if self.once {
            emblem_of_severed_fate(&mut modifiable_data[self.idx.0]);
        }
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
        if self.once {
            self.once = false;
        }
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if self.once {
            emblem_of_severed_fate(&mut modifiable_data[self.idx.0]);
        }
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
        if self.once {
            self.once = false;
        }
    }

    fn modify(&self, modifiable_data: &mut [CharacterData], enemy: &mut Enemy) -> () {
        if self.once {
            emblem_of_severed_fate(&mut modifiable_data[self.idx.0]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulate::simulate;
    use crate::types::{Vision, ElementalGauge, ElementalGaugeDecay};
    use crate::fc::{FieldAbility};
    use crate::testutil::{TestEnvironment};

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
            total_dmg += simulate(0.2, &mut members, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
        }

        // (burst skill na na na) and (skill na na na)
        let expect = (300.0 + 200.0 + 100.0 + 100.0 + 100.0)
                   + (200.0 + 100.0 + 100.0 + 100.0);
        assert_eq!(total_dmg, 0.5 * expect);
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
            total_dmg += simulate(0.2, &mut members, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
        }

        // (burst skill na na na) and (skill na na na)
        let expect = 1.2 * (300.0 + 200.0 + 100.0 + 100.0 + 100.0)
                   + 1.2 * (200.0 + 100.0 + 100.0 + 100.0);
        let differnce = (total_dmg - 0.5 * expect).abs();
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
            total_dmg += simulate(0.2, &mut members, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
        }

        // twice (burst skill na na na)
        let expect = 1.2 * (300.0 + 200.0 + 3.0 * 100.0)
                   + 1.2 * (300.0 + 200.0 + 2.0 * 100.0);
        let differnce = (total_dmg - 0.5 * expect).abs();
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
    //     let expect = 0.5 * (
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
            total_dmg += simulate(0.2, &mut members, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
        }

        // skill 15 na, skill 5 na
        let expect = 0.5 * (
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
            total_dmg += simulate(1.0, &mut members, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
        }
        let expect = 4.0 * 200.0       // skill
                   + 9.0 * 100.0 * 1.5 // na
                   + 9.0 * 100.0;      // na
        assert_eq!(total_dmg, 0.5 * expect);
    }
}
