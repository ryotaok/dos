use std::rc::Rc;
use std::cell::RefCell;

use crate::fc::{FieldCharacterIndex, CharacterRecord, SpecialAbility, FieldAbilityBuilder};
use crate::action::{ICDTimers};

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
    kujousara: (CharacterRecord, KujouSara),
    aloy: (CharacterRecord, Aloy),
}

impl AllCharacters {
    pub fn new(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Self {
        Self {
            // pyro
            amber: (Amber::record(), Amber::new(idx, icd_timer)),
            bennett: (Bennett::record(), Bennett::new(idx, icd_timer)),
            xiangling: (Xiangling::record(), Xiangling::new(idx, icd_timer)),
            diluc: (Diluc::record(), Diluc::new(idx, icd_timer)),
            klee: (Klee::record(), Klee::new(idx, icd_timer)),
            // hydro
            barbara: (Barbara::record(), Barbara::new(idx, icd_timer)),
            xingqiu: (Xingqiu::record(), Xingqiu::new(idx, icd_timer)),
            mona: (Mona::record(), Mona::new(idx, icd_timer)),
            // electro
            beidou: (Beidou::record(), Beidou::new(idx, icd_timer)),
            fischl: (Fischl::record(), Fischl::new(idx, icd_timer)),
            lisa: (Lisa::record(), Lisa::new(idx, icd_timer)),
            razor: (Razor::record(), Razor::new(idx, icd_timer)),
            keqing: (Keqing::record(), Keqing::new(idx, icd_timer)),
            // cryo
            chongyun: (Chongyun::record(), Chongyun::new(idx, icd_timer)),
            kaeya: (Kaeya::record(), Kaeya::new(idx, icd_timer)),
            qiqi: (Qiqi::record(), Qiqi::new(idx, icd_timer)),
            // anemo
            sucrose: (Sucrose::record(), Sucrose::new(idx, icd_timer)),
            traveleranemo: (TravelerAnemo::record(), TravelerAnemo::new(idx, icd_timer)),
            jean: (Jean::record(), Jean::new(idx, icd_timer)),
            venti: (Venti::record(), Venti::new(idx, icd_timer)),
            // geo
            ningguang: (Ningguang::record(), Ningguang::new(idx, icd_timer)),
            noelle: (Noelle::record(), Noelle::new(idx, icd_timer)),
            travelergeo: (TravelerGeo::record(), TravelerGeo::new(idx, icd_timer)),
            // version_1_1
            tartaglia: (Tartaglia::record(), Tartaglia::new(idx, icd_timer)),
            diona: (Diona::record(), Diona::new(idx, icd_timer)),
            zhongli: (Zhongli::record(), Zhongli::new(idx, icd_timer)),
            xinyan: (Xinyan::record(), Xinyan::new(idx, icd_timer)),
            // version_1_2
            albedo: (Albedo::record(), Albedo::new(idx, icd_timer)),
            ganyu: (Ganyu::record(), Ganyu::new(idx, icd_timer)),
            // version_1_3
            xiao: (Xiao::record(), Xiao::new(idx, icd_timer)),
            hutao: (HuTao::record(), HuTao::new(idx, icd_timer)),
            // version_1_4
            rosaria: (Rosaria::record(), Rosaria::new(idx, icd_timer)),
            // version_1_5
            yanfei: (Yanfei::record(), Yanfei::new(idx, icd_timer)),
            eula: (Eula::record(), Eula::new(idx, icd_timer)),
            // version_1_6
            kazuha: (Kazuha::record(), Kazuha::new(idx, icd_timer)),
            // version_2_0
            ayaka: (Ayaka::record(), Ayaka::new(idx, icd_timer)),
            yoimiya: (Yoimiya::record(), Yoimiya::new(idx, icd_timer)),
            sayu: (Sayu::record(), Sayu::new(idx, icd_timer)),
            // version_2_1
            raidenshogun: (RaidenShogun::record(), RaidenShogun::new(idx, icd_timer)),
            sangonomiyakokomi: (SangonomiyaKokomi::record(), SangonomiyaKokomi::new(idx, icd_timer)),
            kujousara: (KujouSara::record(), KujouSara::new(idx, icd_timer)),
            aloy: (Aloy::record(), Aloy::new(idx, icd_timer)),
        }
    }

    // #![feature(unsized_tuple_coercion)]
    pub fn find<'a>(&'a mut self, name: &CharacterName, builder: &mut FieldAbilityBuilder) -> &'a mut (CharacterRecord, dyn SpecialAbility + 'a) {
        use CharacterName::*;
        match name {
            // pyro
            Amber => { self.amber.1.build(builder); &mut self.amber },
            Bennett => { self.bennett.1.build(builder); &mut self.bennett },
            Xiangling => { self.xiangling.1.build(builder); &mut self.xiangling },
            Diluc => { self.diluc.1.build(builder); &mut self.diluc },
            Klee => { self.klee.1.build(builder); &mut self.klee },
            // hydro
            Barbara => { self.barbara.1.build(builder); &mut self.barbara },
            Xingqiu => { self.xingqiu.1.build(builder); &mut self.xingqiu },
            Mona => { self.mona.1.build(builder); &mut self.mona },
            // electro
            Beidou => { self.beidou.1.build(builder); &mut self.beidou },
            Fischl => { self.fischl.1.build(builder); &mut self.fischl },
            Lisa => { self.lisa.1.build(builder); &mut self.lisa },
            Razor => { self.razor.1.build(builder); &mut self.razor },
            Keqing => { self.keqing.1.build(builder); &mut self.keqing },
            // cryo
            Chongyun => { self.chongyun.1.build(builder); &mut self.chongyun },
            Kaeya => { self.kaeya.1.build(builder); &mut self.kaeya },
            Qiqi => { self.qiqi.1.build(builder); &mut self.qiqi },
            // anemo
            Sucrose => { self.sucrose.1.build(builder); &mut self.sucrose },
            TravelerAnemo => { self.traveleranemo.1.build(builder); &mut self.traveleranemo },
            Jean => { self.jean.1.build(builder); &mut self.jean },
            Venti => { self.venti.1.build(builder); &mut self.venti },
            // geo
            Ningguang => { self.ningguang.1.build(builder); &mut self.ningguang },
            Noelle => { self.noelle.1.build(builder); &mut self.noelle },
            TravelerGeo => { self.travelergeo.1.build(builder); &mut self.travelergeo },
            // version_1_1
            Tartaglia => { self.tartaglia.1.build(builder); &mut self.tartaglia },
            Diona => { self.diona.1.build(builder); &mut self.diona },
            Zhongli => { self.zhongli.1.build(builder); &mut self.zhongli },
            Xinyan => { self.xinyan.1.build(builder); &mut self.xinyan },
            // version_1_2
            Albedo => { self.albedo.1.build(builder); &mut self.albedo },
            Ganyu => { self.ganyu.1.build(builder); &mut self.ganyu },
            // version_1_3
            Xiao => { self.xiao.1.build(builder); &mut self.xiao },
            HuTao => { self.hutao.1.build(builder); &mut self.hutao },
            // version_1_4
            Rosaria => { self.rosaria.1.build(builder); &mut self.rosaria },
            // version_1_5
            Yanfei => { self.yanfei.1.build(builder); &mut self.yanfei },
            Eula => { self.eula.1.build(builder); &mut self.eula },
            // version_1_6
            Kazuha => { self.kazuha.1.build(builder); &mut self.kazuha },
            // version_2_0
            Ayaka => { self.ayaka.1.build(builder); &mut self.ayaka },
            Yoimiya => { self.yoimiya.1.build(builder); &mut self.yoimiya },
            Sayu => { self.sayu.1.build(builder); &mut self.sayu },
            // version_2_1
            RaidenShogun => { self.raidenshogun.1.build(builder); &mut self.raidenshogun },
            SangonomiyaKokomi => { self.sangonomiyakokomi.1.build(builder); &mut self.sangonomiyakokomi },
            KujouSara => { self.kujousara.1.build(builder); &mut self.kujousara },
            Aloy => { self.aloy.1.build(builder); &mut self.aloy },
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
            "KujouSara" => KujouSara,
            "Aloy" => Aloy,
            _ => unimplemented!(),
        }
    }
}

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
