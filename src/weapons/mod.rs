pub mod sword_4star;
pub mod claymore_4star;
pub mod polearm_4star;
pub mod bow_4star;
pub mod catalyst_4star;
pub mod favonius_series;
pub mod sacrificial_series;
pub mod version_1_5star;
pub mod version_1_1;
pub mod version_1_2;
pub mod version_1_3;
pub mod version_1_4;
pub mod version_1_5;
pub mod version_1_6;
pub mod version_2_0;

use crate::cli::Args;
use crate::fc::{SpecialAbility, WeaponRecord};
use sword_4star::*;
use claymore_4star::*;
use polearm_4star::*;
use bow_4star::*;
use catalyst_4star::*;
use favonius_series::*;
use sacrificial_series::*;
use version_1_5star::*;
use version_1_1::*;
use version_1_2::*;
use version_1_3::*;
use version_1_4::*;
use version_1_5::*;
use version_1_6::*;
use version_2_0::*;

pub fn all() -> Vec<Box<dyn SpecialAbility>> {
    vec![
// sword_4star
Box::new(PrototypeRancourR5::new()),
Box::new(TheBlackSwordR5),
Box::new(BlackcliffLongswordR5),
Box::new(RoyalLongswordR5),
Box::new(HarbingerOfDawnR5),
Box::new(TheFluteR5::new()),
Box::new(LionsRoarR5),
// claymore_4star
Box::new(PrototypeArchaicR5::new()),
Box::new(WhiteblindR5::new()),
Box::new(SerpentSpineR5::new()),
Box::new(BlackcliffSlasherR5),
Box::new(RoyalGreatswordR5),
Box::new(RainslasherR5),
// polearm_4star
Box::new(PrototypeStarglitterR5::new()),
Box::new(CrescentPikeR5::new()),
Box::new(DeathmatchR5),
Box::new(BlackcliffPoleR5),
Box::new(RoyalSpearR5),
Box::new(WhiteTasselR5),
Box::new(DragonsBaneR5),
// bow_4star
Box::new(PrototypeCrescentR5),
Box::new(CompoundBowR5::new()),
Box::new(TheViridescentHuntR5::new()),
Box::new(BlackcliffWarbowR5),
Box::new(RoyalBowR5),
Box::new(SlingshotR5),
Box::new(RustR5),
Box::new(TheStringlessR5),
// catalyst_4star
Box::new(PrototypeAmberR5),
Box::new(MappaMareR5::new()),
Box::new(SolarPearlR5::new()),
Box::new(BlackcliffAgateR5),
Box::new(RoyalGrimoireR5),
Box::new(ThrillingTalesOfDragonSlayersR5::new()),
Box::new(EyeOfPerceptionR5::new()),
Box::new(TheWidsithR5::new()),
// favonius_series 1 1 1 1 1
Box::new(FavoniusGreatswordR5::new()),
Box::new(FavoniusSwordR5::new()),
Box::new(FavoniusLanceR5::new()),
Box::new(FavoniusWarbowR5::new()),
Box::new(FavoniusCodexR5::new()),
// sacrificial_series 1 1 0 1 1
Box::new(SacrificialSwordR5::new()),
Box::new(SacrificialGreatswordR5::new()),
Box::new(SacrificialBowR5::new()),
Box::new(SacrificialFragmentsR5::new()),
// version_1_5star 2 2 2 2 2
Box::new(SkywardBlade::new()),
Box::new(AquilaFavonia::new()),
Box::new(SkywardPride::new()),
Box::new(WolfsGravestone),
Box::new(SkywardSpine::new()),
Box::new(PrimordialJadeWingedSpear::new()),
Box::new(SkywardHarp::new()),
Box::new(AmosBow),
Box::new(SkywardAtlas::new()),
Box::new(LostPrayerToTheSacredWinds::new()),
// version_1_1 1 1 1 0 1 
Box::new(TheUnforged::new()),
Box::new(SummitShaper::new()),
Box::new(VortexVanquisher::new()),
Box::new(MemoryOfDust::new()),
// // version_1_2 1 1 1 0 1
Box::new(FesteringDesire),
Box::new(SnowTombedStarsilver::new()),
Box::new(DragonspineSpear::new()),
Box::new(Frostbearer::new()),
// // version_1_3 1 1 2 0 0
Box::new(PrimordialJadeCutter::new()),
Box::new(PrimordialJadeGS::new()),
Box::new(PrimordialJadeVista::new()),
Box::new(StaffOfHoma::new()),
Box::new(LithicSpear),
Box::new(LithicBlade),
// // version_1_4 1 0 0 3 1
Box::new(ElegyForTheEnd::new()),
Box::new(TheAlleyFlash),
Box::new(AlleyHunter::new()),
Box::new(WineAndSong),
Box::new(WindblumeOde::new()),
// // version_1_5 0 1 0 0 0
Box::new(SongOfBrokenPines::new()),
// // version_1_6 1 0 0 1 1
Box::new(FreedomSworn::new()),
Box::new(MitternachtsWaltz::new()),
Box::new(DodocoTales::new()),
// // version_2_0 1 0 0 1 0
Box::new(MistsplittersReforged::new()),
Box::new(ThunderingPulse::new()),
    ]
}

pub fn setup(args: &Args) -> Vec<(WeaponRecord, Box<dyn SpecialAbility>)> {
    let all = all();
    let mut result: Vec<(WeaponRecord, Box<dyn SpecialAbility>)> = Vec::with_capacity(all.len());
    for wa in all {
        let r = wa.weapon();
        if r.version <= args.weapon_version {
            result.push((r, wa));
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::state::State;
    use crate::simulate::simulate;
    use crate::fc::{FieldCharacterIndex, FieldAbility};
    use crate::testutil::{TestEnvironment, TestCharacter, TestArtifact, TestAbility};

    // Note that TestAbility disables each weapon's main stats

    #[test]
    fn prototype_rancour() {
        let mut members = vec![
            FieldAbility::boxed(
                TestCharacter::new(),
                TestAbility(PrototypeRancourR5::new()),
                TestArtifact(State::new()),
            ).to_data(FieldCharacterIndex(0))
        ];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // skill na na na
        let expect = 0.5 * (200.0 + 108.0 + 116.0 + 124.0);
        assert_eq!(total_dmg, expect);
    }

    #[test]
    fn the_flute() {
        let mut members = vec![TestEnvironment::no_skill(
            Box::new(TestCharacter::new()),
            Box::new(TestAbility(TheFluteR5::new())),
            Box::new(TestArtifact(State::new()))
        )];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..40 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // 10 na, 2 flute
        let expect = 0.5 * (10.0 * 100.0 + 2.0 * 200.0);
        assert_eq!(total_dmg, expect);
    }

    #[test]
    fn prototype_archaic() {
        let mut members = vec![
            FieldAbility::boxed(
                TestCharacter::new(),
                TestAbility(PrototypeArchaicR5::new()),
                TestArtifact(State::new()),
            ).to_data(FieldCharacterIndex(0))
        ];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // skill na na na (prototype_archaic)
        let expect = 0.5 * (200.0 + 100.0 + 100.0 + 100.0 + 480.0);
        assert_eq!(total_dmg, expect);
    }

    #[test]
    fn prototype_archaic_physical() {
        let mut members = vec![
            FieldAbility::boxed(
                TestCharacter::new(),
                TestAbility(PrototypeArchaicR5::new()),
                TestArtifact(State::new().physical_dmg(10.0)),
            ).to_data(FieldCharacterIndex(0))
        ];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // skill na na na (prototype_archaic)
        let expect = 0.5 * (200.0 + 110.0 + 110.0 + 110.0 + 480.0 * 1.1);
        assert_eq!(total_dmg, expect);
    }

    #[test]
    fn prototype_archaic_na() {
        let mut members = vec![
            FieldAbility::boxed(
                TestCharacter::new(),
                TestAbility(PrototypeArchaicR5::new()),
                TestArtifact(State::new().na_dmg(10.0)),
            ).to_data(FieldCharacterIndex(0))
        ];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // skill na na na (prototype_archaic)
        let expect = 0.5 * (200.0 + 110.0 + 110.0 + 110.0 + 480.0);
        assert_eq!(total_dmg, expect);
    }

    #[test]
    fn prototype_archaic_cd() {
        let mut members = vec![TestEnvironment::no_skill(
            Box::new(TestCharacter::new()),
            Box::new(TestAbility(PrototypeArchaicR5::new())),
            Box::new(TestArtifact(State::new()))
        )];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..4 {
            total_dmg += simulate(&mut members, &mut enemy, 10.0);
        }
        // na na (prototype_archaic) na na (prototype_archaic)
        let expect = 0.5 * (100.0 + 100.0 + 480.0 + 100.0 + 100.0 + 480.0);
        assert_eq!(total_dmg, expect);
    }

    #[test]
    fn skywardblade() {
        let mut members = vec![TestEnvironment::no_skill(
            Box::new(TestCharacter::new()),
            Box::new(TestAbility(SkywardBlade::new())),
            Box::new(TestArtifact(State::new()))
        )];
        members[0].0.state.energy.0 = members[0].0.cr.energy_cost;
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // burst na na na
        let expect = 0.5 * (300.0 + 3.0 * 120.0);
        assert_eq!(total_dmg, expect);
    }

    #[test]
    fn songofbrokenpines() {
        let mut members = vec![TestEnvironment::no_skill(
            Box::new(TestCharacter::new()),
            Box::new(TestAbility(SongOfBrokenPines::new())),
            Box::new(TestArtifact(State::new()))
        )];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..20 {
            total_dmg += simulate(&mut members, &mut enemy, 1.0);
        }
        // 8 na and 12 na
        let expect = 0.5 * (8.0 * 100.0 + 12.0 * 120.0);
        assert_eq!(total_dmg, expect);
    }
}
