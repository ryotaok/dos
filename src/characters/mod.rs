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

use crate::fc::{SpecialAbility, CharacterRecord};
use crate::types::Vision;
use crate::cli::Args;
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

pub fn all() -> Vec<Box<dyn SpecialAbility>> {
    vec![
// pyro
Box::new(Amber::new()),
Box::new(Bennett::new()),
Box::new(Xiangling::new()),
Box::new(Diluc::new()),
Box::new(Klee::new()),
// hydro
Box::new(Barbara),
Box::new(Xingqiu::new()),
Box::new(Mona::new()),
// electro
Box::new(Beidou::new()),
Box::new(Fischl::new()),
Box::new(Lisa::new()),
Box::new(Razor::new()),
Box::new(Keqing::new()),
// cryo
Box::new(Chongyun::new()),
Box::new(Kaeya::new()),
Box::new(Qiqi::new()),
// anemo
Box::new(Sucrose::new()),
Box::new(TravelerAnemo::new()),
Box::new(Jean::new()),
Box::new(Venti::new()),
// geo
Box::new(Ningguang::new()),
Box::new(Noelle::new()),
Box::new(TravelerGeo::new()),
// version_1_1
Box::new(Tartaglia::new()),
Box::new(Diona::new()),
Box::new(Zhongli::new()),
Box::new(Xinyan::new()),
// version_1_2
Box::new(Albedo::new()),
Box::new(Ganyu::new()),
// version_1_3
Box::new(Xiao::new()),
Box::new(HuTao::new()),
// version_1_4
Box::new(Rosaria::new()),
// version_1_5
Box::new(Yanfei::new()),
Box::new(Eula::new()),
// version_1_6
Box::new(Kazuha::new()),
// version_2_0
Box::new(Ayaka::new()),
Box::new(Yoimiya::new()),
Box::new(Sayu::new()),
    ]
}

pub fn setup(args: &Args) -> Vec<(CharacterRecord, Vision, Box<dyn SpecialAbility>)> {
    let all = all();
    let mut result: Vec<(CharacterRecord, Vision, Box<dyn SpecialAbility>)> = Vec::with_capacity(all.len());
    for ca in all {
        let r = ca.character();
        if r.version <= args.character_version {
            let vision = Vision::from(&r.vision);
            result.push((r, vision, ca));
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::state::State;
    use crate::simulate::simulate;
    use crate::testutil::{TestEnvironment, TestWeapon, TestArtifact, TestAbility};

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
        members[0].0.state.energy.0 += members[0].0.state.energy_cost;
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
