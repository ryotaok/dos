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

use crate::fc::{FieldCharacterIndex, CharacterRecord, CharacterAbility};
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

fn field<T: CharacterAbility>(ca: T) -> (CharacterRecord, T) {
    let a = ca.record();
    (a, ca)
}

pub struct AllCharacters {
    // pyro
    amber: (CharacterRecord, Amber),
    bennett: (CharacterRecord, Bennett),
    xiangling: (CharacterRecord, Xiangling),
    diluc: (CharacterRecord, Diluc),
    klee: (CharacterRecord, Klee),
    // hydro
    barbara: (CharacterRecord, Barbara),
    xingqiu: (CharacterRecord, Xingqiu),
    mona: (CharacterRecord, Mona),
    // electro
    beidou: (CharacterRecord, Beidou),
    fischl: (CharacterRecord, Fischl),
    lisa: (CharacterRecord, Lisa),
    razor: (CharacterRecord, Razor),
    keqing: (CharacterRecord, Keqing),
    // cryo
    chongyun: (CharacterRecord, Chongyun),
    kaeya: (CharacterRecord, Kaeya),
    qiqi: (CharacterRecord, Qiqi),
    // anemo
    sucrose: (CharacterRecord, Sucrose),
    traveleranemo: (CharacterRecord, TravelerAnemo),
    jean: (CharacterRecord, Jean),
    venti: (CharacterRecord, Venti),
    // geo
    ningguang: (CharacterRecord, Ningguang),
    noelle: (CharacterRecord, Noelle),
    travelergeo: (CharacterRecord, TravelerGeo),
    // version_1_1
    tartaglia: (CharacterRecord, Tartaglia),
    diona: (CharacterRecord, Diona),
    zhongli: (CharacterRecord, Zhongli),
    xinyan: (CharacterRecord, Xinyan),
    // version_1_2
    albedo: (CharacterRecord, Albedo),
    ganyu: (CharacterRecord, Ganyu),
    // version_1_3
    xiao: (CharacterRecord, Xiao),
    hutao: (CharacterRecord, HuTao),
    // version_1_4
    rosaria: (CharacterRecord, Rosaria),
    // version_1_5
    yanfei: (CharacterRecord, Yanfei),
    eula: (CharacterRecord, Eula),
    // version_1_6
    kazuha: (CharacterRecord, Kazuha),
    // version_2_0
    ayaka: (CharacterRecord, Ayaka),
    yoimiya: (CharacterRecord, Yoimiya),
    sayu: (CharacterRecord, Sayu),
    // version_2_1
    raidenshogun: (CharacterRecord, RaidenShogun),
    sangonomiyakokomi: (CharacterRecord, SangonomiyaKokomi),
    sangonomiyakokomihp: (CharacterRecord, SangonomiyaKokomiHp),
    kujousara: (CharacterRecord, KujouSara),
    aloy: (CharacterRecord, Aloy),
}

impl AllCharacters {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            // pyro
            amber: field(Amber::new(idx)),
            bennett: field(Bennett::new(idx)),
            xiangling: field(Xiangling::new(idx)),
            diluc: field(Diluc::new(idx)),
            klee: field(Klee::new(idx)),
            // hydro
            barbara: field(Barbara::new(idx)),
            xingqiu: field(Xingqiu::new(idx)),
            mona: field(Mona::new(idx)),
            // electro
            beidou: field(Beidou::new(idx)),
            fischl: field(Fischl::new(idx)),
            lisa: field(Lisa::new(idx)),
            razor: field(Razor::new(idx)),
            keqing: field(Keqing::new(idx)),
            // cryo
            chongyun: field(Chongyun::new(idx)),
            kaeya: field(Kaeya::new(idx)),
            qiqi: field(Qiqi::new(idx)),
            // anemo
            sucrose: field(Sucrose::new(idx)),
            traveleranemo: field(TravelerAnemo::new(idx)),
            jean: field(Jean::new(idx)),
            venti: field(Venti::new(idx)),
            // geo
            ningguang: field(Ningguang::new(idx)),
            noelle: field(Noelle::new(idx)),
            travelergeo: field(TravelerGeo::new(idx)),
            // version_1_1
            tartaglia: field(Tartaglia::new(idx)),
            diona: field(Diona::new(idx)),
            zhongli: field(Zhongli::new(idx)),
            xinyan: field(Xinyan::new(idx)),
            // version_1_2
            albedo: field(Albedo::new(idx)),
            ganyu: field(Ganyu::new(idx)),
            // version_1_3
            xiao: field(Xiao::new(idx)),
            hutao: field(HuTao::new(idx)),
            // version_1_4
            rosaria: field(Rosaria::new(idx)),
            // version_1_5
            yanfei: field(Yanfei::new(idx)),
            eula: field(Eula::new(idx)),
            // version_1_6
            kazuha: field(Kazuha::new(idx)),
            // version_2_0
            ayaka: field(Ayaka::new(idx)),
            yoimiya: field(Yoimiya::new(idx)),
            sayu: field(Sayu::new(idx)),
            // version_2_1
            raidenshogun: field(RaidenShogun::new(idx)),
            sangonomiyakokomi: field(SangonomiyaKokomi::new(idx)),
            sangonomiyakokomihp: field(SangonomiyaKokomiHp::new(idx)),
            kujousara: field(KujouSara::new(idx)),
            aloy: field(Aloy::new(idx)),
        }
    }

    // #![feature(unsized_tuple_coercion)]
    pub fn find<'a>(&'a mut self, name: &CharacterName) -> &'a mut (CharacterRecord, dyn CharacterAbility) {
        use CharacterName::*;
        match name {
            // pyro
            Amber => &mut self.amber,
            Bennett => &mut self.bennett,
            Xiangling => &mut self.xiangling,
            Diluc => &mut self.diluc,
            Klee => &mut self.klee,
            // hydro
            Barbara => &mut self.barbara,
            Xingqiu => &mut self.xingqiu,
            Mona => &mut self.mona,
            // electro
            Beidou => &mut self.beidou,
            Fischl => &mut self.fischl,
            Lisa => &mut self.lisa,
            Razor => &mut self.razor,
            Keqing => &mut self.keqing,
            // cryo
            Chongyun => &mut self.chongyun,
            Kaeya => &mut self.kaeya,
            Qiqi => &mut self.qiqi,
            // anemo
            Sucrose => &mut self.sucrose,
            TravelerAnemo => &mut self.traveleranemo,
            Jean => &mut self.jean,
            Venti => &mut self.venti,
            // geo
            Ningguang => &mut self.ningguang,
            Noelle => &mut self.noelle,
            TravelerGeo => &mut self.travelergeo,
            // version_1_1
            Tartaglia => &mut self.tartaglia,
            Diona => &mut self.diona,
            Zhongli => &mut self.zhongli,
            Xinyan => &mut self.xinyan,
            // version_1_2
            Albedo => &mut self.albedo,
            Ganyu => &mut self.ganyu,
            // version_1_3
            Xiao => &mut self.xiao,
            HuTao => &mut self.hutao,
            // version_1_4
            Rosaria => &mut self.rosaria,
            // version_1_5
            Yanfei => &mut self.yanfei,
            Eula => &mut self.eula,
            // version_1_6
            Kazuha => &mut self.kazuha,
            // version_2_0
            Ayaka => &mut self.ayaka,
            Yoimiya => &mut self.yoimiya,
            Sayu => &mut self.sayu,
            // version_2_1
            RaidenShogun => &mut self.raidenshogun,
            SangonomiyaKokomi => &mut self.sangonomiyakokomi,
            SangonomiyaKokomiHp => &mut self.sangonomiyakokomihp,
            KujouSara => &mut self.kujousara,
            Aloy => &mut self.aloy,
        }
    }
}

#[derive(Debug)]
pub enum CharacterName {
    // pyro
    Amber,
    Bennett,
    Xiangling,
    Diluc,
    Klee,
    // hydro
    Barbara,
    Xingqiu,
    Mona,
    // electro
    Beidou,
    Fischl,
    Lisa,
    Razor,
    Keqing,
    // cryo
    Chongyun,
    Kaeya,
    Qiqi,
    // anemo
    Sucrose,
    TravelerAnemo,
    Jean,
    Venti,
    // geo
    Ningguang,
    Noelle,
    TravelerGeo,
    // version_1_1
    Tartaglia,
    Diona,
    Zhongli,
    Xinyan,
    // version_1_2
    Albedo,
    Ganyu,
    // version_1_3
    Xiao,
    HuTao,
    // version_1_4
    Rosaria,
    // version_1_5
    Yanfei,
    Eula,
    // version_1_6
    Kazuha,
    // version_2_0
    Ayaka,
    Yoimiya,
    Sayu,
    // version_2_1
    RaidenShogun,
    SangonomiyaKokomi,
    SangonomiyaKokomiHp,
    KujouSara,
    Aloy,
}

impl CharacterName {
    pub fn vec() -> Vec<CharacterName> {
        use CharacterName::*;
        vec![
    // pyro
    Amber,
    Bennett,
    Xiangling,
    Diluc,
    Klee,
    // hydro
    Barbara,
    Xingqiu,
    Mona,
    // electro
    Beidou,
    Fischl,
    Lisa,
    Razor,
    Keqing,
    // cryo
    Chongyun,
    Kaeya,
    Qiqi,
    // anemo
    Sucrose,
    TravelerAnemo,
    Jean,
    Venti,
    // geo
    Ningguang,
    Noelle,
    TravelerGeo,
    // version_1_1
    Tartaglia,
    Diona,
    Zhongli,
    Xinyan,
    // version_1_2
    Albedo,
    Ganyu,
    // version_1_3
    Xiao,
    HuTao,
    // version_1_4
    Rosaria,
    // version_1_5
    Yanfei,
    Eula,
    // version_1_6
    Kazuha,
    // version_2_0
    Ayaka,
    Yoimiya,
    Sayu,
    // version_2_1
    RaidenShogun,
    SangonomiyaKokomi,
    SangonomiyaKokomiHp,
    KujouSara,
    Aloy,
        ]
    }

    pub fn cryo() -> Vec<CharacterName> {
        use CharacterName::*;
        vec![
    // cryo
    Chongyun,
    Kaeya,
    Qiqi,
    // version_1_1
    Diona,
    // version_1_2
    Ganyu,
    // version_1_4
    Rosaria,
    // version_1_5
    Eula,
    // version_2_0
    Ayaka,
    // version_2_1
    Aloy,
        ]
    }

    pub fn electro() -> Vec<CharacterName> {
        use CharacterName::*;
        vec![
    // electro
    // Beidou,
    Fischl,
    Lisa,
    // Razor,
    Keqing,
    // version_2_1
    RaidenShogun,
    KujouSara,
        ]
    }
}

impl<'a> From<&'a str> for CharacterName {
    fn from(name: &'a str) -> Self {
        use CharacterName::*;
        match name {
            // pyro
            "Amber" => Amber,
            "Bennett" => Bennett,
            "Xiangling" => Xiangling,
            "Diluc" => Diluc,
            "Klee" => Klee,
            // hydro
            "Barbara" => Barbara,
            "Xingqiu" => Xingqiu,
            "Mona" => Mona,
            // electro
            "Beidou" => Beidou,
            "Fischl" => Fischl,
            "Lisa" => Lisa,
            "Razor" => Razor,
            "Keqing" => Keqing,
            // cryo
            "Chongyun" => Chongyun,
            "Kaeya" => Kaeya,
            "Qiqi" => Qiqi,
            // anemo
            "Sucrose" => Sucrose,
            "TravelerAnemo" => TravelerAnemo,
            "Jean" => Jean,
            "Venti" => Venti,
            // geo
            "Ningguang" => Ningguang,
            "Noelle" => Noelle,
            "TravelerGeo" => TravelerGeo,
            // version_1_1
            "Tartaglia" => Tartaglia,
            "Diona" => Diona,
            "Zhongli" => Zhongli,
            "Xinyan" => Xinyan,
            // version_1_2
            "Albedo" => Albedo,
            "Ganyu" => Ganyu,
            // version_1_3
            "Xiao" => Xiao,
            "HuTao" => HuTao,
            // version_1_4
            "Rosaria" => Rosaria,
            // version_1_5
            "Yanfei" => Yanfei,
            "Eula" => Eula,
            // version_1_6
            "Kazuha" => Kazuha,
            // version_2_0
            "Ayaka" => Ayaka,
            "Yoimiya" => Yoimiya,
            "Sayu" => Sayu,
            // version_2_1
            "RaidenShogun" => RaidenShogun,
            "SangonomiyaKokomi" => SangonomiyaKokomi,
            "SangonomiyaKokomiHp" => SangonomiyaKokomiHp,
            "KujouSara" => KujouSara,
            "Aloy" => Aloy,
            _ => unimplemented!(),
        }
    }
}

// pub fn setup(args: &Args) -> Vec<(CharacterRecord, Vision, Box<dyn SpecialAbility>)> {
//     let all = all();
//     let mut result: Vec<(CharacterRecord, Vision, Box<dyn SpecialAbility>)> = Vec::with_capacity(all.len());
//     for ca in all {
//         let r = ca.character();
//         if r.version <= args.character_version {
//             let vision = Vision::from(&r.vision);
//             result.push((r, vision, ca));
//         }
//     }
//     result
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     use crate::state::State;
//     use crate::simulate::simulate;
//     use crate::testutil::{TestEnvironment, TestWeapon, TestArtifact, TestAbility};

//     #[test]
//     fn eula_0() {
//         let mut members = vec![TestEnvironment::no_skill(
//             Box::new(TestAbility(Eula::new())),
//             Box::new(TestWeapon),
//             Box::new(TestArtifact(State::new()))
//         )];
//         let mut enemy = TestEnvironment::enemy();
//         let mut total_dmg = 0.0;
//         let mut current_time = 0.0;
//         while current_time < 8.0 {
//             total_dmg += simulate(&mut members, &mut enemy, 0.1);
//             current_time += 0.1;
//         }
//         // 12 times na
//         let expect = 0.5 * (100.0 * 12.0);
//         assert_eq!(total_dmg, expect);
//     }

//     #[test]
//     fn eula_1() {
//         let mut members = vec![TestEnvironment::no_skill(
//             Box::new(TestAbility(Eula::new())),
//             Box::new(TestWeapon),
//             Box::new(TestArtifact(State::new()))
//         )];
//         members[0].0.state.energy += members[0].0.state.energy_cost;
//         let mut enemy = TestEnvironment::enemy();
//         let mut total_dmg = 0.0;
//         let mut current_time = 0.0;
//         while current_time < 8.0 {
//             total_dmg += simulate(&mut members, &mut enemy, 0.1);
//             current_time += 0.1;
//         }
//         // hit 10 times nas and 1 burst before lightfall sword epires
//         let expect = 0.5 * (300.0 + 100.0 * 12.0 + 2356.2);
//         assert_eq!(total_dmg, expect);
//     }
// }
