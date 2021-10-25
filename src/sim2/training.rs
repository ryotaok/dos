use crate::sim2::state::State;
use crate::sim2::timeline::{Timeline};
use crate::sim2::attack::{WeaponAttack};
use crate::sim2::types::{WeaponType, SCORE};
use crate::sim2::record::{WeaponRecord, Artifact};

use WeaponType::*;

// 
// weapons
// 

pub enum TrainingWeaponUnion {
    TrainingSword(TrainingSword),
    TrainingClaymore(TrainingClaymore),
    TrainingPolearm(TrainingPolearm),
    TrainingBow(TrainingBow),
    TrainingCatalyst(TrainingCatalyst),
}

impl TrainingWeaponUnion {
    pub fn timeline(&mut self) -> &mut dyn Timeline {
        use TrainingWeaponUnion::*;
        match self {
            TrainingSword(x) => x,
            TrainingClaymore(x) => x,
            TrainingPolearm(x) => x,
            TrainingBow(x) => x,
            TrainingCatalyst(x) => x,
        }
    }

    pub fn field(&mut self) -> &mut dyn WeaponAttack {
        use TrainingWeaponUnion::*;
        match self {
            TrainingSword(x) => x,
            TrainingClaymore(x) => x,
            TrainingPolearm(x) => x,
            TrainingBow(x) => x,
            TrainingCatalyst(x) => x,
        }
    }
}

pub fn weapons() -> Vec<(WeaponRecord, TrainingWeaponUnion)> {
    vec![
    (TrainingSword::record(), TrainingWeaponUnion::TrainingSword(TrainingSword)),
    (TrainingClaymore::record(), TrainingWeaponUnion::TrainingClaymore(TrainingClaymore)),
    (TrainingPolearm::record(), TrainingWeaponUnion::TrainingPolearm(TrainingPolearm)),
    (TrainingBow::record(), TrainingWeaponUnion::TrainingBow(TrainingBow)),
    (TrainingCatalyst::record(), TrainingWeaponUnion::TrainingCatalyst(TrainingCatalyst)),
    ]
}

#[derive(Debug)]
pub struct TrainingSword;
impl Timeline for TrainingSword {}
impl WeaponAttack for TrainingSword {}
impl TrainingSword {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Training Sword").type_(Sword).version(1.0)
            .base_atk(608.)
    }
}

#[derive(Debug)]
pub struct TrainingClaymore;
impl Timeline for TrainingClaymore {}
impl WeaponAttack for TrainingClaymore {}
impl TrainingClaymore {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Training Claymore").type_(Claymore).version(1.0)
            .base_atk(608.)
    }
}

#[derive(Debug)]
pub struct TrainingPolearm;
impl Timeline for TrainingPolearm {}
impl WeaponAttack for TrainingPolearm {}
impl TrainingPolearm {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Training Polearm").type_(Polearm).version(1.0)
            .base_atk(608.)
    }
}

#[derive(Debug)]
pub struct TrainingBow;
impl Timeline for TrainingBow {}
impl WeaponAttack for TrainingBow {}
impl TrainingBow {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Training Bow").type_(Bow).version(1.0)
            .base_atk(608.)
    }
}

#[derive(Debug)]
pub struct TrainingCatalyst;
impl Timeline for TrainingCatalyst {}
impl WeaponAttack for TrainingCatalyst {}
impl TrainingCatalyst {
    pub fn record() -> WeaponRecord {
        WeaponRecord::default()
            .name("Training Catalyst").type_(Catalyst).version(1.0)
            .base_atk(608.)
    }
}

// 
// artifacts
// 

pub enum TrainingArtifactUnion {
    TrainingArtifact0(TrainingArtifact0),
    TrainingArtifact1(TrainingArtifact1),
    TrainingArtifact2(TrainingArtifact2),
    TrainingArtifact3(TrainingArtifact3),
    TrainingArtifact4(TrainingArtifact4),
    TrainingArtifact5(TrainingArtifact5),
    TrainingArtifact6(TrainingArtifact6),
    TrainingArtifact7(TrainingArtifact7),
    TrainingArtifact8(TrainingArtifact8),
    TrainingArtifact9(TrainingArtifact9),
}

impl TrainingArtifactUnion {
    pub fn timeline(&mut self) -> &mut dyn Timeline {
        use TrainingArtifactUnion::*;
        match self {
            TrainingArtifact0(x) => x,
            TrainingArtifact1(x) => x,
            TrainingArtifact2(x) => x,
            TrainingArtifact3(x) => x,
            TrainingArtifact4(x) => x,
            TrainingArtifact5(x) => x,
            TrainingArtifact6(x) => x,
            TrainingArtifact7(x) => x,
            TrainingArtifact8(x) => x,
            TrainingArtifact9(x) => x,
        }
    }

    pub fn field(&mut self) -> &mut dyn WeaponAttack {
        use TrainingArtifactUnion::*;
        match self {
            TrainingArtifact0(x) => x,
            TrainingArtifact1(x) => x,
            TrainingArtifact2(x) => x,
            TrainingArtifact3(x) => x,
            TrainingArtifact4(x) => x,
            TrainingArtifact5(x) => x,
            TrainingArtifact6(x) => x,
            TrainingArtifact7(x) => x,
            TrainingArtifact8(x) => x,
            TrainingArtifact9(x) => x,
        }
    }
}

pub fn artifacts() -> Vec<(Artifact, TrainingArtifactUnion)> {
    vec![
    (TrainingArtifact0::record(), TrainingArtifactUnion::TrainingArtifact0(TrainingArtifact0)),
    (TrainingArtifact1::record(), TrainingArtifactUnion::TrainingArtifact1(TrainingArtifact1)),
    (TrainingArtifact2::record(), TrainingArtifactUnion::TrainingArtifact2(TrainingArtifact2)),
    (TrainingArtifact3::record(), TrainingArtifactUnion::TrainingArtifact3(TrainingArtifact3)),
    (TrainingArtifact4::record(), TrainingArtifactUnion::TrainingArtifact4(TrainingArtifact4)),
    (TrainingArtifact5::record(), TrainingArtifactUnion::TrainingArtifact5(TrainingArtifact5)),
    (TrainingArtifact6::record(), TrainingArtifactUnion::TrainingArtifact6(TrainingArtifact6)),
    (TrainingArtifact7::record(), TrainingArtifactUnion::TrainingArtifact7(TrainingArtifact7)),
    (TrainingArtifact8::record(), TrainingArtifactUnion::TrainingArtifact8(TrainingArtifact8)),
    (TrainingArtifact9::record(), TrainingArtifactUnion::TrainingArtifact9(TrainingArtifact9)),
    ]
}

#[derive(Debug)]
pub struct TrainingArtifact0;
impl Timeline for TrainingArtifact0 {}
impl WeaponAttack for TrainingArtifact0 {}
impl TrainingArtifact0 {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("Atk70 Em280 Er78").version(1.0).preference(&[])
            .elemental_dmg(15.).physical_dmg(25.).cr(20.)
            .atk(SCORE.atk(33.3333))
            .em(SCORE.em(33.3333))
            .er(SCORE.er(33.3333))
    }
}

#[derive(Debug)]
pub struct TrainingArtifact1;
impl Timeline for TrainingArtifact1 {}
impl WeaponAttack for TrainingArtifact1 {}
impl TrainingArtifact1 {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("Atk168 Em167").version(1.0).preference(&[])
            .elemental_dmg(15.).physical_dmg(25.).cr(20.)
            .atk(SCORE.atk(80.))
            .em(SCORE.em(20.))
    }
}

#[derive(Debug)]
pub struct TrainingArtifact2;
impl Timeline for TrainingArtifact2 {}
impl WeaponAttack for TrainingArtifact2 {}
impl TrainingArtifact2 {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("Atk168 Er46").version(1.0).preference(&[])
            .elemental_dmg(15.).physical_dmg(25.).cr(20.)
            .atk(SCORE.atk(80.))
            .er(SCORE.er(20.))
    }
}

#[derive(Debug)]
pub struct TrainingArtifact3;
impl Timeline for TrainingArtifact3 {}
impl WeaponAttack for TrainingArtifact3 {}
impl TrainingArtifact3 {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("Atk41 Em671").version(1.0).preference(&[])
            .elemental_dmg(15.).physical_dmg(25.).cr(20.)
            .atk(SCORE.atk(20.))
            .em(SCORE.em(80.))
    }
}

#[derive(Debug)]
pub struct TrainingArtifact4;
impl Timeline for TrainingArtifact4 {}
impl WeaponAttack for TrainingArtifact4 {}
impl TrainingArtifact4 {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("Em671 Er46").version(1.0).preference(&[])
            .elemental_dmg(15.).physical_dmg(25.).cr(20.)
            .em(SCORE.em(80.))
            .er(SCORE.er(20.))
    }
}

#[derive(Debug)]
pub struct TrainingArtifact5;
impl Timeline for TrainingArtifact5 {}
impl WeaponAttack for TrainingArtifact5 {}
impl TrainingArtifact5 {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("Atk42 Er186").version(1.0).preference(&[])
            .elemental_dmg(15.).physical_dmg(25.).cr(20.)
            .atk(SCORE.atk(20.))
            .er(SCORE.er(80.))
    }
}

#[derive(Debug)]
pub struct TrainingArtifact6;
impl Timeline for TrainingArtifact6 {}
impl WeaponAttack for TrainingArtifact6 {}
impl TrainingArtifact6 {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("Em167 Er186").version(1.0).preference(&[])
            .elemental_dmg(15.).physical_dmg(25.).cr(20.)
            .em(SCORE.em(20.))
            .er(SCORE.er(80.))
    }
}

#[derive(Debug)]
pub struct TrainingArtifact7;
impl Timeline for TrainingArtifact7 {}
impl WeaponAttack for TrainingArtifact7 {}
impl TrainingArtifact7 {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("Atk105 Em420").version(1.0).preference(&[])
            .elemental_dmg(15.).physical_dmg(25.).cr(20.)
            .atk(SCORE.atk(50.))
            .em(SCORE.em(50.))
    }
}

#[derive(Debug)]
pub struct TrainingArtifact8;
impl Timeline for TrainingArtifact8 {}
impl WeaponAttack for TrainingArtifact8 {}
impl TrainingArtifact8 {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("Atk105 Er116").version(1.0).preference(&[])
            .elemental_dmg(15.).physical_dmg(25.).cr(20.)
            .atk(SCORE.atk(50.))
            .er(SCORE.er(50.))
    }
}

#[derive(Debug)]
pub struct TrainingArtifact9;
impl Timeline for TrainingArtifact9 {}
impl WeaponAttack for TrainingArtifact9 {}
impl TrainingArtifact9 {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("Em420 Er116").version(1.0).preference(&[])
            .elemental_dmg(15.).physical_dmg(25.).cr(20.)
            .em(SCORE.em(50.))
            .er(SCORE.er(50.))
    }
}

#[derive(Debug)]
pub struct TrainingArtifact10;
impl Timeline for TrainingArtifact10 {}
impl WeaponAttack for TrainingArtifact10 {}
impl TrainingArtifact10 {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("Atk210").version(1.0).preference(&[])
            .elemental_dmg(15.).physical_dmg(25.).cr(20.)
            .atk(SCORE.atk(100.))
    }
}

#[derive(Debug)]
pub struct TrainingArtifact11;
impl Timeline for TrainingArtifact11 {}
impl WeaponAttack for TrainingArtifact11 {}
impl TrainingArtifact11 {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("Em839").version(1.0).preference(&[])
            .elemental_dmg(15.).physical_dmg(25.).cr(20.)
            .em(SCORE.em(100.))
    }
}

#[derive(Debug)]
pub struct TrainingArtifact12;
impl Timeline for TrainingArtifact12 {}
impl WeaponAttack for TrainingArtifact12 {}
impl TrainingArtifact12 {
    pub fn record() -> Artifact {
        Artifact::default()
            .name("Er233").version(1.0).preference(&[])
            .elemental_dmg(15.).physical_dmg(25.).cr(20.)
            .er(SCORE.er(100.))
    }
}
