use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, WeaponAttack};
use crate::sim2::types::{DamageType, CharacterAction, WeaponType, FieldEnergy, Preference, Vision, GearScore, SCORE, NOBLESSE_OBLIGE, TENACITY_OF_THE_MILLELITH};
use crate::sim2::element::{ElementalGauge, ElementalReactionType, ElementalReaction, PHYSICAL_GAUGE};
use crate::sim2::record::{CharacterData, Artifact, Enemy};

use DamageType::*;

#[derive(Debug)]
pub enum ArtifactUnion {
    Bcpf(Bcpf),
    ThunderingFury(ThunderingFury),
    ViridescentVenerer(ViridescentVenerer),
    VVem(VVem),
    ArchaicPetra(ArchaicPetra),
    CrimsonWitchOfFlames(CrimsonWitchOfFlames),
    CrimsonWitchOfFlamesHp(CrimsonWitchOfFlamesHp),
    NoblesseOblige(NoblesseOblige),
    GladiatorsFinale(GladiatorsFinale),
    GladiatorsFinaleDef(GladiatorsFinaleDef),
    WanderersTroupe(WanderersTroupe),
    RetracingBolide(RetracingBolide),
    RetracingBolideDef(RetracingBolideDef),
    RetracingBolideHP(RetracingBolideHP),
    Thundersoother(Thundersoother),
    Lavawalker(Lavawalker),
    LavawalkerHp(LavawalkerHp),
    Gfelm(Gfelm),
    Gfelm1(Gfelm1),
    Gfelm2(Gfelm2),
    GfelmHpCr(GfelmHpCr),
    BlizzardStrayer(BlizzardStrayer),
    HeartOfDepth(HeartOfDepth),
    GlacierAndSnowfield(GlacierAndSnowfield),
    PaleFlame(PaleFlame),
    TenacityOfTheMillelith(TenacityOfTheMillelith),
    TenacityOfTheMillelithHP(TenacityOfTheMillelithHP),
    TenacityOfTheMillelithDEF(TenacityOfTheMillelithDEF),
    ShimenawasReminiscence(ShimenawasReminiscence),
    GfShimenawa(GfShimenawa),
    EmblemOfSeveredFate(EmblemOfSeveredFate),
    EmblemOfSeveredFateER(EmblemOfSeveredFateER),
    HuskOfOpulentDreams(HuskOfOpulentDreams),
    HuskOfOpulentDreamsAlbedo(HuskOfOpulentDreamsAlbedo),
    OceanHuedClam(OceanHuedClam),
    OceanHuedClamKokomi(OceanHuedClamKokomi),
}

impl ArtifactUnion {
    pub fn timeline(&mut self) -> &mut dyn Timeline {
        use ArtifactUnion::*;
        match self {
            Bcpf(x) => x,
            ThunderingFury(x) => x,
            ViridescentVenerer(x) => x,
            VVem(x) => x,
            ArchaicPetra(x) => x,
            CrimsonWitchOfFlames(x) => x,
            CrimsonWitchOfFlamesHp(x) => x,
            NoblesseOblige(x) => x,
            GladiatorsFinale(x) => x,
            GladiatorsFinaleDef(x) => x,
            WanderersTroupe(x) => x,
            RetracingBolide(x) => x,
            RetracingBolideDef(x) => x,
            RetracingBolideHP(x) => x,
            Thundersoother(x) => x,
            Lavawalker(x) => x,
            LavawalkerHp(x) => x,
            Gfelm(x) => x,
            Gfelm1(x) => x,
            Gfelm2(x) => x,
            GfelmHpCr(x) => x,
            BlizzardStrayer(x) => x,
            HeartOfDepth(x) => x,
            GlacierAndSnowfield(x) => x,
            PaleFlame(x) => x,
            TenacityOfTheMillelith(x) => x,
            TenacityOfTheMillelithHP(x) => x,
            TenacityOfTheMillelithDEF(x) => x,
            ShimenawasReminiscence(x) => x,
            GfShimenawa(x) => x,
            EmblemOfSeveredFate(x) => x,
            EmblemOfSeveredFateER(x) => x,
            HuskOfOpulentDreams(x) => x,
            HuskOfOpulentDreamsAlbedo(x) => x,
            OceanHuedClam(x) => x,
            OceanHuedClamKokomi(x) => x,
        }
    }

    pub fn field(&mut self) -> &mut dyn WeaponAttack {
        use ArtifactUnion::*;
        match self {
            Bcpf(x) => x,
            ThunderingFury(x) => x,
            ViridescentVenerer(x) => x,
            VVem(x) => x,
            ArchaicPetra(x) => x,
            CrimsonWitchOfFlames(x) => x,
            CrimsonWitchOfFlamesHp(x) => x,
            NoblesseOblige(x) => x,
            GladiatorsFinale(x) => x,
            GladiatorsFinaleDef(x) => x,
            WanderersTroupe(x) => x,
            RetracingBolide(x) => x,
            RetracingBolideDef(x) => x,
            RetracingBolideHP(x) => x,
            Thundersoother(x) => x,
            Lavawalker(x) => x,
            LavawalkerHp(x) => x,
            Gfelm(x) => x,
            Gfelm1(x) => x,
            Gfelm2(x) => x,
            GfelmHpCr(x) => x,
            BlizzardStrayer(x) => x,
            HeartOfDepth(x) => x,
            GlacierAndSnowfield(x) => x,
            PaleFlame(x) => x,
            TenacityOfTheMillelith(x) => x,
            TenacityOfTheMillelithHP(x) => x,
            TenacityOfTheMillelithDEF(x) => x,
            ShimenawasReminiscence(x) => x,
            GfShimenawa(x) => x,
            EmblemOfSeveredFate(x) => x,
            EmblemOfSeveredFateER(x) => x,
            HuskOfOpulentDreams(x) => x,
            HuskOfOpulentDreamsAlbedo(x) => x,
            OceanHuedClam(x) => x,
            OceanHuedClamKokomi(x) => x,
        }
    }
}

pub fn all() -> Vec<(Artifact, ArtifactUnion)> {
    vec![
    // (Gfelm::record(), ArtifactUnion::Gfelm(Gfelm)),
    (Bcpf::record(), ArtifactUnion::Bcpf(Bcpf)),
    (ThunderingFury::record(), ArtifactUnion::ThunderingFury(ThunderingFury::new())),
    (ViridescentVenerer::record(), ArtifactUnion::ViridescentVenerer(ViridescentVenerer::new())),
    (VVem::record(), ArtifactUnion::VVem(VVem::new())),
    (ArchaicPetra::record(), ArtifactUnion::ArchaicPetra(ArchaicPetra::new())),
    (CrimsonWitchOfFlames::record(), ArtifactUnion::CrimsonWitchOfFlames(CrimsonWitchOfFlames::new())),
    (CrimsonWitchOfFlamesHp::record(), ArtifactUnion::CrimsonWitchOfFlamesHp(CrimsonWitchOfFlamesHp::new())),
    (NoblesseOblige::record(), ArtifactUnion::NoblesseOblige(NoblesseOblige::new())),
    (GladiatorsFinale::record(), ArtifactUnion::GladiatorsFinale(GladiatorsFinale::new())),
    (GladiatorsFinaleDef::record(), ArtifactUnion::GladiatorsFinaleDef(GladiatorsFinaleDef::new())),
    (WanderersTroupe::record(), ArtifactUnion::WanderersTroupe(WanderersTroupe)),
    (RetracingBolide::record(), ArtifactUnion::RetracingBolide(RetracingBolide)),
    (RetracingBolideDef::record(), ArtifactUnion::RetracingBolideDef(RetracingBolideDef)),
    (RetracingBolideHP::record(), ArtifactUnion::RetracingBolideHP(RetracingBolideHP)),
    (Thundersoother::record(), ArtifactUnion::Thundersoother(Thundersoother::new())),
    (Lavawalker::record(), ArtifactUnion::Lavawalker(Lavawalker::new())),
    (LavawalkerHp::record(), ArtifactUnion::LavawalkerHp(LavawalkerHp::new())),
    (Gfelm::record(), ArtifactUnion::Gfelm(Gfelm)),
    (Gfelm1::record(), ArtifactUnion::Gfelm1(Gfelm1)),
    (Gfelm2::record(), ArtifactUnion::Gfelm2(Gfelm2)),
    (GfelmHpCr::record(), ArtifactUnion::GfelmHpCr(GfelmHpCr)),
    (BlizzardStrayer::record(), ArtifactUnion::BlizzardStrayer(BlizzardStrayer::new())),
    (HeartOfDepth::record(), ArtifactUnion::HeartOfDepth(HeartOfDepth::new())),
    (GlacierAndSnowfield::record(), ArtifactUnion::GlacierAndSnowfield(GlacierAndSnowfield::new())),
    (PaleFlame::record(), ArtifactUnion::PaleFlame(PaleFlame::new())),
    (TenacityOfTheMillelith::record(), ArtifactUnion::TenacityOfTheMillelith(TenacityOfTheMillelith::new())),
    (TenacityOfTheMillelithHP::record(), ArtifactUnion::TenacityOfTheMillelithHP(TenacityOfTheMillelithHP::new())),
    (TenacityOfTheMillelithDEF::record(), ArtifactUnion::TenacityOfTheMillelithDEF(TenacityOfTheMillelithDEF::new())),
    (ShimenawasReminiscence::record(), ArtifactUnion::ShimenawasReminiscence(ShimenawasReminiscence::new())),
    (GfShimenawa::record(), ArtifactUnion::GfShimenawa(GfShimenawa)),
    (EmblemOfSeveredFate::record(), ArtifactUnion::EmblemOfSeveredFate(EmblemOfSeveredFate::new())),
    (EmblemOfSeveredFateER::record(), ArtifactUnion::EmblemOfSeveredFateER(EmblemOfSeveredFateER::new())),
    (HuskOfOpulentDreams::record(), ArtifactUnion::HuskOfOpulentDreams(HuskOfOpulentDreams::new())),
    (HuskOfOpulentDreamsAlbedo::record(), ArtifactUnion::HuskOfOpulentDreamsAlbedo(HuskOfOpulentDreamsAlbedo::new())),
    (OceanHuedClam::record(), ArtifactUnion::OceanHuedClam(OceanHuedClam::new())),
    (OceanHuedClamKokomi::record(), ArtifactUnion::OceanHuedClamKokomi(OceanHuedClamKokomi::new())),
    ]
}

#[derive(Debug)]
pub struct Bcpf;

impl Timeline for Bcpf {}

impl WeaponAttack for Bcpf {}

impl Bcpf {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("BCPF Physical 50")
            .version(1.0)
            .preference(&[Preference::Physical])
            .physical_dmg(50.0)
            .atk(SCORE.atk(40.0)).cr(SCORE.cr(60.0))
    }
}

#[derive(Debug)]
pub struct ThunderingFury {
}

impl ThunderingFury {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn record() -> Artifact {
        Artifact::default()
            .name("Thundering Fury")
            .version(1.0)
            .preference(&[Preference::Electro])
            .electro_dmg(15.0)
            .transformative_bonus(40.0)
            .atk(SCORE.atk(40.0)).cr(SCORE.cr(60.0))
    }
}

impl Timeline for ThunderingFury {
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        if data.character.name.contains("Lisa") ||
           data.character.name.contains("Yae Miko (C2)") ||
           data.character.name.contains("Yae Miko") {
            return;
        }
        match (event) {
            CharacterAction::PressSkill |
            CharacterAction::HoldSkill => state.reduce_skill += 2.,
            _ => (),
        }
    }
}

impl WeaponAttack for ThunderingFury {}

#[derive(Debug)]
pub struct ViridescentVenerer {
    pyro: f32,
    hydro: f32,
    electro: f32,
    cryo: f32,
}

impl ViridescentVenerer {
    pub fn new() -> Self {
        Self {
            pyro: -1.,
            hydro: -1.,
            electro: -1.,
            cryo: -1.,
        }
    }

    pub fn record() -> Artifact {
        Artifact::default()
            .name("Viridescent Venerer")
            .version(1.0)
            .preference(&[Preference::Anemo])
            .anemo_dmg(15.)
            .transformative_bonus(60.)
            .atk(SCORE.atk(40.)).cr(SCORE.cr(60.))
    }
}

impl Timeline for ViridescentVenerer {}

impl WeaponAttack for ViridescentVenerer {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if data.idx == attack.idx && enemy.trigger_er(&attack.element.aura).is_swirl() {
            match &enemy.aura.aura {
                Vision::Pyro => {
                    if self.pyro < 0. {
                        enemy.debuff.pyro += 40.;
                    }
                    self.pyro = attack.time;
                },
                Vision::Hydro => {
                    if self.hydro < 0. {
                        enemy.debuff.hydro += 40.;
                    }
                    self.hydro = attack.time;
                },
                Vision::Electro => {
                    if self.electro < 0. {
                        enemy.debuff.electro += 40.;
                    }
                    self.electro = attack.time;
                },
                Vision::Cryo => {
                    if self.cryo < 0. {
                        enemy.debuff.cryo += 40.;
                    }
                    self.cryo = attack.time;
                },
                _ => (),
            }
        }
        // check expire
        if self.pyro >= 0. && attack.time - self.pyro > 10. {
            enemy.debuff.pyro -= 40.;
            self.pyro = -1.;
        }
        if self.hydro >= 0. && attack.time - self.hydro > 10. {
            enemy.debuff.hydro -= 40.;
            self.hydro = -1.;
        }
        if self.electro >= 0. && attack.time - self.electro > 10. {
            enemy.debuff.electro -= 40.;
            self.electro = -1.;
        }
        if self.cryo >= 0. && attack.time - self.cryo > 10. {
            enemy.debuff.cryo -= 40.;
            self.cryo = -1.;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.pyro = -1.;
        self.hydro = -1.;
        self.electro = -1.;
        self.cryo = -1.;
    }
}

#[derive(Debug)]
pub struct VVem(ViridescentVenerer);

impl VVem {
    pub fn new() -> Self {
        Self(ViridescentVenerer::new())
    }

    pub fn record() -> Artifact {
        Artifact::default()
            .name("Viridescent Venerer (EM)")
            .version(1.0)
            .preference(&[Preference::Anemo])
            .anemo_dmg(15.0)
            .transformative_bonus(60.0)
            .atk(SCORE.atk(10.0)).cr(SCORE.cr(10.0)).em(SCORE.em(80.0))
    }
}

impl Timeline for VVem {}

impl WeaponAttack for VVem {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        self.0.modify(action_state, data, attack, state, enemy);
    }

    fn reset_modify(&mut self) -> () {
        self.0.reset_modify();
    }
}

#[derive(Debug)]
pub struct ArchaicPetra {
    pyro: f32,
    hydro: f32,
    electro: f32,
    cryo: f32,
}

impl ArchaicPetra {
    pub fn new() -> Self {
        Self {
            pyro: -99.,
            hydro: -99.,
            electro: -99.,
            cryo: -99.,
        }
    }

    pub fn record() -> Artifact {
        Artifact::default()
            .name("Archaic Petra")
            .version(1.0)
            .preference(&[Preference::Geo])
            .geo_dmg(15.0)
            .atk(SCORE.atk(40.0)).cr(SCORE.cr(60.0))
    }
}

impl Timeline for ArchaicPetra {}

impl WeaponAttack for ArchaicPetra {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if data.idx == attack.idx && enemy.trigger_er(&attack.element.aura).is_crystallize() {
            match &enemy.aura.aura {
                Vision::Pyro => self.pyro = attack.time,
                Vision::Hydro => self.hydro = attack.time,
                Vision::Electro => self.electro = attack.time,
                Vision::Cryo => self.cryo = attack.time,
                _ => (),
            }
        }
        if attack.time - self.pyro <= 10. {
            state.pyro_dmg += 35.;
        }
        if attack.time - self.hydro <= 10. {
            state.hydro_dmg += 35.;
        }
        if attack.time - self.electro <= 10. {
            state.electro_dmg += 35.;
        }
        if attack.time - self.cryo <= 10. {
            state.cryo_dmg += 35.;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.pyro = -99.;
        self.hydro = -99.;
        self.electro = -99.;
        self.cryo = -99.;
    }
}

#[derive(Debug)]
pub struct CrimsonWitchOfFlames {
    time: f32,
    stack: f32,
}

impl CrimsonWitchOfFlames {
    pub fn new() -> Self {
        Self {
            time: -1.,
            stack: 0.,
        }
    }

    pub fn record() -> Artifact {
        Artifact::default()
            .name("Crimson Witch of Flames")
            .version(1.0)
            .preference(&[Preference::Pyro])
            .pyro_dmg(15.0)
            .amplifying_bonus(15.0).transformative_bonus(40.0)
            .atk(SCORE.atk(40.0)).cr(SCORE.cr(60.0))
    }
}

impl Timeline for CrimsonWitchOfFlames {}

impl WeaponAttack for CrimsonWitchOfFlames {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_skill() {
            // gain a stack on skill
            self.time = attack.time;
            self.stack += 1.;
            if self.stack > 3. {
                self.stack = 3.;
            }
        } else if self.time > 0. && attack.time - self.time > 10. {
            // expire
            self.time = -1.;
            self.stack = 0.;
        }
        if attack.idx == data.idx {
            state.pyro_dmg += 7.5 * self.stack;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.time = -1.;
        self.stack = 0.;
    }
}

#[derive(Debug)]
pub struct CrimsonWitchOfFlamesHp(CrimsonWitchOfFlames);

impl CrimsonWitchOfFlamesHp {
    pub fn new() -> Self {
        Self(CrimsonWitchOfFlames::new())
    }

    pub fn record() -> Artifact {
        Artifact::default()
            .name("Crimson Witch of Flames (HP)")
            .version(1.0)
            .preference(&[Preference::HuTao])
            .pyro_dmg(15.0)
            .amplifying_bonus(15.0).transformative_bonus(40.0)
            .hp(SCORE.hp(40.0)).cr(SCORE.cr(60.0))
    }
}

impl Timeline for CrimsonWitchOfFlamesHp {}

impl WeaponAttack for CrimsonWitchOfFlamesHp {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        self.0.modify(action_state, data, attack, state, enemy);
    }

    fn reset_modify(&mut self) -> () {
        self.0.reset_modify();
    }
}

#[derive(Debug)]
pub struct NoblesseOblige {
    time: f32,
}

impl NoblesseOblige {
    pub fn new() -> Self {
        Self { time: -99., }
    }

    pub fn record() -> Artifact {
        use Preference::*;
        Artifact::default()
            .name("Noblesse Oblige")
            .version(1.0)
            .preference(&[
                Chongyun, Keqing, Ningguang, Amber, Diluc, Zhongli, Albedo, Aloy,
                Kaeya, Mona, Bennett, Ganyu, Rosaria
                ])
            .burst_dmg(20.0)
            .atk(SCORE.atk(40.0)).cr(SCORE.cr(60.0))
    }
}

impl Timeline for NoblesseOblige {}

impl WeaponAttack for NoblesseOblige {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_burst() {
            self.time = action_state.current_time;
        }
        if attack.time - self.time <= 12. && state.stacked_buff != NOBLESSE_OBLIGE {
            state.atk += 20.0;
            state.stacked_buff.turn_on(&NOBLESSE_OBLIGE);
        }
    }

    fn reset_modify(&mut self) -> () {
        self.time = -99.;
    }
}

#[derive(Debug)]
pub struct GladiatorsFinale {}

impl GladiatorsFinale {
    pub fn new() -> Self {
        Self {}
    }

    pub fn record() -> Artifact {
        Artifact::default()
            .name("Gladiator's Finale")
            .version(1.0)
            .preference(&[Preference::Melee])
            .na_dmg(35.0)
            .atk(18.0 + SCORE.atk(40.0)).cr(SCORE.cr(60.0))
    }
}

impl Timeline for GladiatorsFinale {}

impl WeaponAttack for GladiatorsFinale {}

#[derive(Debug)]
pub struct GladiatorsFinaleDef {}

impl GladiatorsFinaleDef {
    fn new() -> Self {
        Self {}
    }

    pub fn record() -> Artifact {
        Artifact::default()
            .name("Gladiator's Finale (DEF)")
            .version(1.0)
            .preference(&[Preference::Noelle, Preference::Albedo, Preference::AratakiItto])
            .atk(18.0)
            .na_dmg(35.0)
            .def(SCORE.def(40.0)).cr(SCORE.cr(60.0))
    }
}

impl Timeline for GladiatorsFinaleDef {}

impl WeaponAttack for GladiatorsFinaleDef {}

#[derive(Debug)]
pub struct WanderersTroupe;

impl Timeline for WanderersTroupe {}

impl WeaponAttack for WanderersTroupe {}

impl WanderersTroupe {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("Wanderer's Troupe")
            .version(1.0)
            .preference(&[Preference::Ganyu])
            .em(80.0)
            .ca_dmg(35.0)
            .atk(SCORE.atk(40.0)).cr(SCORE.cr(60.0))
    }
}

#[derive(Debug)]
pub struct RetracingBolide;

impl Timeline for RetracingBolide {}

impl WeaponAttack for RetracingBolide {}

impl RetracingBolide {
    pub fn record() -> Artifact {
        use Preference::*;
        Artifact::default()
            .name("Retracing Bolide")
            .version(1.0)
            .preference(&[
                Diluc, Klee, Razor, Keqing, Noelle,
                Tartaglia, Xinyan, Ganyu, Xiao, HuTao, Yanfei, Eula,
                Ayaka, Yoimiya, Aloy
                ])
            .na_dmg(40.0).ca_dmg(40.0)
            .atk(SCORE.atk(40.0)).cr(SCORE.cr(60.0))
    }
}

#[derive(Debug)]
pub struct RetracingBolideDef;

impl Timeline for RetracingBolideDef {}

impl WeaponAttack for RetracingBolideDef {}

impl RetracingBolideDef {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("Retracing Bolide (DEF)")
            .version(1.0)
            .preference(&[Preference::Noelle,Preference::AratakiItto])
            .na_dmg(40.0).ca_dmg(40.0)
            .def(SCORE.def(40.0)).cr(SCORE.cr(60.0))
    }
}

#[derive(Debug)]
pub struct RetracingBolideHP;

impl Timeline for RetracingBolideHP {}

impl WeaponAttack for RetracingBolideHP {}

impl RetracingBolideHP {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("Retracing Bolide (HP)")
            .version(1.0)
            .preference(&[Preference::HuTao])
            .na_dmg(40.0).ca_dmg(40.0)
            .hp(SCORE.hp(40.0)).cr(SCORE.cr(60.0))
    }
}

#[derive(Debug)]
pub struct Thundersoother {}

impl Thundersoother {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("Thundersoother")
            .version(1.0)
            .preference(&[Preference::Electro])
            .atk(SCORE.atk(40.0))
            .cr(SCORE.cr(60.0))
    }

    pub fn new() -> Self {
        Self {
        }
    }
}

impl Timeline for Thundersoother {}

impl WeaponAttack for Thundersoother {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx && enemy.aura.aura == Vision::Electro {
            state.all_dmg += 35.0;
        }
    }
}

#[derive(Debug)]
pub struct Lavawalker {}

impl Lavawalker {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("Lavawalker")
            .version(1.0)
            .preference(&[Preference::Pyro])
            .atk(SCORE.atk(40.0)).cr(SCORE.cr(60.0))
    }

    pub fn new() -> Self {
        Self {
        }
    }
}

impl Timeline for Lavawalker {}

impl WeaponAttack for Lavawalker {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx && enemy.aura.aura == Vision::Pyro {
            state.all_dmg += 35.0;
        }
    }
}

#[derive(Debug)]
pub struct LavawalkerHp {
}

impl LavawalkerHp {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("Lavawalker (HP)")
            .version(1.0)
            .preference(&[Preference::HuTao])
            .hp(SCORE.hp(40.0))
            .cr(SCORE.cr(60.0))
    }

    pub fn new() -> Self {
        Self {
        }
    }
}

impl Timeline for LavawalkerHp {}

impl WeaponAttack for LavawalkerHp {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx && enemy.aura.aura == Vision::Pyro {
            state.all_dmg += 35.0;
        }
    }
}

#[derive(Debug)]
pub struct Gfelm;

impl Timeline for Gfelm {}

impl WeaponAttack for Gfelm {}

impl Gfelm {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("GFElem ATK 18 DMG 15")
            .version(1.0)
            .preference(&[])
            .elemental_dmg(15.0)
            .atk(18.0 + SCORE.atk(40.0)).cr(SCORE.cr(60.0))
    }
}

#[derive(Debug)]
pub struct Gfelm1;

impl Timeline for Gfelm1 {}

impl WeaponAttack for Gfelm1 {}

impl Gfelm1 {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("GFElem ATK88 CR46 Em280 DMG15")
            .version(1.0)
            .preference(&[])
            .elemental_dmg(15.0)
            .atk(18.0 + SCORE.atk(33.3333)).cr(SCORE.cr(33.3333)).em(SCORE.em(33.3333))
    }
}

#[derive(Debug)]
pub struct Gfelm2;

impl Timeline for Gfelm2 {}

impl WeaponAttack for Gfelm2 {}

impl Gfelm2 {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("GFElem ATK70 CR35 Em420 DMG15")
            .version(1.0)
            .preference(&[])
            .elemental_dmg(15.0)
            .atk(18.0 + SCORE.atk(25.0)).cr(SCORE.cr(25.0)).em(SCORE.em(50.0))
    }
}

#[derive(Debug)]
pub struct GfelmHpCr;

impl Timeline for GfelmHpCr {}

impl WeaponAttack for GfelmHpCr {}

impl GfelmHpCr {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("GFE HP 105 ATK 105 DMG15")
            .version(1.0)
            .preference(&[Preference::SangonomiyaKokomi])
            .elemental_dmg(15.0)
            .hp(SCORE.hp(50.0)).atk(SCORE.atk(50.0))
    }
}

#[derive(Debug)]
pub struct BlizzardStrayer {}

impl BlizzardStrayer {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("Blizzard Strayer")
            .version(1.2)
            .preference(&[Preference::Cryo, Preference::Hydro])
            .cryo_dmg(15.0)
            .atk(SCORE.atk(40.0)).cr(SCORE.cr(60.0))
    }

    pub fn new() -> Self {
        Self {
        }
    }
}

impl Timeline for BlizzardStrayer {}

impl WeaponAttack for BlizzardStrayer {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx && enemy.aura.aura == Vision::Cryo {
            state.cr += if enemy.isfrozen {
                40.
            } else {
                20.
            };
        }
    }
}

#[derive(Debug)]
pub struct HeartOfDepth {
    time: f32,
}

impl HeartOfDepth {
    pub fn new() -> Self {
        Self {
            time: -99.
        }
    }

    pub fn record() -> Artifact {
        Artifact::default()
            .name("Heart of Depth")
            .version(1.2)
            .preference(&[Preference::Hydro])
            .hydro_dmg(15.0)
            .atk(SCORE.atk(40.0)).cr(SCORE.cr(60.0))
    }
}

impl Timeline for HeartOfDepth {}

impl WeaponAttack for HeartOfDepth {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_skill() {
            self.time = attack.time;
        }
        if attack.idx == data.idx && attack.time - self.time <= 15. {
            state.na_dmg += 30.0;
            state.ca_dmg += 30.0;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.time = -99.;
    }
}

#[derive(Debug)]
pub struct GlacierAndSnowfield {
    time: f32,
}

impl GlacierAndSnowfield {
    pub fn new() -> Self {
        Self {
            time: -99.
        }
    }

    pub fn record() -> Artifact {
        Artifact::default()
            .name("Glacier and Snowfield")
            .version(99.0)
            .preference(&[Preference::Cryo])
            .cryo_dmg(15.0)
            .amplifying_bonus(15.0).transformative_bonus(100.0)
            .atk(SCORE.atk(40.0)).cr(SCORE.cr(60.0))
    }
}

impl Timeline for GlacierAndSnowfield {}

impl WeaponAttack for GlacierAndSnowfield {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if action_state.did_burst() {
            self.time = attack.time;
        }
        if attack.idx == data.idx && attack.time - self.time <= 10. {
            state.cryo_dmg += 30.0;
        }
    }
}

#[derive(Debug)]
pub struct PaleFlame {
    time: f32,
    stack: f32,
}

impl PaleFlame {
    pub fn new() -> Self {
        Self {
            time: -99.,
            stack: 0.,
        }
    }

    pub fn record() -> Artifact {
        Artifact::default()
            .name("Pale Flame")
            .version(1.5)
            .preference(&[Preference::Physical])
            .physical_dmg(25.0)
            .atk(SCORE.atk(40.0)).cr(SCORE.cr(60.0))
    }
}

impl Timeline for PaleFlame {}

impl WeaponAttack for PaleFlame {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        let oneself = attack.idx == data.idx;
        if oneself && attack.kind == Skill { // TODO cooldown .3
            self.time = attack.time;
            self.stack += 1.;
            if self.stack > 2. {
                self.stack = 2.;
            }
        } else if self.time > 0. && attack.time - self.time > 7. {
            // expire
            self.time = -1.;
            self.stack = 0.;
        }
        if oneself {
            state.atk += 9.0 * self.stack;
            if self.stack == 2. {
                state.physical_dmg += 25.;
            }
        }
    }

    fn reset_modify(&mut self) -> () {
        self.time = -99.;
        self.stack = 0.;
    }
}

#[derive(Debug)]
pub struct TenacityOfTheMillelith {
    time: f32,
}

impl TenacityOfTheMillelith {
    pub fn new() -> Self {
        Self { time: -99. }
    }

    pub fn record() -> Artifact {
        use Preference::*;
        Artifact::default()
            .name("Tenacity of the Millelith")
            .version(1.5)
            .preference(&[
                Fischl, Qiqi,
                Zhongli, Xinyan,
                RaidenShogun,
                ])
            .hp(20.0)
            .atk(SCORE.atk(40.0)).cr(SCORE.cr(60.0))
    }
}

impl Timeline for TenacityOfTheMillelith {}

impl WeaponAttack for TenacityOfTheMillelith {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx && attack.kind == Skill {
            self.time = attack.time;
        }
        if attack.time - self.time <= 3. && state.stacked_buff != TENACITY_OF_THE_MILLELITH {
            state.atk += 20.0;
            state.stacked_buff.turn_on(&TENACITY_OF_THE_MILLELITH);
        }
    }

    fn reset_modify(&mut self) -> () {
        self.time = -99.;
    }
}

#[derive(Debug)]
pub struct TenacityOfTheMillelithHP(TenacityOfTheMillelith);

impl TenacityOfTheMillelithHP {
    pub fn new() -> Self {
        Self(TenacityOfTheMillelith::new())
    }

    pub fn record() -> Artifact {
        Artifact::default()
            .name("Tenacity of the Millelith (HP)")
            .version(1.5)
            .preference(&[Preference::SangonomiyaKokomi ])
            .hp(20. + SCORE.hp(50.))
            .atk(SCORE.atk(50.))
    }
}

impl Timeline for TenacityOfTheMillelithHP {}

impl WeaponAttack for TenacityOfTheMillelithHP {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        self.0.modify(action_state, data, attack, state, enemy);
    }

    fn reset_modify(&mut self) -> () {
        self.0.reset_modify();
    }
}

#[derive(Debug)]
pub struct TenacityOfTheMillelithDEF(TenacityOfTheMillelith);

impl TenacityOfTheMillelithDEF {
    pub fn new() -> Self {
        Self(TenacityOfTheMillelith::new())
    }

    pub fn record() -> Artifact {
        Artifact::default()
            .name("Tenacity of the Millelith (DEF)")
            .version(1.5)
            .preference(&[Preference::Albedo ])
            .hp(20.)
            .def(SCORE.def(40.))
            .cr(SCORE.cr(60.))
    }
}

impl Timeline for TenacityOfTheMillelithDEF {}

impl WeaponAttack for TenacityOfTheMillelithDEF {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        self.0.modify(action_state, data, attack, state, enemy);
    }

    fn reset_modify(&mut self) -> () {
        self.0.reset_modify();
    }
}

#[derive(Debug)]
pub struct ShimenawasReminiscence {
    time: f32,
    did_activate: Vec<f32>,
}

// 4 Piece: When casting an Elemental Skill, if the character has 15 or more
// Energy, they lose 15 Energy and Normal/Charged/ Plunging Attack DMG is
// increased by 50% for 10s.
impl ShimenawasReminiscence {
    pub fn new() -> Self {
        Self {
            time: -99.,
            did_activate: Vec::new(),
        }
    }

    pub fn record() -> Artifact {
        use Preference::*;
        Artifact::default()
            .name("Shimenawa's Reminiscence")
            .version(2.0)
            // copy of "Retracing Bolide"
            .preference(&[
                Diluc, Klee, Razor, Keqing, Noelle,
                Tartaglia, Xinyan, Ganyu, Xiao, HuTao, Yanfei, Eula,
                Ayaka, Yoimiya, Aloy
            ])
            .atk(18.0 + SCORE.atk(40.0)).cr(SCORE.cr(60.0))
    }
}

impl Timeline for ShimenawasReminiscence {
    fn accelerate(&mut self, field_energy: &mut Vec<FieldEnergy>, event: &CharacterAction, state: &mut ActionState, data: &CharacterData) -> () {
        match (state.energy >= 15., event) {
            (true, CharacterAction::PressSkill) |
            (true, CharacterAction::HoldSkill) => {
                state.energy -= 15.;
                self.did_activate.push(state.current_time);
            },
            _ => (),
        }
    }
}

impl WeaponAttack for ShimenawasReminiscence {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if self.did_activate.contains(&action_state.current_time) {
            self.time = action_state.current_time;
        }
        if attack.idx == data.idx && attack.time - self.time <= 10. {
            state.na_dmg += 50.0;
            state.ca_dmg += 50.0;
        }
    }

    fn reset_modify(&mut self) -> () {
        self.time = -99.;
        self.did_activate.clear();
    }
}

#[derive(Debug)]
pub struct GfShimenawa;

impl Timeline for GfShimenawa {}

impl WeaponAttack for GfShimenawa {}

impl GfShimenawa {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("GFShimenawa ATK 36")
            .version(2.0)
            .preference(&[])
            .atk(36.0 + SCORE.atk(40.0)).cr(SCORE.cr(60.0))
    }
}

#[derive(Debug)]
pub struct EmblemOfSeveredFate {
}

// 4 Piece: Increases Elemental Burst DMG by 25% of Energy Recharge. A maximum
// 75% DMG increase can be obtained in this way.
impl EmblemOfSeveredFate {
    pub fn record() -> Artifact {
        use Preference::*;
        Artifact::default()
            .name("Emblem of Severed Fate")
            .version(2.0)
            .preference(&[
                Beidou, Lisa, Xingqiu, Xiangling, Diona, Eula,
                Ayaka, TravelerElectro, RaidenShogun, KujouSara, Thoma
            ])
            .er(20.0)
            .atk(SCORE.atk(40.0)).cr(SCORE.cr(60.0))
    }

    pub fn new() -> Self {
        Self {
        }
    }
}

fn emblem_of_severed_fate(action_state: &ActionState, state: &mut State) -> () {
    // the maximum DMG bonus is obtained if ER is 300%.
    // `State.er` does not contain base 100% of characters.
    let er = 100.0 + action_state.er;
    state.burst_dmg += if er > 300.0 {
        75.0
    } else {
        er * 0.25
    };
}

impl Timeline for EmblemOfSeveredFate {}

impl WeaponAttack for EmblemOfSeveredFate {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx && attack.kind == Burst {
            emblem_of_severed_fate(action_state, state);
        }
    }
}

#[derive(Debug)]
pub struct EmblemOfSeveredFateER {}

impl EmblemOfSeveredFateER {
    pub fn record() -> Artifact {
        use Preference::*;
        Artifact::default()
            .name("EoSF ATK70 CR47 ER77")
            .version(2.0)
            .preference(&[
                Beidou, Lisa, Xingqiu, Xiangling, Diona, Eula,
                Ayaka, TravelerElectro, RaidenShogun, KujouSara, Thoma
            ])
            .er(20.0 + SCORE.er(33.3333))
            .atk(SCORE.atk(33.3333)).cr(SCORE.cr(33.3333))
    }

    pub fn new() -> Self {
        Self {
        }
    }
}

impl Timeline for EmblemOfSeveredFateER {}

impl WeaponAttack for EmblemOfSeveredFateER {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx && attack.kind == Burst {
            emblem_of_severed_fate(action_state, state);
        }
    }
}

// 4 Piece: A character equipped with this Artifact set will obtain the
// Curiosity effect in the following conditions: When on the field, the
// character gains 1 stack after hitting an opponent with a Geo attack,
// triggering a maximum of once every 0.3s. When off the field, the character
// gains 1 stack every 3s. Curiosity can stack up to 4 times, each providing 6%
// DEF and a 6% Geo DMG Bonus. When 6 seconds pass without gaining a Curiosity
// stack, 1 stack is lost.
#[derive(Debug)]
pub struct HuskOfOpulentDreams {
    stack: f32,
    time: f32,
}

impl HuskOfOpulentDreams {
    pub fn new() -> Self {
        Self {
            time: -99.,
            stack: 2.,
        }
    }

    pub fn record() -> Artifact {
        use Preference::*;
        Artifact::default()
            .name("Husk of Opulent Dreams (start 2s)")
            .version(2.3)
            .preference(&[Noelle, Albedo, AratakiItto, Gorou ])
            .def(30. + SCORE.def(40.0)).cr(SCORE.cr(60.0))
    }
}

impl Timeline for HuskOfOpulentDreams {}

impl WeaponAttack for HuskOfOpulentDreams {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if data.idx.is_on_field() {
            if attack.idx == data.idx && attack.element.aura == Vision::Geo && self.time - attack.time >= 0.3 {
                self.stack += 1.;
            }
        } else {
            if self.time - attack.time >= 3. {
                self.stack += 1.;
            }
        }
        if self.stack > 4. {
            self.stack = 4.;
        }
        state.def += 6. * self.stack;
        state.geo_dmg += 6. * self.stack;
    }

    fn reset_modify(&mut self) -> () {
        self.time = -99.;
        self.stack = 2.;
    }
}

#[derive(Debug)]
pub struct HuskOfOpulentDreamsAlbedo(HuskOfOpulentDreams);

impl HuskOfOpulentDreamsAlbedo {
    pub fn new() -> Self {
        Self(HuskOfOpulentDreams::new())
    }

    pub fn record() -> Artifact {
        Artifact::default()
            .name("Husk of Opulent Dreams (DEF goblet)")
            .version(2.3)
            .preference(&[Preference::Albedo])
            .def(30. + 58.3 + SCORE.def(40.0))
            .geo_dmg(-46.6)
            .cr(SCORE.cr(60.0))
    }
}

impl Timeline for HuskOfOpulentDreamsAlbedo {}

impl WeaponAttack for HuskOfOpulentDreamsAlbedo {
    fn attack(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.0.attack(time, event, data, atk_queue, state, enemy);
    }

    fn reset_attack(&mut self) -> () {
        self.0.reset_attack();
    }
}

// 4 Piece: When the character equipping this artifact set heals a character in
// the party, a Sea-Dyed Foam will appear for 3 seconds, accumulating the amount
// of HP recovered from healing (including overflow healing). At the end of the
// duration, the Sea-Dyed Foam will explode, dealing DMG to nearby opponents
// based on 90% of the accumulated healing. (This DMG is calculated similarly to
// Reactions such as Electro-Charged, and Superconduct, but is not affected by
// Elemental Mastery, Character Levels, or Reaction DMG Bonuses). Only one
// Sea-Dyed Foam can be produced every 3.5 seconds. Each Sea-Dyed Foam can
// accumulate up to 30,000 HP (including overflow healing). There can be no more
// than one Sea-Dyed Foam active at any given time. This effect can still be
// triggered even when the character who is using this artifact set is not on
// the field.
#[derive(Debug)]
pub struct OceanHuedClam {
    time: f32,
}

impl OceanHuedClam {
    pub fn new() -> Self {
        Self {
            time: -99.,
        }
    }

    pub fn record() -> Artifact {
        use Preference::*;
        Artifact::default()
            .name("Ocean-Hued Clam (7k heal)")
            .version(2.3)
            .preference(&[Bennett, Barbara, Qiqi, Jean, Noelle, Diona, Sayu, ])
            .atk(SCORE.atk(40.0))
            .cr(SCORE.cr(60.0))
    }
}

impl Timeline for OceanHuedClam {}

// Assuming 2000 heal per second. Then 7000 heal for 3.5 seconds.
impl WeaponAttack for OceanHuedClam {
    fn attack(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        if time - self.time >= 3.5 {
            self.time = time;
            atk_queue.push(Attack {
                kind: DamageType::FlatDMG,
                multiplier: 0.9 * 7000.,
                element: &PHYSICAL_GAUGE,
                aura_application: false,
                time,
                idx: data.idx,
            });
        }
    }

    fn reset_attack(&mut self) -> () {
        self.time = -99.;
    }
}

#[derive(Debug)]
pub struct OceanHuedClamKokomi(OceanHuedClam);

impl OceanHuedClamKokomi {
    pub fn new() -> Self {
        Self(OceanHuedClam::new())
    }

    pub fn record() -> Artifact {
        Artifact::default()
            .name("Ocean-Hued Clam (7k heal)")
            .version(2.3)
            .preference(&[Preference::SangonomiyaKokomi])
            .atk(SCORE.atk(50.0))
            .hp(SCORE.hp(50.0))
    }
}

impl Timeline for OceanHuedClamKokomi {}

impl WeaponAttack for OceanHuedClamKokomi {
    fn attack(&mut self, time: f32, event: &CharacterAction, data: &CharacterData, atk_queue: &mut Vec<Attack>, state: &mut State, enemy: &mut Enemy) -> () {
        self.0.attack(time, event, data, atk_queue, state, enemy);
    }

    fn reset_attack(&mut self) -> () {
        self.0.reset_attack();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::sim2::testutil;
    use crate::sim2::testutil::{Sim2TestCharacter, NoopTimeline};
    use crate::sim2::simulate;
    use crate::sim2::simulate::History;
    use crate::sim2::element::{ElementalGauge, ElementalGaugeDecay};
    use crate::sim2::types::{Vision};
    use crate::sim2::attack::{DamageResultUtil};
    use crate::sim2::timeline::{ActionColumn, Timeline};
    use crate::sim2::record::{WeaponRecord, Artifact, FieldMember, TimelineMember};
    use crate::sim2::training;

    use Vision::*;

    // #[test]
    // fn name() {
    //     println!("{:?}", training::TrainingArtifact0::record());
    //     println!("{:?}", training::TrainingArtifact1::record());
    //     println!("{:?}", training::TrainingArtifact2::record());
    //     println!("{:?}", training::TrainingArtifact3::record());
    //     println!("{:?}", training::TrainingArtifact4::record());
    //     println!("{:?}", training::TrainingArtifact5::record());
    //     println!("{:?}", training::TrainingArtifact6::record());
    //     // println!("{:?}", training::TrainingArtifact7::record());
    //     // println!("{:?}", training::TrainingArtifact8::record());
    //     // println!("{:?}", training::TrainingArtifact9::record());
    //     assert!(false);
    // }

    // fc0 triggers burst, which is invariant to fc1 who equips an artifact
    // that can be triggered by own burst.
    // #[test]
    // fn invariance_0() {
    //     let mut enemy = TestEnvironment::enemy();
    //     let mut members: Vec<CharacterData> = Vec::new();
    //     let mut abilities: Vec<FieldAbility> = Vec::new();
    //     let mut atk_queue: Vec<*const Attack> = Vec::new();
    //     let mut field_energy: Vec<FieldEnergy> = Vec::new();

    //     let mut aa = NoblesseOblige::new();

    //     let mut env1 = TestEnvironment::new();
    //     let (data, ability) = env1.vision(FieldCharacterIndex(0), State::new(), Pyro);
    //     members.push(data);
    //     abilities.push(ability);
    //     let mut env2 = TestEnvironment::new();
    //     let (data, ability) = env2.artifact(FieldCharacterIndex(1), State::new(), Pyro, &mut aa);
    //     members.push(data);
    //     abilities.push(ability);

    //     let mut total_dmg = 0.0;
    //     members[0].state.energy = members[0].character.energy_cost;
    //     for _ in 0..10 {
    //         for data in members.iter_mut() {
    //             data.state.clear();
    //             data.init();
    //         }
    //         total_dmg += simulate(0.2, &mut members, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
    //     }

    //     // (burst skill na na na) and (skill na na na)
    //     let expect = (300.0 + 200.0 + 100.0 + 100.0 + 100.0)
    //                + (200.0 + 100.0 + 100.0 + 100.0);
    //     assert_eq!(total_dmg, expect);
    // }

    #[test]
    fn invariance_1() {
        let mut history = testutil::history_2at02();
        let mut enemy = Enemy::simple();
        let cr = Sim2TestCharacter::record(Pyro);
        let wr = WeaponRecord::default();
        let ar = Artifact::default();
        let mut data = [CharacterData::new(0, &cr, &wr, &ar), CharacterData::new(1, &cr, &wr, &ar)];

        let mut character1 = Sim2TestCharacter::new();
        let mut character2 = Sim2TestCharacter::new();
        let mut weapon1    = WeaponRecord::default();
        let mut weapon2    = WeaponRecord::default();
        let mut artifact1  = NoblesseOblige::new();
        let mut artifact2  = Artifact::default();
        let mut members = [FieldMember {
            character: &mut character1,
            weapon: &mut weapon1,
            artifact: &mut artifact1,
        }, FieldMember {
            character: &mut character2,
            weapon: &mut weapon2,
            artifact: &mut artifact2,
        }];

        let dmg = simulate::calculate_damage(&mut history, &mut members, &mut data, &mut enemy).total_damage();
        let expect: f32 = 1.2 * (8.*100. + 2.*200. + 2.*300.);
        assert_eq!(dmg.floor(), expect.floor());
    }

    #[test]
    fn noblesse_oblige_unstackable() {
        let mut history = testutil::history_2at02();
        let mut enemy = Enemy::simple();
        let cr = Sim2TestCharacter::record(Pyro);
        let wr = WeaponRecord::default();
        let ar = Artifact::default();
        let mut data = [CharacterData::new(0, &cr, &wr, &ar), CharacterData::new(1, &cr, &wr, &ar)];

        let mut character1 = Sim2TestCharacter::new();
        let mut character2 = Sim2TestCharacter::new();
        let mut weapon1 = WeaponRecord::default();
        let mut weapon2 = WeaponRecord::default();
        let mut artifact1 = NoblesseOblige::new();
        let mut artifact2 = NoblesseOblige::new();
        let mut members = [FieldMember {
            character: &mut character1,
            weapon: &mut weapon1,
            artifact: &mut artifact1,
        }, FieldMember {
            character: &mut character2,
            weapon: &mut weapon2,
            artifact: &mut artifact2,
        }];

        let dmg = simulate::calculate_damage(&mut history, &mut members, &mut data, &mut enemy).total_damage();
        let expect: f32 = 1.2 * (8.*100. + 2.*200. + 2.*300.);
        assert_eq!(dmg.floor(), expect.floor());
    }

    #[test]
    fn viridescent_venerer() {
        let mut history = testutil::history_2at02();
        let mut enemy = Enemy::simple();
        let cr1 = Sim2TestCharacter::record(Anemo);
        let cr2 = Sim2TestCharacter::record(Pyro);
        let wr = WeaponRecord::default();
        let ar = Artifact::default();
        let mut data = [CharacterData::new(0, &cr1, &wr, &ar), CharacterData::new(1, &cr2, &wr, &ar)];

        let mut character1 = Sim2TestCharacter::new();
        let mut character2 = Sim2TestCharacter::new();
        let mut weapon1    = WeaponRecord::default();
        let mut weapon2    = WeaponRecord::default();
        let mut artifact1  = ViridescentVenerer::new();
        let mut artifact2  = Artifact::default();
        let mut members = [FieldMember {
            character: &mut character1,
            weapon: &mut weapon1,
            artifact: &mut artifact1,
        }, FieldMember {
            character: &mut character2,
            weapon: &mut weapon2,
            artifact: &mut artifact2,
        }];
        enemy.aura = ElementalGauge {
            aura: Vision::Pyro,
            unit: 1.0,
            decay: ElementalGaugeDecay::A,
        };

        let dmg = simulate::calculate_damage(&mut history, &mut members, &mut data, &mut enemy).total_damage();

        let expect: f32 = (
            // 2 swirls
            // swirl damage: 725.36 * 1.2 = 870.432
            2. * 870.432 * 1.2

            // [Burst, Burst(Pyro)],
            // because the enemy has no resistance, VV gives 20% increased damage.
            + 300. + 300. * 1.2

            // [PressSkill, PressSkill(Pyro)],
            + 200. + 200. * 1.2
            // [Na1(0.), Na1(0.)],
            + 2. * 100.
            // [Na2(0.), Na2(0.)],
            + 2. * 100.
            // [Na3(0.), Na3(0.)],
            + 2. * 100.
            // [Na4(0.), Na4(0.)],
            + 2. * 100.
        );
        assert_eq!(dmg.floor(), expect.floor());
    }

    #[test]
    fn paleflame_1() {
        let mut history = testutil::history_7at02();
        let mut enemy = Enemy::simple();
        // use infusion to ignore the artifact bonus
        let mut character = Sim2TestCharacter::new().infusion(true);
        let mut weapon = WeaponRecord::default();
        let mut artifact = PaleFlame::new();
        let mut members = [FieldMember {
            character: &mut character,
            weapon: &mut weapon,
            artifact: &mut artifact,
        }; 1];
        let cr = Sim2TestCharacter::record(Pyro);
        let wr = WeaponRecord::default();
        let ar = Artifact::default();
        let mut data = [CharacterData::new(0, &cr, &wr, &ar); 1];
        let dmg = simulate::calculate_damage(&mut history, &mut members, &mut data, &mut enemy).total_damage();
        let expect = (
            // na
            15.*100.*1.09 + 2.*100.*1.18 +
            // skill
            200.*1.09 + 200.*1.18 +
            // burst
            1.*300.
        );
        assert_eq!(dmg, expect);
    }

    #[test]
    fn shimenawa_1() {
        let mut target = testutil::history_12at02enrgy15();
        let mut history = History::<1>::new(12., 0.2);
        let cr = Sim2TestCharacter::record(Pyro);
        let wr = WeaponRecord::default();
        let ar = Artifact::default();
        let mut data = [CharacterData::new(0, &cr, &wr, &ar); 1];
        let mut enemy = Enemy::simple();
        let mut character = Sim2TestCharacter::new();
        let mut weapon = WeaponRecord::default();
        let mut artifact = ShimenawasReminiscence::new();
        {
            let mut states = [ActionState::new(); 1];
            let mut members = [TimelineMember {
                character: &mut character,
                weapon: &mut weapon,
                artifact: &mut artifact,
            }; 1];
            states[0].energy += 15.0;
            simulate::decide_action(&mut history, &mut members, &mut states, &mut data);
            assert_eq!(history.action, target.action);
            assert_eq!(states[0].energy, 12.);
        }

        let mut members = [FieldMember {
            character: &mut character,
            weapon: &mut weapon,
            artifact: &mut artifact,
        }; 1];
        let dmg = simulate::calculate_damage(&mut history, &mut members, &mut data, &mut enemy).total_damage();
        let expect = (
            // na
            25.*100.*1.5 + 5.*100. +
            // skill
            2.*200.
        );
        assert_eq!(dmg, expect);
    }
}
