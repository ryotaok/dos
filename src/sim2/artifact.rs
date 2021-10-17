use crate::sim2::state::State;
use crate::sim2::timeline::{ActionState, Timeline};
use crate::sim2::attack::{Attack, WeaponAttack};
use crate::sim2::types::{DamageType, CharacterAction, WeaponType, FieldEnergy, Preference, Vision, GearScore, NOBLESSE_OBLIGE, TENACITY_OF_THE_MILLELITH};
use crate::sim2::element::{ElementalGauge, ElementalReactionType, ElementalReaction};
use crate::sim2::record::{CharacterData, Artifact, Enemy};

use DamageType::*;

const SCORE: GearScore = GearScore { score: 140.0 };

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
    Gfno(Gfno),
    GladiatorsFinale(GladiatorsFinale),
    GladiatorsFinaleDef(GladiatorsFinaleDef),
    WanderersTroupe(WanderersTroupe),
    RetracingBolide(RetracingBolide),
    RetracingBolideDef(RetracingBolideDef),
    Thundersoother(Thundersoother),
    Lavawalker(Lavawalker),
    LavawalkerHp(LavawalkerHp),
    Gfelm(Gfelm),
    GfelmEr(GfelmEr),
    GfelmEr2(GfelmEr2),
    GfelmHpCr(GfelmHpCr),
    BlizzardStrayer(BlizzardStrayer),
    HeartOfDepth(HeartOfDepth),
    GlacierAndSnowfield(GlacierAndSnowfield),
    PaleFlame(PaleFlame),
    TenacityOfTheMillelith(TenacityOfTheMillelith),
    ShimenawasReminiscence(ShimenawasReminiscence),
    GfShimenawa(GfShimenawa),
    EmblemOfSeveredFate(EmblemOfSeveredFate),
    EmblemOfSeveredFateER(EmblemOfSeveredFateER),
    EmblemOfSeveredFateER2(EmblemOfSeveredFateER2),
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
            Gfno(x) => x,
            GladiatorsFinale(x) => x,
            GladiatorsFinaleDef(x) => x,
            WanderersTroupe(x) => x,
            RetracingBolide(x) => x,
            RetracingBolideDef(x) => x,
            Thundersoother(x) => x,
            Lavawalker(x) => x,
            LavawalkerHp(x) => x,
            Gfelm(x) => x,
            GfelmEr(x) => x,
            GfelmEr2(x) => x,
            GfelmHpCr(x) => x,
            BlizzardStrayer(x) => x,
            HeartOfDepth(x) => x,
            GlacierAndSnowfield(x) => x,
            PaleFlame(x) => x,
            TenacityOfTheMillelith(x) => x,
            ShimenawasReminiscence(x) => x,
            GfShimenawa(x) => x,
            EmblemOfSeveredFate(x) => x,
            EmblemOfSeveredFateER(x) => x,
            EmblemOfSeveredFateER2(x) => x,
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
            Gfno(x) => x,
            GladiatorsFinale(x) => x,
            GladiatorsFinaleDef(x) => x,
            WanderersTroupe(x) => x,
            RetracingBolide(x) => x,
            RetracingBolideDef(x) => x,
            Thundersoother(x) => x,
            Lavawalker(x) => x,
            LavawalkerHp(x) => x,
            Gfelm(x) => x,
            GfelmEr(x) => x,
            GfelmEr2(x) => x,
            GfelmHpCr(x) => x,
            BlizzardStrayer(x) => x,
            HeartOfDepth(x) => x,
            GlacierAndSnowfield(x) => x,
            PaleFlame(x) => x,
            TenacityOfTheMillelith(x) => x,
            ShimenawasReminiscence(x) => x,
            GfShimenawa(x) => x,
            EmblemOfSeveredFate(x) => x,
            EmblemOfSeveredFateER(x) => x,
            EmblemOfSeveredFateER2(x) => x,
        }
    }
}

pub fn all() -> Vec<(Artifact, ArtifactUnion)> {
    vec![
    (Bcpf::record(), ArtifactUnion::Bcpf(Bcpf)),
    (ThunderingFury::record(), ArtifactUnion::ThunderingFury(ThunderingFury::new())),
    (ViridescentVenerer::record(), ArtifactUnion::ViridescentVenerer(ViridescentVenerer::new())),
    (VVem::record(), ArtifactUnion::VVem(VVem::new())),
    (ArchaicPetra::record(), ArtifactUnion::ArchaicPetra(ArchaicPetra::new())),
    (CrimsonWitchOfFlames::record(), ArtifactUnion::CrimsonWitchOfFlames(CrimsonWitchOfFlames::new())),
    (CrimsonWitchOfFlamesHp::record(), ArtifactUnion::CrimsonWitchOfFlamesHp(CrimsonWitchOfFlamesHp::new())),
    (NoblesseOblige::record(), ArtifactUnion::NoblesseOblige(NoblesseOblige::new())),
    (Gfno::record(), ArtifactUnion::Gfno(Gfno)),
    (GladiatorsFinale::record(), ArtifactUnion::GladiatorsFinale(GladiatorsFinale::new())),
    (GladiatorsFinaleDef::record(), ArtifactUnion::GladiatorsFinaleDef(GladiatorsFinaleDef::new())),
    (WanderersTroupe::record(), ArtifactUnion::WanderersTroupe(WanderersTroupe)),
    (RetracingBolide::record(), ArtifactUnion::RetracingBolide(RetracingBolide)),
    (RetracingBolideDef::record(), ArtifactUnion::RetracingBolideDef(RetracingBolideDef)),
    (Thundersoother::record(), ArtifactUnion::Thundersoother(Thundersoother::new())),
    (Lavawalker::record(), ArtifactUnion::Lavawalker(Lavawalker::new())),
    (LavawalkerHp::record(), ArtifactUnion::LavawalkerHp(LavawalkerHp::new())),
    (Gfelm::record(), ArtifactUnion::Gfelm(Gfelm)),
    (GfelmEr::record(), ArtifactUnion::GfelmEr(GfelmEr)),
    (GfelmEr2::record(), ArtifactUnion::GfelmEr2(GfelmEr2)),
    (GfelmHpCr::record(), ArtifactUnion::GfelmHpCr(GfelmHpCr)),
    (BlizzardStrayer::record(), ArtifactUnion::BlizzardStrayer(BlizzardStrayer::new())),
    (HeartOfDepth::record(), ArtifactUnion::HeartOfDepth(HeartOfDepth::new())),
    (GlacierAndSnowfield::record(), ArtifactUnion::GlacierAndSnowfield(GlacierAndSnowfield::new())),
    (PaleFlame::record(), ArtifactUnion::PaleFlame(PaleFlame::new())),
    (TenacityOfTheMillelith::record(), ArtifactUnion::TenacityOfTheMillelith(TenacityOfTheMillelith::new())),
    (ShimenawasReminiscence::record(), ArtifactUnion::ShimenawasReminiscence(ShimenawasReminiscence::new())),
    (GfShimenawa::record(), ArtifactUnion::GfShimenawa(GfShimenawa)),
    (EmblemOfSeveredFate::record(), ArtifactUnion::EmblemOfSeveredFate(EmblemOfSeveredFate::new())),
    (EmblemOfSeveredFateER::record(), ArtifactUnion::EmblemOfSeveredFateER(EmblemOfSeveredFateER::new())),
    (EmblemOfSeveredFateER2::record(), ArtifactUnion::EmblemOfSeveredFateER2(EmblemOfSeveredFateER2::new())),
    ]
}

#[derive(Debug)]
pub struct Bcpf;

impl Timeline for Bcpf {}

impl WeaponAttack for Bcpf {}

impl Bcpf {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("BCPF Physical 50%")
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

impl Timeline for ThunderingFury {}

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
                    enemy.debuff.pyro += 40.;
                    self.pyro = attack.time;
                },
                Vision::Hydro => {
                    enemy.debuff.hydro += 40.;
                    self.hydro = attack.time;
                },
                Vision::Electro => {
                    enemy.debuff.electro += 40.;
                    self.electro = attack.time;
                },
                Vision::Cryo => {
                    enemy.debuff.cryo += 40.;
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
        if data.idx == attack.idx && enemy.trigger_er(&attack.element.aura).is_crystallize() {
            match &enemy.aura.aura {
                Vision::Pyro => self.pyro = attack.time,
                Vision::Hydro => self.hydro = attack.time,
                Vision::Electro => self.electro = attack.time,
                Vision::Cryo => self.cryo = attack.time,
                _ => (),
            }
        }
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
            .preference(&[Preference::Pyro])
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
        Artifact::default()
            .name("Noblesse Oblige")
            .version(1.0)
            .preference(&[Preference::Supporter])
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
}


#[derive(Debug)]
pub struct Gfno;

impl Timeline for Gfno {}

impl WeaponAttack for Gfno {}

impl Gfno {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("GFNO ATK 18% Burst 20%")
            .version(1.0)
            .preference(&[Preference::Supporter])
            .burst_dmg(20.0)
            .atk(18.0 + SCORE.atk(40.0)).cr(SCORE.cr(60.0))
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
pub struct GladiatorsFinaleDef(GladiatorsFinale);

impl GladiatorsFinaleDef {
    fn new() -> Self {
        Self(GladiatorsFinale::new())
    }

    pub fn record() -> Artifact {
        Artifact::default()
            .name("Gladiator's Finale (DEF)")
            .version(1.0)
            .preference(&[Preference::Melee])
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
            .preference(&[Preference::Ranged])
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
        Artifact::default()
            .name("Retracing Bolide")
            .version(1.0)
            .preference(&[Preference::Attacker])
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
            .preference(&[Preference::Attacker])
            .na_dmg(40.0).ca_dmg(40.0)
            .def(SCORE.def(40.0)).cr(SCORE.cr(60.0))
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
            .preference(&[Preference::Pyro])
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
            .name("GFElem ATK 18% DMG 15%")
            .version(1.0)
            .preference(&[])
            .elemental_dmg(15.0)
            .atk(18.0 + SCORE.atk(40.0)).cr(SCORE.cr(60.0))
    }
}

#[derive(Debug)]
pub struct GfelmEr;

impl Timeline for GfelmEr {}

impl WeaponAttack for GfelmEr {}

impl GfelmEr {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("GFElem ATK88% CR46% ER77% DMG 15%")
            .version(1.0)
            .preference(&[])
            .elemental_dmg(15.0)
            .atk(18.0 + SCORE.atk(33.3333)).cr(SCORE.cr(33.3333)).er(SCORE.er(33.3333))
    }
}

#[derive(Debug)]
pub struct GfelmEr2;

impl Timeline for GfelmEr2 {}

impl WeaponAttack for GfelmEr2 {}

impl GfelmEr2 {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("GFElem ATK70% CR35% ER116% DMG 15%")
            .version(1.0)
            .preference(&[])
            .elemental_dmg(15.0)
            .atk(18.0 + SCORE.atk(25.0)).cr(SCORE.cr(25.0)).er(SCORE.er(50.0))
    }
}

#[derive(Debug)]
pub struct GfelmHpCr;

impl Timeline for GfelmHpCr {}

impl WeaponAttack for GfelmHpCr {}

impl GfelmHpCr {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("GFE HP 105% ATK 105% DMG 15%")
            .version(1.0)
            .preference(&[Preference::Hydro])
            .elemental_dmg(15.0)
            .hp(SCORE.hp(50.0)).atk(SCORE.atk(50.0))
    }
}

#[derive(Debug)]
pub struct BlizzardStrayer {
}

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
        if oneself && attack.kind == Skill { // TODO CD .3
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
        Artifact::default()
            .name("Tenacity of the Millelith")
            .version(1.5)
            .preference(&[Preference::Supporter])
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
        Artifact::default()
            .name("Shimenawa's Reminiscence")
            .version(2.0)
            .preference(&[Preference::Attacker])
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
}

#[derive(Debug)]
pub struct GfShimenawa;

impl Timeline for GfShimenawa {}

impl WeaponAttack for GfShimenawa {}

impl GfShimenawa {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("GFShimenawa ATK 36%")
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
        Artifact::default()
            .name("Emblem of Severed Fate")
            .version(2.0)
            .preference(&[])
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
pub struct EmblemOfSeveredFateER {
}

impl EmblemOfSeveredFateER {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("EoSF ATK70% CR47% ER77%")
            .version(2.0)
            .preference(&[])
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

#[derive(Debug)]
pub struct EmblemOfSeveredFateER2 {
}

impl EmblemOfSeveredFateER2 {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("EoSF ATK52% CR35% ER136%")
            .version(2.0)
            //(ODO both attacker and supporter)
            .preference(&[Preference::Supporter])
            .er(20.0 + SCORE.er(50.0))
            .atk(SCORE.atk(25.0)).cr(SCORE.cr(25.0))
    }

    pub fn new() -> Self {
        Self {
        }
    }
}

impl Timeline for EmblemOfSeveredFateER2 {}

impl WeaponAttack for EmblemOfSeveredFateER2 {
    fn modify(&mut self, action_state: &ActionState, data: &CharacterData, attack: &mut Attack, state: &mut State, enemy: &mut Enemy) -> () {
        if attack.idx == data.idx && attack.kind == Burst {
            emblem_of_severed_fate(action_state, state);
        }
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
    use crate::sim2::timeline::{ActionColumn, Timeline};
    use crate::sim2::record::{WeaponRecord, Artifact, FieldMember, TimelineMember};

    use Vision::*;

    // #[test]
    // fn name() {
    //     println!("{:?}", Gfelm::record().state);
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

        let dmg = simulate::calculate_damage(&mut history, &mut members, &mut data, &mut enemy);
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

        let dmg = simulate::calculate_damage(&mut history, &mut members, &mut data, &mut enemy);
        let expect: f32 = 1.2 * (8.*100. + 2.*200. + 2.*300.);
        assert_eq!(dmg.floor(), expect.floor());
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
        let dmg = simulate::calculate_damage(&mut history, &mut members, &mut data, &mut enemy);
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
        let dmg = simulate::calculate_damage(&mut history, &mut members, &mut data, &mut enemy);
        let expect = (
            // na
            25.*100.*1.5 + 5.*100. +
            // skill
            2.*200.
        );
        assert_eq!(dmg, expect);
    }
}
