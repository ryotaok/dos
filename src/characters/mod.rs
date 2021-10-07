use crate::fc::{FieldCharacterIndex, CharacterRecord, SpecialAbility, CharacterAbility};
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

pub const N_CHARACTERS: usize = 42;

pub fn all(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Vec<(CharacterRecord, Box<dyn CharacterAbility>)> {
    vec![
    // pyro
    (Amber::record(), Box::new(Amber::new(idx, icd_timer))),
    (Bennett::record(), Box::new(Bennett::new(idx, icd_timer))),
    (Xiangling::record(), Box::new(Xiangling::new(idx, icd_timer))),
    (Diluc::record(), Box::new(Diluc::new(idx, icd_timer))),
    (Klee::record(), Box::new(Klee::new(idx, icd_timer))),
    // hydro
    (Barbara::record(), Box::new(Barbara::new(idx, icd_timer))),
    (Xingqiu::record(), Box::new(Xingqiu::new(idx, icd_timer))),
    (Mona::record(), Box::new(Mona::new(idx, icd_timer))),
    // electro
    (Beidou::record(), Box::new(Beidou::new(idx, icd_timer))),
    (Fischl::record(), Box::new(Fischl::new(idx, icd_timer))),
    (Lisa::record(), Box::new(Lisa::new(idx, icd_timer))),
    (Razor::record(), Box::new(Razor::new(idx, icd_timer))),
    (Keqing::record(), Box::new(Keqing::new(idx, icd_timer))),
    // cryo
    (Chongyun::record(), Box::new(Chongyun::new(idx, icd_timer))),
    (Kaeya::record(), Box::new(Kaeya::new(idx, icd_timer))),
    (Qiqi::record(), Box::new(Qiqi::new(idx, icd_timer))),
    // anemo
    (Sucrose::record(), Box::new(Sucrose::new(idx, icd_timer))),
    (TravelerAnemo::record(), Box::new(TravelerAnemo::new(idx, icd_timer))),
    (Jean::record(), Box::new(Jean::new(idx, icd_timer))),
    (Venti::record(), Box::new(Venti::new(idx, icd_timer))),
    // geo
    (Ningguang::record(), Box::new(Ningguang::new(idx, icd_timer))),
    (Noelle::record(), Box::new(Noelle::new(idx, icd_timer))),
    (TravelerGeo::record(), Box::new(TravelerGeo::new(idx, icd_timer))),
    // version_1_1
    (Tartaglia::record(), Box::new(Tartaglia::new(idx, icd_timer))),
    (Diona::record(), Box::new(Diona::new(idx, icd_timer))),
    (Zhongli::record(), Box::new(Zhongli::new(idx, icd_timer))),
    (Xinyan::record(), Box::new(Xinyan::new(idx, icd_timer))),
    // version_1_2
    (Albedo::record(), Box::new(Albedo::new(idx, icd_timer))),
    (Ganyu::record(), Box::new(Ganyu::new(idx, icd_timer))),
    // version_1_3
    (Xiao::record(), Box::new(Xiao::new(idx, icd_timer))),
    (HuTao::record(), Box::new(HuTao::new(idx, icd_timer))),
    // version_1_4
    (Rosaria::record(), Box::new(Rosaria::new(idx, icd_timer))),
    // version_1_5
    (Yanfei::record(), Box::new(Yanfei::new(idx, icd_timer))),
    (Eula::record(), Box::new(Eula::new(idx, icd_timer))),
    // version_1_6
    (Kazuha::record(), Box::new(Kazuha::new(idx, icd_timer))),
    // version_2_0
    (Ayaka::record(), Box::new(Ayaka::new(idx, icd_timer))),
    (Yoimiya::record(), Box::new(Yoimiya::new(idx, icd_timer))),
    (Sayu::record(), Box::new(Sayu::new(idx, icd_timer))),
    // version_2_1
    (RaidenShogun::record(), Box::new(RaidenShogun::new(idx, icd_timer))),
    (KujouSara::record(), Box::new(KujouSara::new(idx, icd_timer))),
    (Aloy::record(), Box::new(Aloy::new(idx, icd_timer))),
    (SangonomiyaKokomi::record(), Box::new(SangonomiyaKokomi::new(idx, icd_timer))),
    ]
}

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

    use crate::state::State;
    use crate::types::Vision;
    use crate::simulate::simulate;
    use crate::testutil::{TestEnvironment2, TestCharacter};
    use crate::fc::NoopAbility;

    use Vision::*;

    // #[test]
    // fn eula_0() {
    //     let mut members = vec![TestEnvironment::no_skill(
    //         Box::new(TestAbility(Eula::new())),
    //         Box::new(TestWeapon),
    //         Box::new(TestArtifact(State::new()))
    //     )];
    //     let mut enemy = TestEnvironment::enemy();
    //     let mut total_dmg = 0.0;
    //     let mut current_time = 0.0;
    //     while current_time < 8.0 {
    //         total_dmg += simulate(&mut members, &mut enemy, 0.1);
    //         current_time += 0.1;
    //     }
    //     // 12 times na
    //     let expect = 0.5 * (100.0 * 12.0);
    //     assert_eq!(total_dmg, expect);
    // }

    // #[test]
    // fn eula_1() {
    //     let mut members = vec![TestEnvironment::no_skill(
    //         Box::new(TestAbility(Eula::new())),
    //         Box::new(TestWeapon),
    //         Box::new(TestArtifact(State::new()))
    //     )];
    //     members[0].0.state.energy += members[0].0.state.energy_cost;
    //     let mut enemy = TestEnvironment::enemy();
    //     let mut total_dmg = 0.0;
    //     let mut current_time = 0.0;
    //     while current_time < 8.0 {
    //         total_dmg += simulate(&mut members, &mut enemy, 0.1);
    //         current_time += 0.1;
    //     }
    //     // hit 10 times nas and 1 burst before lightfall sword epires
    //     let expect = 0.5 * (300.0 + 100.0 * 12.0 + 2356.2);
    //     assert_eq!(total_dmg, expect);
    // }
}
