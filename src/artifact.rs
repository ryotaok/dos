use crate::state::State;
use crate::fc::{CharacterData, SpecialAbility, ArtifactAbility, Enemy, Debuff};
use crate::action::{ElementalAttack, TimerGuard, TimerGuardCheck, FullCharacterTimers, EffectTimer, StackTimer, DurationTimer};
use crate::types::{AttackType, WeaponType, UnstackableBuff, Particle, Preference, Vision};

use AttackType::*;

#[derive(Debug)]
pub struct Artifact {
    pub name: &'static str,
    pub version: f32,
    pub preference: Vec<Preference>,
    pub state: State,
}

impl Default for Artifact {
    fn default() -> Self {
        Self {
            name: "",
            version: 1.0,
            preference: Vec::new(),
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
        self.state.atk += 80.0;
        self.state.cr  += 80.0;
    }
}

fn field<T: ArtifactAbility>(aa: T) -> (Artifact, T) {
    let mut a = aa.record();
    a.setup();
    (a, aa)
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
    blizzardstrayer: (Artifact, BlizzardStrayer),
    heartofdepth: (Artifact, HeartOfDepth),
    glacierandsnowfield: (Artifact, GlacierAndSnowfield),
    paleflame: (Artifact, PaleFlame),
    tenacityofthemillelith: (Artifact, TenacityOfTheMillelith),
    shimenawasreminiscence: (Artifact, ShimenawasReminiscence),
    gfshimenawa: (Artifact, GfShimenawa),
    emblemofseveredfate: (Artifact, EmblemOfSeveredFate),
}

impl AllArtifacts {
    pub fn new() -> Self {
        Self {
            bloodstainedchivalry: field(BloodstainedChivalry),
            bcpf: field(Bcpf),
            thunderingfury: field(ThunderingFury),
            viridescentvenerer: field(ViridescentVenerer),
            vvem: field(VVem::new()),
            archaicpetra: field(ArchaicPetra),
            crimsonwitchofflames: field(CrimsonWitchOfFlames),
            crimsonwitchofflameshp: field(CrimsonWitchOfFlamesHp),
            noblesseoblige: field(NoblesseOblige::new()),
            gfno: field(Gfno),
            gladiatorsfinale: field(GladiatorsFinale::new()),
            gladiatorsfinaledef: field(GladiatorsFinaleDef::new()),
            wandererstroupe: field(WanderersTroupe),
            retracingbolide: field(RetracingBolide),
            retracingbolidedef: field(RetracingBolideDef),
            thundersoother: field(Thundersoother),
            lavawalker: field(Lavawalker),
            lavawalkerhp: field(LavawalkerHp),
            gfelm: field(Gfelm),
            blizzardstrayer: field(BlizzardStrayer),
            heartofdepth: field(HeartOfDepth::new()),
            glacierandsnowfield: field(GlacierAndSnowfield::new()),
            paleflame: field(PaleFlame::new()),
            tenacityofthemillelith: field(TenacityOfTheMillelith::new()),
            shimenawasreminiscence: field(ShimenawasReminiscence::new()),
            gfshimenawa: field(GfShimenawa),
            emblemofseveredfate: field(EmblemOfSeveredFate),
        }
    }

    pub fn find<'a>(&'a mut self, name: &ArtifactName) -> &'a mut (Artifact, dyn ArtifactAbility) {
        use ArtifactName::*;
        match name {
            BloodstainedChivalry => &mut self.bloodstainedchivalry,
            Bcpf => &mut self.bcpf,
            ThunderingFury => &mut self.thunderingfury,
            ViridescentVenerer => &mut self.viridescentvenerer,
            VVem => &mut self.vvem,
            ArchaicPetra => &mut self.archaicpetra,
            CrimsonWitchOfFlames => &mut self.crimsonwitchofflames,
            CrimsonWitchOfFlamesHp => &mut self.crimsonwitchofflameshp,
            NoblesseOblige => &mut self.noblesseoblige,
            Gfno => &mut self.gfno,
            GladiatorsFinale => &mut self.gladiatorsfinale,
            GladiatorsFinaleDef => &mut self.gladiatorsfinaledef,
            WanderersTroupe => &mut self.wandererstroupe,
            RetracingBolide => &mut self.retracingbolide,
            RetracingBolideDef => &mut self.retracingbolidedef,
            Thundersoother => &mut self.thundersoother,
            Lavawalker => &mut self.lavawalker,
            LavawalkerHp => &mut self.lavawalkerhp,
            Gfelm => &mut self.gfelm,
            BlizzardStrayer => &mut self.blizzardstrayer,
            HeartOfDepth => &mut self.heartofdepth,
            GlacierAndSnowfield => &mut self.glacierandsnowfield,
            PaleFlame => &mut self.paleflame,
            TenacityOfTheMillelith => &mut self.tenacityofthemillelith,
            ShimenawasReminiscence => &mut self.shimenawasreminiscence,
            GfShimenawa => &mut self.gfshimenawa,
            EmblemOfSeveredFate => &mut self.emblemofseveredfate,
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
    BlizzardStrayer,
    HeartOfDepth,
    GlacierAndSnowfield,
    PaleFlame,
    TenacityOfTheMillelith,
    ShimenawasReminiscence,
    GfShimenawa,
    EmblemOfSeveredFate,
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
            BlizzardStrayer,
            HeartOfDepth,
            GlacierAndSnowfield,
            PaleFlame,
            TenacityOfTheMillelith,
            ShimenawasReminiscence,
            GfShimenawa,
            EmblemOfSeveredFate,
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
            "BlizzardStrayer" => BlizzardStrayer,
            "HeartOfDepth" => HeartOfDepth,
            "GlacierAndSnowfield" => GlacierAndSnowfield,
            "PaleFlame" => PaleFlame,
            "TenacityOfTheMillelith" => TenacityOfTheMillelith,
            "ShimenawasReminiscence" => ShimenawasReminiscence,
            "GfShimenawa" => GfShimenawa,
            "EmblemOfSeveredFate" => EmblemOfSeveredFate,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct BloodstainedChivalry;

impl SpecialAbility for BloodstainedChivalry {}

impl ArtifactAbility for BloodstainedChivalry {
    fn record(&self) -> Artifact {
        Artifact {
            name: "Bloodstained Chivalry",
            version: 1.0,
            preference: vec![Preference::Physical],
            state: State::new().physical_dmg(25.0)
        }
    }
}

#[derive(Debug)]
pub struct Bcpf;

impl SpecialAbility for Bcpf {}

impl ArtifactAbility for Bcpf {
    fn record(&self) -> Artifact {
        Artifact {
            name: "BCPF Physical 50%",
            version: 1.0,
            preference: vec![Preference::Physical],
            state: State::new().physical_dmg(50.0)
        }
    }
}

#[derive(Debug)]
pub struct ThunderingFury;

impl SpecialAbility for ThunderingFury {}

impl ArtifactAbility for ThunderingFury {
    fn record(&self) -> Artifact {
        Artifact {
            name: "Thundering Fury",
            version: 1.0,
            preference: vec![Preference::Electro],
            state: State::new().electro_dmg(15.0).transformative_bonus(40.0)
        }
    }
}

#[derive(Debug)]
pub struct ViridescentVenerer;

impl ArtifactAbility for ViridescentVenerer {
    fn record(&self) -> Artifact {
        Artifact {
            name: "Viridescent Venerer",
            version: 1.0,
            preference: vec![Preference::Anemo],
            state: State::new().anemo_dmg(15.0).transformative_bonus(60.0)
        }
    }
}

impl SpecialAbility for ViridescentVenerer {
    fn modify(&self, _modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        if data.vision == Vision::Anemo {
            match &enemy.aura.aura {
                Vision::Pyro |
                Vision::Hydro |
                Vision::Electro |
                Vision::Cryo => enemy.element_res_debuff.push(Debuff::viridescent_venerer()),
                _ => (),
            }
        }
    }
}

#[derive(Debug)]
pub struct VVem(ViridescentVenerer);

impl VVem {
    pub fn new() -> Self {
        Self(ViridescentVenerer)
    }
}

impl ArtifactAbility for VVem {
    fn record(&self) -> Artifact {
        Artifact {
            name: "Viridescent Venerer (EM)",
            version: 1.0,
            preference: vec![Preference::Anemo],
            state: State::new().anemo_dmg(15.0).transformative_bonus(60.0).em(6.012 * (53.333+80.0)).atk(-80.0).cr(-80.0)
        }
    }
}

impl SpecialAbility for VVem {
    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, timers, data, enemy);
    }
}

#[derive(Debug)]
pub struct ArchaicPetra;

impl SpecialAbility for ArchaicPetra {}

impl ArtifactAbility for ArchaicPetra {
    fn record(&self) -> Artifact {
        Artifact {
            name: "Archaic Petra",
            version: 1.0,
            preference: vec![Preference::Geo],
            state: State::new().geo_dmg(15.0)
        }
    }
}

#[derive(Debug)]
pub struct CrimsonWitchOfFlames;

impl SpecialAbility for CrimsonWitchOfFlames {}

impl ArtifactAbility for CrimsonWitchOfFlames {
    fn record(&self) -> Artifact {
        Artifact {
            name: "Crimson Witch of Flames",
            version: 1.0,
            preference: vec![Preference::Pyro],
            state: State::new().pyro_dmg(15.0+7.5).amplifying_bonus(15.0).transformative_bonus(40.0)
        }
    }
}

#[derive(Debug)]
pub struct CrimsonWitchOfFlamesHp;

impl SpecialAbility for CrimsonWitchOfFlamesHp {}

impl ArtifactAbility for CrimsonWitchOfFlamesHp {
    fn record(&self) -> Artifact {
        Artifact {
            name: "Crimson Witch of Flames (HP)",
            version: 1.0,
            preference: vec![Preference::Pyro],
            state: State::new().pyro_dmg(15.0+7.5).amplifying_bonus(15.0).transformative_bonus(40.0).hp(80.0).atk(-80.0)
        }
    }
}

#[derive(Debug)]
pub struct NoblesseOblige {
    timer: DurationTimer
}

impl NoblesseOblige {
    pub fn new() -> Self {
        Self { timer: DurationTimer::new(0.0, 12.0) }
    }
}

impl ArtifactAbility for NoblesseOblige {
    fn record(&self) -> Artifact {
        Artifact {
            name: "Noblesse Oblige",
            version: 1.0,
            preference: vec![Preference::Supporter],
            state: State::new().burst_dmg(20.0)
        }
    }
}

impl SpecialAbility for NoblesseOblige {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        self.timer.update(guard.check_second(Burst), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, _data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            for s in modifiable_state.iter_mut() {
                if s.stacked_buff != UnstackableBuff::NoblesseOblige() {
                    s.atk += 20.0;
                    s.stacked_buff += UnstackableBuff::NoblesseOblige();
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

impl ArtifactAbility for Gfno {
    fn record(&self) -> Artifact {
        Artifact {
            name: "GFNO ATK 18% Burst 20%",
            version: 1.0,
            preference: vec![Preference::Supporter],
            state: State::new().burst_dmg(20.0).atk(18.0)
        }
    }
}

#[derive(Debug)]
pub struct GladiatorsFinale {
    bonus: f32,
    checked: bool,
}

impl GladiatorsFinale {
    fn new() -> Self {
        Self { bonus: 0.0, checked: false }
    }
}

impl ArtifactAbility for GladiatorsFinale {
    fn record(&self) -> Artifact {
        Artifact {
            name: "Gladiator's Finale",
            version: 1.0,
            preference: vec![Preference::Melee],
            state: State::new().atk(18.0)
        }
    }
}

impl SpecialAbility for GladiatorsFinale {
    fn update(&mut self, _guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], data: &CharacterData, _enemy: &Enemy, _time: f32) -> () {
        if !self.checked {
            self.checked = true;
            match data.cr.weapon {
                WeaponType::Sword    => self.bonus = 35.0,
                WeaponType::Claymore => self.bonus = 35.0,
                WeaponType::Polearm  => self.bonus = 35.0,
                _ => ()
            };
        }
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        modifiable_state[data.idx.0].na_dmg += self.bonus;
    }

    fn reset(&mut self) -> () {
        self.bonus = 0.0;
        self.checked = false;
    }
}

#[derive(Debug)]
pub struct GladiatorsFinaleDef(GladiatorsFinale);

impl GladiatorsFinaleDef {
    fn new() -> Self {
        Self(GladiatorsFinale::new())
    }
}

impl ArtifactAbility for GladiatorsFinaleDef {
    fn record(&self) -> Artifact {
        Artifact {
            name: "Gladiator's Finale (DEF)",
            version: 1.0,
            preference: vec![Preference::Melee],
            state: State::new().atk(18.0-80.0).def(110.0)
        }
    }
}

impl SpecialAbility for GladiatorsFinaleDef {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, attack: &[ElementalAttack], particles: &[Particle], data: &CharacterData, enemy: &Enemy, time: f32) -> () {
        self.0.update(guard, timers, attack, particles, data, enemy, time);
    }

    fn modify(&self, modifiable_state: &mut [State], timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        self.0.modify(modifiable_state, timers, data, enemy);
    }

    fn reset(&mut self) -> () {
        self.0.reset();
    }
}

#[derive(Debug)]
pub struct WanderersTroupe;

impl SpecialAbility for WanderersTroupe {}

impl ArtifactAbility for WanderersTroupe {
    fn record(&self) -> Artifact {
        Artifact {
            name: "Wanderer's Troupe",
            version: 1.0,
            preference: vec![Preference::Ranged],
            state: State::new().ca_dmg(35.0).em(80.0)
        }
    }
}

#[derive(Debug)]
pub struct RetracingBolide;

impl SpecialAbility for RetracingBolide {}

impl ArtifactAbility for RetracingBolide {
    fn record(&self) -> Artifact {
        Artifact {
            name: "Retracing Bolide",
            version: 1.0,
            preference: vec![Preference::Attacker],
            state: State::new().na_dmg(40.0).ca_dmg(40.0)
        }
    }
}

#[derive(Debug)]
pub struct RetracingBolideDef;

impl SpecialAbility for RetracingBolideDef {}

impl ArtifactAbility for RetracingBolideDef {
    fn record(&self) -> Artifact {
        Artifact {
            name: "Retracing Bolide (DEF)",
            version: 1.0,
            preference: vec![Preference::Attacker],
            state: State::new().na_dmg(40.0).ca_dmg(40.0).atk(-80.0).def(110.0)
        }
    }
}

#[derive(Debug)]
pub struct Thundersoother;

impl ArtifactAbility for Thundersoother {
    fn record(&self) -> Artifact {
        Artifact {
            name: "Thundersoother",
            version: 1.0,
            preference: vec![Preference::Electro],
            state: State::new()
        }
    }
}

impl SpecialAbility for Thundersoother {
    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        match &enemy.aura.aura {
            Vision::Electro => modifiable_state[data.idx.0].all_dmg += 35.0,
            _ => (),
        }
    }
}

#[derive(Debug)]
pub struct Lavawalker;

impl ArtifactAbility for Lavawalker {
    fn record(&self) -> Artifact {
        Artifact {
            name: "Lavawalker",
            version: 1.0,
            preference: vec![Preference::Pyro],
            state: State::new()
        }
    }
}

impl SpecialAbility for Lavawalker {
    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        match &enemy.aura.aura {
            Vision::Pyro => modifiable_state[data.idx.0].all_dmg += 35.0,
            _ => (),
        }
    }
}

#[derive(Debug)]
pub struct LavawalkerHp;

impl ArtifactAbility for LavawalkerHp {
    fn record(&self) -> Artifact {
        Artifact {
            name: "Lavawalker (HP)",
            version: 1.0,
            preference: vec![Preference::Pyro],
            state: State::new().atk(-80.0).hp(80.0)
        }
    }
}

impl SpecialAbility for LavawalkerHp {
    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        match &enemy.aura.aura {
            Vision::Pyro => modifiable_state[data.idx.0].all_dmg += 35.0,
            _ => (),
        }
    }
}

#[derive(Debug)]
pub struct Gfelm;

impl SpecialAbility for Gfelm {}

impl ArtifactAbility for Gfelm {
    fn record(&self) -> Artifact {
        Artifact {
            name: "GFElem ATK 18% DMG 15%",
            version: 1.0,
            preference: Vec::new(),
            state: State::new().atk(18.0).elemental_dmg(15.0)
        }
    }
}

#[derive(Debug)]
pub struct BlizzardStrayer;

impl ArtifactAbility for BlizzardStrayer {
    fn record(&self) -> Artifact {
        Artifact {
            name: "Blizzard Strayer",
            version: 1.2,
            preference: vec![Preference::Cryo, Preference::Hydro],
            state: State::new().cryo_dmg(15.0)
        }
    }
}

impl SpecialAbility for BlizzardStrayer {
    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, enemy: &mut Enemy) -> () {
        match (enemy.isfrozen, &enemy.aura.aura) {
            (true,  Vision::Cryo) => modifiable_state[data.idx.0].cr += 40.0,
            (false, Vision::Cryo) => modifiable_state[data.idx.0].cr += 20.0,
            _ => (),
        }
    }
}

#[derive(Debug)]
pub struct HeartOfDepth {
    timer: DurationTimer
}

impl HeartOfDepth {
    pub fn new() -> Self {
        Self { timer: DurationTimer::new(0.0, 15.0) }
    }
}

impl ArtifactAbility for HeartOfDepth {
    fn record(&self) -> Artifact {
        Artifact {
            name: "Heart of Depth",
            version: 1.2,
            preference: vec![Preference::Hydro],
            state: State::new().hydro_dmg(15.0)
        }
    }
}

impl SpecialAbility for HeartOfDepth {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        // let should_update = timers.press_timer().is_active() || timers.hold_timer().is_active();
        // unsafe {
        //     attack.iter().any(|&a| match (*a).kind {
        //         PressSkill | HoldSkill => true,
        //         _ => false,
        //     })
        // };
        self.timer.update(guard.second(guard.kind == PressSkill || guard.kind == HoldSkill), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            let state = &mut modifiable_state[data.idx.0];
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
    timer: DurationTimer
}

impl GlacierAndSnowfield {
    pub fn new() -> Self {
        Self { timer: DurationTimer::new(0.0, 10.0) }
    }
}

impl ArtifactAbility for GlacierAndSnowfield {
    fn record(&self) -> Artifact {
        Artifact {
            name: "Glacier and Snowfield",
            version: 99.0,
            preference: vec![Preference::Cryo],
            state: State::new().cryo_dmg(15.0).amplifying_bonus(15.0).transformative_bonus(100.0)
        }
    }
}

impl SpecialAbility for GlacierAndSnowfield {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        // let should_update = timers.burst_timer().is_active();
        // unsafe {
        //     attack.iter().any(|&a| match (*a).kind {
        //         Burst => true,
        //         _ => false,
        //     })
        // };
        self.timer.update(guard.check_second(Burst), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            modifiable_state[data.idx.0].cryo_dmg += 30.0;
        }
    }

    fn reset(&mut self) -> () {
        self.timer.reset();
    }
}

#[derive(Debug)]
pub struct PaleFlame {
    timer: StackTimer
}

impl PaleFlame {
    pub fn new() -> Self {
        Self { timer: StackTimer::new(0.3, 7.0, 2) }
    }
}

impl ArtifactAbility for PaleFlame {
    fn record(&self) -> Artifact {
        Artifact {
            name: "Pale Flame",
            version: 1.5,
            preference: vec![Preference::Physical],
            state: State::new().physical_dmg(25.0)
        }
    }
}

impl SpecialAbility for PaleFlame {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        // let should_update = unsafe {
        //     attack.iter().any(|&a| match (*a).kind {
        //         PressSkill | HoldSkill | SkillDot => true,
        //         _ => false,
        //     })
        // };
        let should_update = timers.press_timer().is_active() || timers.hold_timer().is_active();
        self.timer.update(guard.second(should_update), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            let state = &mut modifiable_state[data.idx.0];
            match self.timer.n {
                2 => {
                    state.atk += 18.0;
                    state.physical_dmg += 25.0;
                },
                1 => state.atk += 9.0,
                _ => (),
            };
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
        Self { timer: DurationTimer::new(0.5, 3.0) }
    }
}

impl ArtifactAbility for TenacityOfTheMillelith {
    fn record(&self) -> Artifact {
        Artifact {
            name: "Tenacity of the Millelith",
            version: 1.5,
            preference: vec![Preference::Supporter],
            state: State::new().hp(20.0)
        }
    }
}

impl SpecialAbility for TenacityOfTheMillelith {
    fn update(&mut self, guard: &mut TimerGuard, timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        // let should_update = unsafe {
        //     attack.iter().any(|&a| match (*a).kind {
        //         PressSkill | HoldSkill | SkillDot => true,
        //         _ => false,
        //     })
        // };
        let should_update = timers.press_timer().is_active() || timers.hold_timer().is_active();
        self.timer.update(guard.second(should_update), time);
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, _data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self.timer.is_active() {
            for s in modifiable_state.iter_mut() {
                if s.stacked_buff != UnstackableBuff::TenacityOfTheMillelith() {
                    s.atk += 20.0;
                    s.stacked_buff += UnstackableBuff::TenacityOfTheMillelith();
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
    first_activation: bool,
    cd: f32,
    duration: f32,
    _cd: f32,
    _dr: f32,
}

impl ShimenawasReminiscence {
    fn new() -> Self {
        Self { first_activation: false, cd: 0.0, duration: 10.0, _cd: 0.0, _dr: 0.0 }
    }
}

// 4 Piece: When casting an Elemental Skill, if the character has 15 or more
// Energy, they lose 15 Energy and Normal/Charged/ Plunging Attack DMG is
// increased by 50% for 10s.
impl ArtifactAbility for ShimenawasReminiscence {
    fn record(&self) -> Artifact {
        Artifact {
            name: "Shimenawa's Reminiscence",
            version: 2.0,
            preference: vec![Preference::Attacker],
            state: State::new().atk(18.0)
        }
    }
}

impl SpecialAbility for ShimenawasReminiscence {
    fn update(&mut self, guard: &mut TimerGuard, _timers: &FullCharacterTimers, _attack: &[ElementalAttack], _particles: &[Particle], _data: &CharacterData, _enemy: &Enemy, time: f32) -> () {
        // let should_update = owner_fc.state.energy.0 >= 15.0 && unsafe {
        //     attack.iter().any(|&a| match (*a).kind {
        //         PressSkill | HoldSkill => true,
        //         _ => false,
        //     })
        // };
        guard.second(guard.kind == PressSkill || guard.kind == HoldSkill);
        guard.third(self._cd > 0.0);
        if !guard.check(()) {
            return;
        }
        if guard.second && self._cd <= 0.0 {
            self._cd = self.cd;
            self._dr = self.duration;
        }
        // notify the first time activation
        self.first_activation = self._dr == self.duration;
        self._cd -= time;
        self._dr -= time;
    }

    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        if self._dr > 0.0 && data.state.energy.0 >= 15.0 && self.first_activation {
            let state = &mut modifiable_state[data.idx.0];
            state.energy.0 -= 15.0;
            state.na_dmg += 50.0;
            state.ca_dmg += 50.0;
        } else if self._dr > 0.0 {
            let state = &mut modifiable_state[data.idx.0];
            state.na_dmg += 50.0;
            state.ca_dmg += 50.0;
        }
    }

    fn reset(&mut self) -> () {
        self._cd = 0.0;
        self._dr = 0.0;
    }
}

#[derive(Debug)]
pub struct GfShimenawa;

impl SpecialAbility for GfShimenawa {}

impl ArtifactAbility for GfShimenawa {
    fn record(&self) -> Artifact {
        Artifact {
            name: "GFShimenawa ATK 36%",
            version: 2.0,
            preference: Vec::new(),
            state: State::new().atk(36.0)
        }
    }
}

#[derive(Debug)]
pub struct EmblemOfSeveredFate;

// 4 Piece: Increases Elemental Burst DMG by 25% of Energy Recharge. A maximum
// 75% DMG increase can be obtained in this way.
impl ArtifactAbility for EmblemOfSeveredFate {
    fn record(&self) -> Artifact {
        Artifact {
            name: "Emblem of Severed Fate",
            version: 2.0,
            preference: Vec::new(),
            state: State::new().er(20.0)
        }
    }
}

impl SpecialAbility for EmblemOfSeveredFate {
    fn modify(&self, modifiable_state: &mut [State], _timers: &FullCharacterTimers, data: &CharacterData, _enemy: &mut Enemy) -> () {
        // the maximum DMG bonus is obtained if ER is 300%.
        // `State.er` does not contain base 100% of characters.
        let er = 100.0 + data.state.er;
        modifiable_state[data.idx.0].burst_dmg += if er > 300.0 {
            75.0
        } else {
            er * 0.25
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulate::simulate;
    use crate::types::{Vision, ElementalGauge, ElementalGaugeDecay};
    use crate::fc::{FieldCharacterIndex};
    use crate::testutil::{TestEnvironment};

    use Vision::*;

    // fc0 triggers burst, which is invariant to fc1 who equips an artifact
    // that can be triggered by own burst.
    #[test]
    fn invariance_0() {
        let mut env1 = TestEnvironment::new();
        let mut env2 = TestEnvironment::new();
        let mut aa = NoblesseOblige::new();
        let mut members = vec![
            env1.vision(FieldCharacterIndex(0), State::new(), Pyro),
            env2.artifact(FieldCharacterIndex(1), State::new(), Pyro, &mut aa),
            ];
        members[0].fc.data.state.energy.0 = members[0].fc.data.cr.energy_cost;
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..21 {
            total_dmg += simulate(&mut members, &mut enemy, 0.1);
        }
        // (burst skill na na na) and (skill na na na)
        let expect = (300.0 + 200.0 + 100.0 + 100.0 + 100.0)
                   + (200.0 + 100.0 + 100.0 + 100.0);
        assert_eq!(total_dmg, 0.5 * expect);
    }

    #[test]
    fn invariance_1() {
        let mut env1 = TestEnvironment::new();
        let mut env2 = TestEnvironment::new();
        let mut aa = NoblesseOblige::new();
        let mut members = vec![
            env1.artifact(FieldCharacterIndex(0), State::new(), Pyro, &mut aa),
            env2.vision(FieldCharacterIndex(1), State::new(), Pyro),
            ];
        members[0].fc.data.state.energy.0 = members[0].fc.data.cr.energy_cost;
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..21 {
            total_dmg += simulate(&mut members, &mut enemy, 0.1);
        }
        // (burst skill na na na) and (skill na na na)
        let expect = 1.2 * (360.0 + 200.0 + 100.0 + 100.0 + 100.0)
                   + 1.2 * (200.0 + 100.0 + 100.0 + 100.0);
        let differnce = (total_dmg - 0.5 * expect).abs();
        assert!(differnce <= 0.001);
    }

    #[test]
    fn noblesse_oblige_unstackable() {
        let mut env1 = TestEnvironment::new();
        let mut env2 = TestEnvironment::new();
        let mut aa1 = NoblesseOblige::new();
        let mut aa2 = NoblesseOblige::new();
        let mut members = vec![
            env1.artifact(FieldCharacterIndex(0), State::new(), Pyro, &mut aa1),
            env2.artifact(FieldCharacterIndex(1), State::new(), Pyro, &mut aa2),
            ];
        members[0].fc.data.state.energy.0 = members[0].fc.data.cr.energy_cost;
        members[1].fc.data.state.energy.0 = members[1].fc.data.cr.energy_cost;
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..21 {
            total_dmg += simulate(&mut members, &mut enemy, 0.1);
        }
        // twice (burst skill na na na)
        let expect = 1.2 * (360.0 + 200.0 + 100.0 + 100.0 + 100.0)
                   + 1.2 * (360.0 + 200.0 + 100.0 + 100.0 + 100.0);
        let differnce = (total_dmg - 0.5 * expect).abs();
        assert!(differnce <= 0.001);
    }

    #[test]
    fn viridescent_venerer() {
        let mut env = TestEnvironment::new();
        let mut aa = ViridescentVenerer;
        let mut members = vec![
            env.artifact(FieldCharacterIndex(0), State::new().infusion(true), Anemo, &mut aa),
        ];
        // members[0].fc.data.ar.state.infusion = true;
        let mut enemy = TestEnvironment::enemy();
        enemy.aura = ElementalGauge {
            aura: Vision::Pyro,
            unit: 1.0,
            decay: ElementalGaugeDecay::A,
        };
        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        let expect = 0.5 * (
            // skill (level multiplier * reaction multiplier * bonus (* TODO bypass enemy defense))
              725.36 * 1.2 * 1.6 * 2.0 + 200.0 * 1.15 * 1.2
            // na
            + 725.36 * 1.2 * 1.6 * 2.0 + 100.0 * 1.15 * 1.2
            // na (action multiplier * vv 2 set bonus * vv 4 set RES down)
            + 100.0 * 1.15 * 1.2
            // na
            + 100.0 * 1.15 * 1.2
            // na
            + 100.0 * 1.15 * 1.2
        );
        let differnce = (total_dmg - expect).abs();
        assert!(differnce <= 0.001);
    }

    #[test]
    fn paleflame_1() {
        let mut env = TestEnvironment::new();
        let mut aa = PaleFlame::new();
        let mut members = vec![
            // disable physical bonus
            env.artifact(FieldCharacterIndex(0), State::new().infusion(true), Pyro, &mut aa),
        ];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..41 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // skill 15 na, skill 5 na
        let expect = 0.5 * (
              1.09 * (200.0 + 15.0 * 100.0)
            + 1.18 * (200.0 + 5.0 * 100.0)
        );
        assert_eq!(total_dmg, expect);
    }
}
