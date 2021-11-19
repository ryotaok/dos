use crate::sim2::timeline::Timeline;
use crate::sim2::attack::CharacterAttack;
use crate::sim2::record::{CharacterRecord};

pub mod pyro;
pub mod hydro;
pub mod electro;
pub mod cryo;
pub mod anemo;
pub mod geo;
pub mod version_1_1;
pub mod version_1_2;
pub mod version_1_3;
pub mod version_1_4;
pub mod version_1_5;
pub mod version_1_6;
pub mod version_2_0;
pub mod version_2_1;
pub mod version_2_2;
pub mod version_2_3;

use pyro::*;
use hydro::*;
use electro::*;
use cryo::*;
use anemo::*;
use geo::*;
use version_1_1::*;
use version_1_2::*;
use version_1_3::*;
use version_1_4::*;
use version_1_5::*;
use version_1_6::*;
use version_2_0::*;
use version_2_1::*;
use version_2_2::*;
use version_2_3::*;

#[derive(Debug)]
pub enum CharacterUnion {
    // pyro
    Amber(Amber),
    Bennett(Bennett),
    Xiangling(Xiangling),
    Diluc(Diluc),
    Klee(Klee),
    // hydro
    Barbara(Barbara),
    Xingqiu(Xingqiu),
    Mona(Mona),
    // electro
    Beidou(Beidou),
    Fischl(Fischl),
    Lisa(Lisa),
    Razor(Razor),
    Keqing(Keqing),
    // cryo
    Chongyun(Chongyun),
    Kaeya(Kaeya),
    Qiqi(Qiqi),
    // anemo
    Sucrose(Sucrose),
    TravelerAnemo(TravelerAnemo),
    Jean(Jean),
    Venti(Venti),
    // geo
    Ningguang(Ningguang),
    Noelle(Noelle),
    TravelerGeo(TravelerGeo),
    // version_1_1
    Tartaglia(Tartaglia),
    Diona(Diona),
    Zhongli(Zhongli),
    Xinyan(Xinyan),
    // version_1_2
    Albedo(Albedo),
    Ganyu(Ganyu),
    // version_1_3
    Xiao(Xiao),
    HuTao(HuTao),
    // version_1_4
    Rosaria(Rosaria),
    // version_1_5
    Yanfei(Yanfei),
    Eula(Eula),
    // version_1_6
    Kazuha(Kazuha),
    // version_2_0
    Ayaka(Ayaka),
    Yoimiya(Yoimiya),
    Sayu(Sayu),
    TravelerElectro(TravelerElectro),
    // version_2_1
    RaidenShogun(RaidenShogun),
    KujouSara(KujouSara),
    Aloy(Aloy),
    SangonomiyaKokomi(SangonomiyaKokomi),
    // version_2_2
    Thoma(Thoma),
    // version_2_3
    AratakiItto(AratakiItto),
    Gorou(Gorou),
}

impl CharacterUnion {
    pub fn timeline(&mut self) -> &mut dyn Timeline {
        use CharacterUnion::*;
        match self {
            // pyro
            Amber(x) => x,
            Bennett(x) => x,
            Xiangling(x) => x,
            Diluc(x) => x,
            Klee(x) => x,
            // hydro
            Barbara(x) => x,
            Xingqiu(x) => x,
            Mona(x) => x,
            // electro
            Beidou(x) => x,
            Fischl(x) => x,
            Lisa(x) => x,
            Razor(x) => x,
            Keqing(x) => x,
            // cryo
            Chongyun(x) => x,
            Kaeya(x) => x,
            Qiqi(x) => x,
            // anemo
            Sucrose(x) => x,
            TravelerAnemo(x) => x,
            Jean(x) => x,
            Venti(x) => x,
            // geo
            Ningguang(x) => x,
            Noelle(x) => x,
            TravelerGeo(x) => x,
            // version_1_1
            Tartaglia(x) => x,
            Diona(x) => x,
            Zhongli(x) => x,
            Xinyan(x) => x,
            // version_1_2
            Albedo(x) => x,
            Ganyu(x) => x,
            // version_1_3
            Xiao(x) => x,
            HuTao(x) => x,
            // version_1_4
            Rosaria(x) => x,
            // version_1_5
            Yanfei(x) => x,
            Eula(x) => x,
            // version_1_6
            Kazuha(x) => x,
            // version_2_0
            Ayaka(x) => x,
            Yoimiya(x) => x,
            Sayu(x) => x,
            TravelerElectro(x) => x,
            // version_2_1
            RaidenShogun(x) => x,
            KujouSara(x) => x,
            Aloy(x) => x,
            SangonomiyaKokomi(x) => x,
            // version_2_2
            Thoma(x) => x,
            // version_2_3
            AratakiItto(x) => x,
            Gorou(x) => x,
        }
    }

    pub fn field(&mut self) -> &mut dyn CharacterAttack {
        use CharacterUnion::*;
        match self {
            // pyro
            Amber(x) => x,
            Bennett(x) => x,
            Xiangling(x) => x,
            Diluc(x) => x,
            Klee(x) => x,
            // hydro
            Barbara(x) => x,
            Xingqiu(x) => x,
            Mona(x) => x,
            // electro
            Beidou(x) => x,
            Fischl(x) => x,
            Lisa(x) => x,
            Razor(x) => x,
            Keqing(x) => x,
            // cryo
            Chongyun(x) => x,
            Kaeya(x) => x,
            Qiqi(x) => x,
            // anemo
            Sucrose(x) => x,
            TravelerAnemo(x) => x,
            Jean(x) => x,
            Venti(x) => x,
            // geo
            Ningguang(x) => x,
            Noelle(x) => x,
            TravelerGeo(x) => x,
            // version_1_1
            Tartaglia(x) => x,
            Diona(x) => x,
            Zhongli(x) => x,
            Xinyan(x) => x,
            // version_1_2
            Albedo(x) => x,
            Ganyu(x) => x,
            // version_1_3
            Xiao(x) => x,
            HuTao(x) => x,
            // version_1_4
            Rosaria(x) => x,
            // version_1_5
            Yanfei(x) => x,
            Eula(x) => x,
            // version_1_6
            Kazuha(x) => x,
            // version_2_0
            Ayaka(x) => x,
            Yoimiya(x) => x,
            Sayu(x) => x,
            TravelerElectro(x) => x,
            // version_2_1
            RaidenShogun(x) => x,
            KujouSara(x) => x,
            Aloy(x) => x,
            SangonomiyaKokomi(x) => x,
            // version_2_2
            Thoma(x) => x,
            // version_2_3
            AratakiItto(x) => x,
            Gorou(x) => x,
        }
    }
}

pub const N_CHARACTERS: usize = 46;

pub fn all() -> Vec<(CharacterRecord, CharacterUnion)> {
    vec![
    // pyro
    (Amber::record(), CharacterUnion::Amber(Amber::new())),
    (Bennett::record(), CharacterUnion::Bennett(Bennett::new())),
    (Xiangling::record(), CharacterUnion::Xiangling(Xiangling::new())),
    (Diluc::record(), CharacterUnion::Diluc(Diluc::new())),
    (Klee::record(), CharacterUnion::Klee(Klee::new())),
    // hydro
    (Barbara::record(), CharacterUnion::Barbara(Barbara::new())),
    (Xingqiu::record(), CharacterUnion::Xingqiu(Xingqiu::new())),
    (Mona::record(), CharacterUnion::Mona(Mona::new())),
    // electro
    (Beidou::record(), CharacterUnion::Beidou(Beidou::new())),
    (Fischl::record(), CharacterUnion::Fischl(Fischl::new())),
    (Lisa::record(), CharacterUnion::Lisa(Lisa::new())),
    (Razor::record(), CharacterUnion::Razor(Razor::new())),
    (Keqing::record(), CharacterUnion::Keqing(Keqing::new())),
    // cryo
    (Chongyun::record(), CharacterUnion::Chongyun(Chongyun::new())),
    (Kaeya::record(), CharacterUnion::Kaeya(Kaeya::new())),
    (Qiqi::record(), CharacterUnion::Qiqi(Qiqi::new())),
    // anemo
    (Sucrose::record(), CharacterUnion::Sucrose(Sucrose::new())),
    (TravelerAnemo::record(), CharacterUnion::TravelerAnemo(TravelerAnemo::new())),
    (Jean::record(), CharacterUnion::Jean(Jean::new())),
    (Venti::record(), CharacterUnion::Venti(Venti::new())),
    // geo
    (Ningguang::record(), CharacterUnion::Ningguang(Ningguang::new())),
    (Noelle::record(), CharacterUnion::Noelle(Noelle::new())),
    (TravelerGeo::record(), CharacterUnion::TravelerGeo(TravelerGeo::new())),
    // version_1_1
    (Tartaglia::record(), CharacterUnion::Tartaglia(Tartaglia::new())),
    (Diona::record(), CharacterUnion::Diona(Diona::new())),
    (Zhongli::record(), CharacterUnion::Zhongli(Zhongli::new())),
    (Xinyan::record(), CharacterUnion::Xinyan(Xinyan::new())),
    // version_1_2
    (Albedo::record(), CharacterUnion::Albedo(Albedo::new())),
    (Ganyu::record(), CharacterUnion::Ganyu(Ganyu::new())),
    // version_1_3
    (Xiao::record(), CharacterUnion::Xiao(Xiao::new())),
    (HuTao::record(), CharacterUnion::HuTao(HuTao::new())),
    // version_1_4
    (Rosaria::record(), CharacterUnion::Rosaria(Rosaria::new())),
    // version_1_5
    (Yanfei::record(), CharacterUnion::Yanfei(Yanfei::new())),
    (Eula::record(), CharacterUnion::Eula(Eula::new())),
    // version_1_6
    (Kazuha::record(), CharacterUnion::Kazuha(Kazuha::new())),
    // version_2_0
    (Ayaka::record(), CharacterUnion::Ayaka(Ayaka::new())),
    (Yoimiya::record(), CharacterUnion::Yoimiya(Yoimiya::new())),
    (Sayu::record(), CharacterUnion::Sayu(Sayu::new())),
    (TravelerElectro::record(), CharacterUnion::TravelerElectro(TravelerElectro::new())),
    // version_2_1
    (RaidenShogun::record(), CharacterUnion::RaidenShogun(RaidenShogun::new())),
    (KujouSara::record(), CharacterUnion::KujouSara(KujouSara::new())),
    (Aloy::record(), CharacterUnion::Aloy(Aloy::new())),
    (SangonomiyaKokomi::record(), CharacterUnion::SangonomiyaKokomi(SangonomiyaKokomi::new())),
    // version_2_2
    (Thoma::record(), CharacterUnion::Thoma(Thoma::new())),
    // version_2_3
    (AratakiItto::record(), CharacterUnion::AratakiItto(AratakiItto::new())),
    (Gorou::record(), CharacterUnion::Gorou(Gorou::new())),
    ]
}

/*
pub const N_CRYO: usize = 18;

pub fn cryo(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Vec<(CharacterRecord, Box<dyn CharacterAbility>)> {
    vec![
    // cryo
    (Chongyun::record(), Box::new(Chongyun::new(idx, icd_timer))),
    (Kaeya::record(), Box::new(Kaeya::new(idx, icd_timer))),
    (Qiqi::record(), Box::new(Qiqi::new(idx, icd_timer))),
    // version_1_1
    (Diona::record(), Box::new(Diona::new(idx, icd_timer))),
    // version_1_2
    (Ganyu::record(), Box::new(Ganyu::new(idx, icd_timer))),
    // version_1_4
    (Rosaria::record(), Box::new(Rosaria::new(idx, icd_timer))),
    // version_1_5
    (Eula::record(), Box::new(Eula::new(idx, icd_timer))),
    // version_2_0
    (Ayaka::record(), Box::new(Ayaka::new(idx, icd_timer))),
    // version_2_1
    (Aloy::record(), Box::new(Aloy::new(idx, icd_timer))),

    // pyro
    (Amber::record(), Box::new(Amber::new(idx, icd_timer))),
    (Bennett::record(), Box::new(Bennett::new(idx, icd_timer))),
    (Xiangling::record(), Box::new(Xiangling::new(idx, icd_timer))),
    (Diluc::record(), Box::new(Diluc::new(idx, icd_timer))),
    (Klee::record(), Box::new(Klee::new(idx, icd_timer))),
    // version_1_1
    (Xinyan::record(), Box::new(Xinyan::new(idx, icd_timer))),
    // version_1_3
    (HuTao::record(), Box::new(HuTao::new(idx, icd_timer))),
    // version_1_5
    (Yanfei::record(), Box::new(Yanfei::new(idx, icd_timer))),
    // version_2_0
    (Yoimiya::record(), Box::new(Yoimiya::new(idx, icd_timer))),
    ]
}

pub const N_ELECTRO: usize = 7;

pub fn electro(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Vec<(CharacterRecord, Box<dyn CharacterAbility>)> {
    vec![
    // electro
    (Beidou::record(), Box::new(Beidou::new(idx, icd_timer))),
    (Fischl::record(), Box::new(Fischl::new(idx, icd_timer))),
    (Lisa::record(), Box::new(Lisa::new(idx, icd_timer))),
    (Razor::record(), Box::new(Razor::new(idx, icd_timer))),
    (Keqing::record(), Box::new(Keqing::new(idx, icd_timer))),
    // version_2_1
    (RaidenShogun::record(), Box::new(RaidenShogun::new(idx, icd_timer))),
    (KujouSara::record(), Box::new(KujouSara::new(idx, icd_timer))),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::sim2::state::State;
    use crate::sim2::types::Vision;
    use crate::sim2::simulate::simulate;
    use crate::sim2::testutil::{TestEnvironment2, TestCharacter};
    use crate::sim2::fc::NoopAbility;

    use Vision::*;

    #[test]
    fn eula_0() {
        let mut members = vec![TestEnvironment::no_skill(
            Box::new(TestAbility(Eula::new())),
            Box::new(TestWeapon),
            Box::new(TestArtifact(State::new()))
        )];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        let mut current_time = 0.0;
        while current_time < 8.0 {
            total_dmg += simulate(&mut members, &mut enemy, 0.1);
            current_time += 0.1;
        }
        // 12 times na
        let expect = 0.5 * (100.0 * 12.0);
        assert_eq!(total_dmg, expect);
    }

    #[test]
    fn eula_1() {
        let mut members = vec![TestEnvironment::no_skill(
            Box::new(TestAbility(Eula::new())),
            Box::new(TestWeapon),
            Box::new(TestArtifact(State::new()))
        )];
        members[0].0.state.energy += members[0].0.state.energy_cost;
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        let mut current_time = 0.0;
        while current_time < 8.0 {
            total_dmg += simulate(&mut members, &mut enemy, 0.1);
            current_time += 0.1;
        }
        // hit 10 times nas and 1 burst before lightfall sword epires
        let expect = 0.5 * (300.0 + 100.0 * 12.0 + 2356.2);
        assert_eq!(total_dmg, expect);
    }
}
*/

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::sim2::testutil;
//     use crate::sim2::simulate;
//     use crate::sim2::types::Vision;
//     use crate::sim2::timeline::ActionState;
//     use crate::sim2::record::{CharacterData, TimelineMember, FieldMember, Enemy};
//     #[test]
//     fn debug_character() {
//         let mut history = simulate::History::<1>::new(10., 0.2);
//         let mut enemy = Enemy::simple();
//         // let mut character = Ganyu::new();
//         // let cr            = Ganyu::record();
//         // let mut weapon    = crate::sim2::weapons::bow_4star::PrototypeCrescentR5;
//         // let wr            = crate::sim2::weapons::bow_4star::PrototypeCrescentR5::record();
//         // let mut character = TravelerAnemo::new();
//         // let cr            = TravelerAnemo::record();
//         // let mut weapon    = crate::sim2::weapons::version_1_3::PrimordialJadeCutter::new();
//         // let wr            = crate::sim2::weapons::version_1_3::PrimordialJadeCutter::record();
//         // let mut character = Ningguang::new();
//         // let cr            = Ningguang::record();
//         // let mut weapon    = crate::sim2::weapons::version_1_5star::LostPrayerToTheSacredWinds::new();
//         // let wr            = crate::sim2::weapons::version_1_5star::LostPrayerToTheSacredWinds::record();
//         let mut character = Eula::new();
//         let cr            = Eula::record();
//         let mut weapon    = crate::sim2::weapons::version_1_5star::WolfsGravestone;
//         let wr            = crate::sim2::weapons::version_1_5star::WolfsGravestone::record();
//         // (some space)
//         let mut artifact  = crate::sim2::artifact::Gfelm;
//         let mut ar        = crate::sim2::artifact::Gfelm::record();
//         ar.flat_atk = 311.;
//         // ar.infuse_goblet(&cr.vision);
//         ar.infuse_goblet(&Vision::Physical);
//         let mut states = [ActionState::new(); 1];
//         let mut data = [CharacterData::new(0, &cr, &wr, &ar); 1];
//         let dmg: f32;
//         {
//             let mut members = [TimelineMember {
//                 character: &mut character,
//                 weapon: &mut weapon,
//                 artifact: &mut artifact,
//             }; 1];
//             states[0].energy += cr.energy_cost;
//             simulate::decide_action(&mut history, &mut members, &mut states, &mut data);
//         }
//         {
//             let mut members = [FieldMember {
//                 character: &mut character,
//                 weapon: &mut weapon,
//                 artifact: &mut artifact,
//             }; 1];
//             dmg = simulate::calculate_damage(&mut history, &mut members, &mut data, &mut enemy);
//         }
//         println!("{:?}", history.action);
//         println!("{:?}", dmg);
//         assert!(false);
//     }
// }
