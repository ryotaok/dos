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
pub mod version_2_1;

use crate::fc::{FieldCharacterIndex, WeaponRecord, SpecialAbility};
use crate::action::ICDTimers;
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
use version_2_1::*;

pub fn all(idx: FieldCharacterIndex, icd_timer: &ICDTimers) -> Vec<(WeaponRecord, Box<dyn SpecialAbility>)> {
    vec![
    // sword_4star
    (PrototypeRancourR5::record(), Box::new(PrototypeRancourR5::new(idx))),
    (TheBlackSwordR5::record(), Box::new(TheBlackSwordR5)),
    (BlackcliffLongswordR5::record(), Box::new(BlackcliffLongswordR5)),
    (RoyalLongswordR5::record(), Box::new(RoyalLongswordR5)),
    (HarbingerOfDawnR5::record(), Box::new(HarbingerOfDawnR5)),
    (TheFluteR5::record(), Box::new(TheFluteR5::new(idx, icd_timer))),
    (LionsRoarR5::record(), Box::new(LionsRoarR5::new(idx))),
    // claymore_4star
    (PrototypeArchaicR5::record(), Box::new(PrototypeArchaicR5::new(idx, icd_timer))),
    (WhiteblindR5::record(), Box::new(WhiteblindR5::new(idx))),
    (SerpentSpineR5::record(), Box::new(SerpentSpineR5::new(idx))),
    (BlackcliffSlasherR5::record(), Box::new(BlackcliffSlasherR5)),
    (RoyalGreatswordR5::record(), Box::new(RoyalGreatswordR5)),
    (RainslasherR5::record(), Box::new(RainslasherR5::new(idx))),
    // polearm_4star
    (PrototypeStarglitterR5::record(), Box::new(PrototypeStarglitterR5::new(idx))),
    (CrescentPikeR5::record(), Box::new(CrescentPikeR5::new(idx, icd_timer))),
    (DeathmatchR5::record(), Box::new(DeathmatchR5)),
    (BlackcliffPoleR5::record(), Box::new(BlackcliffPoleR5)),
    (RoyalSpearR5::record(), Box::new(RoyalSpearR5)),
    (WhiteTasselR5::record(), Box::new(WhiteTasselR5)),
    (DragonsBaneR5::record(), Box::new(DragonsBaneR5::new(idx))),
    // bow_4star
    (PrototypeCrescentR5::record(), Box::new(PrototypeCrescentR5)),
    (CompoundBowR5::record(), Box::new(CompoundBowR5::new(idx))),
    (TheViridescentHuntR5::record(), Box::new(TheViridescentHuntR5::new(idx, icd_timer))),
    (BlackcliffWarbowR5::record(), Box::new(BlackcliffWarbowR5)),
    (RoyalBowR5::record(), Box::new(RoyalBowR5)),
    (SlingshotR5::record(), Box::new(SlingshotR5)),
    (RustR5::record(), Box::new(RustR5)),
    (TheStringlessR5::record(), Box::new(TheStringlessR5)),
    // catalyst_4star
    (PrototypeAmberR5::record(), Box::new(PrototypeAmberR5)),
    (MappaMareR5::record(), Box::new(MappaMareR5::new(idx))),
    (SolarPearlR5::record(), Box::new(SolarPearlR5::new(idx))),
    (BlackcliffAgateR5::record(), Box::new(BlackcliffAgateR5)),
    (RoyalGrimoireR5::record(), Box::new(RoyalGrimoireR5)),
    (ThrillingTalesOfDragonSlayersR5::record(), Box::new(ThrillingTalesOfDragonSlayersR5::new())),
    (EyeOfPerceptionR5::record(), Box::new(EyeOfPerceptionR5::new(idx, icd_timer))),
    (TheWidsithR5::record(), Box::new(TheWidsithR5::new(idx))),
    // favonius_series
    (FavoniusGreatswordR5::record(), Box::new(FavoniusGreatswordR5::new())),
    (FavoniusSwordR5::record(), Box::new(FavoniusSwordR5::new())),
    (FavoniusLanceR5::record(), Box::new(FavoniusLanceR5::new())),
    (FavoniusWarbowR5::record(), Box::new(FavoniusWarbowR5::new())),
    (FavoniusCodexR5::record(), Box::new(FavoniusCodexR5::new())),
    // sacrificial_series
    (SacrificialSwordR5::record(), Box::new(SacrificialSwordR5::new())),
    (SacrificialGreatswordR5::record(), Box::new(SacrificialGreatswordR5::new())),
    (SacrificialBowR5::record(), Box::new(SacrificialBowR5::new())),
    (SacrificialFragmentsR5::record(), Box::new(SacrificialFragmentsR5::new())),
    // version_1_5star
    (SkywardBlade::record(), Box::new(SkywardBlade::new(idx, icd_timer))),
    (AquilaFavonia::record(), Box::new(AquilaFavonia::new(idx, icd_timer))),
    (SkywardPride::record(), Box::new(SkywardPride::new(idx, icd_timer))),
    (WolfsGravestone::record(), Box::new(WolfsGravestone)),
    (SkywardSpine::record(), Box::new(SkywardSpine::new(idx, icd_timer))),
    (PrimordialJadeWingedSpear::record(), Box::new(PrimordialJadeWingedSpear::new(idx))),
    (SkywardHarp::record(), Box::new(SkywardHarp::new(idx, icd_timer))),
    (AmosBow::record(), Box::new(AmosBow)),
    (SkywardAtlas::record(), Box::new(SkywardAtlas::new(idx, icd_timer))),
    (LostPrayerToTheSacredWinds::record(), Box::new(LostPrayerToTheSacredWinds::new(idx))),
    // version_1_1
    (TheUnforged::record(), Box::new(TheUnforged::new(idx))),
    (SummitShaper::record(), Box::new(SummitShaper::new(idx))),
    (VortexVanquisher::record(), Box::new(VortexVanquisher::new(idx))),
    (MemoryOfDust::record(), Box::new(MemoryOfDust::new(idx))),
    // version_1_2
    (FesteringDesire::record(), Box::new(FesteringDesire::new(idx, icd_timer))),
    (SnowTombedStarsilver::record(), Box::new(SnowTombedStarsilver::new(idx, icd_timer))),
    (DragonspineSpear::record(), Box::new(DragonspineSpear::new(idx, icd_timer))),
    (Frostbearer::record(), Box::new(Frostbearer::new(idx, icd_timer))),
    // version_1_3
    (PrimordialJadeCutter::record(), Box::new(PrimordialJadeCutter::new(idx))),
    (PrimordialJadeGS::record(), Box::new(PrimordialJadeGS::new(idx))),
    (PrimordialJadeVista::record(), Box::new(PrimordialJadeVista::new(idx))),
    (StaffOfHoma::record(), Box::new(StaffOfHoma::new(idx))),
    (LithicSpear::record(), Box::new(LithicSpear)),
    (LithicBlade::record(), Box::new(LithicBlade)),
    // version_1_4
    (ElegyForTheEnd::record(), Box::new(ElegyForTheEnd::new(idx))),
    (TheAlleyFlash::record(), Box::new(TheAlleyFlash)),
    (AlleyHunter::record(), Box::new(AlleyHunter::new(idx))),
    (WineAndSong::record(), Box::new(WineAndSong)),
    (WindblumeOde::record(), Box::new(WindblumeOde::new(idx))),
    // version_1_5
    (SongOfBrokenPines::record(), Box::new(SongOfBrokenPines::new(idx))),
    // version_1_6
    (FreedomSworn::record(), Box::new(FreedomSworn::new(idx))),
    (MitternachtsWaltz::record(), Box::new(MitternachtsWaltz::new(idx))),
    (DodocoTales::record(), Box::new(DodocoTales::new(idx))),
    // version_2_0
    (MistsplitterReforged::record(), Box::new(MistsplitterReforged::new(idx))),
    (ThunderingPulse::record(), Box::new(ThunderingPulse::new(idx))),
    (AmenomaKageuchi::record(), Box::new(AmenomaKageuchi::new(idx))),
    (KatsuragikiriNagamasa::record(), Box::new(KatsuragikiriNagamasa::new(idx))),
    (KitainCrossSpear::record(), Box::new(KitainCrossSpear::new(idx))),
    (Hamayumi::record(), Box::new(Hamayumi::new(idx))),
    (HakushinRing::record(), Box::new(HakushinRing::new(idx))),
    // version_2_1
    (EngulfingLightning::record(), Box::new(EngulfingLightning::new(idx))),
    (EverlastingMoonglow::record(), Box::new(EverlastingMoonglow::new(idx))),
    (LuxuriousSeaLord::record(), Box::new(LuxuriousSeaLord::new(idx, icd_timer))),
    (TheCatch::record(), Box::new(TheCatch(idx))),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::state::State;
    use crate::types::{Vision, FieldEnergy};
    use crate::simulate::simulate;
    use crate::fc::{CharacterData, FieldAbility};
    use crate::action::Attack;
    use crate::testutil::{TestEnvironment};

    use Vision::*;

    // Note that Test disables each weapon's main stats

    #[test]
    fn prototype_rancour() {
        let idx = FieldCharacterIndex(0);
        let mut enemy = TestEnvironment::enemy();
        let mut members: Vec<CharacterData> = Vec::new();
        let mut abilities: Vec<FieldAbility> = Vec::new();
        let mut atk_queue: Vec<*const Attack> = Vec::new();
        let mut field_energy: Vec<FieldEnergy> = Vec::new();

        let mut wa = PrototypeRancourR5::new(idx);

        let mut env1 = TestEnvironment::new();
        let (data, ability) = env1.weapon(idx, State::new(), Pyro, &mut wa);
        members.push(data);
        abilities.push(ability);

        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(0.2, &mut members, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
        }
        // skill na na na
        // TODO why twice 116.0
        let expect = 200.0 + 108.0 + 116.0 * 2.0 + 124.0 + 132.0;
        assert_eq!(total_dmg, expect);
    }

    #[test]
    fn the_flute() {
        let idx = FieldCharacterIndex(0);
        let mut enemy = TestEnvironment::enemy();
        let mut members: Vec<CharacterData> = Vec::new();
        let mut abilities: Vec<FieldAbility> = Vec::new();
        let mut atk_queue: Vec<*const Attack> = Vec::new();
        let mut field_energy: Vec<FieldEnergy> = Vec::new();

        let mut env1 = TestEnvironment::new();
        let mut wa = TheFluteR5::new(idx, &env1.timers);

        let (data, ability) = env1.no_skill_weapon(idx, State::new(), Pyro, &mut wa);
        members.push(data);
        abilities.push(ability);

        let mut total_dmg = 0.0;
        for _ in 0..40 {
            total_dmg += simulate(0.2, &mut members, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
        }
        // 20 na, 2 flute
        let expect = 20.0 * 100.0 + 2.0 * 200.0;
        assert_eq!(total_dmg, expect);
    }

    #[test]
    fn prototype_archaic() {
        let idx = FieldCharacterIndex(0);
        let mut enemy = TestEnvironment::enemy();
        let mut members: Vec<CharacterData> = Vec::new();
        let mut abilities: Vec<FieldAbility> = Vec::new();
        let mut atk_queue: Vec<*const Attack> = Vec::new();
        let mut field_energy: Vec<FieldEnergy> = Vec::new();

        let mut env1 = TestEnvironment::new();
        let mut wa = PrototypeArchaicR5::new(idx, &env1.timers);

        let (data, ability) = env1.weapon(idx, State::new(), Pyro, &mut wa);
        members.push(data);
        abilities.push(ability);

        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(0.2, &mut members, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
        }
        // skill na na na na (prototype_archaic)
        let expect = 200.0 + 5.0 * 100.0 + 480.0;
        assert_eq!(total_dmg, expect);
    }

    #[test]
    fn prototype_archaic_physical() {
        let idx = FieldCharacterIndex(0);
        let mut enemy = TestEnvironment::enemy();
        let mut members: Vec<CharacterData> = Vec::new();
        let mut abilities: Vec<FieldAbility> = Vec::new();
        let mut atk_queue: Vec<*const Attack> = Vec::new();
        let mut field_energy: Vec<FieldEnergy> = Vec::new();

        let mut env1 = TestEnvironment::new();
        let mut wa = PrototypeArchaicR5::new(idx, &env1.timers);

        let (data, ability) = env1.weapon(idx, State::new().physical_dmg(10.0), Pyro, &mut wa);
        members.push(data);
        abilities.push(ability);

        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(0.2, &mut members, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
        }
        // skill na na na na (prototype_archaic)
        let expect = 200.0 + 5.0 * 110.0 + 480.0 * 1.1;
        assert_eq!(total_dmg, expect);
    }

    #[test]
    fn prototype_archaic_na() {
        let idx = FieldCharacterIndex(0);
        let mut enemy = TestEnvironment::enemy();
        let mut members: Vec<CharacterData> = Vec::new();
        let mut abilities: Vec<FieldAbility> = Vec::new();
        let mut atk_queue: Vec<*const Attack> = Vec::new();
        let mut field_energy: Vec<FieldEnergy> = Vec::new();

        let mut env1 = TestEnvironment::new();
        let mut wa = PrototypeArchaicR5::new(idx, &env1.timers);

        let (data, ability) = env1.weapon(idx, State::new().na_dmg(10.0), Pyro, &mut wa);
        members.push(data);
        abilities.push(ability);

        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(0.2, &mut members, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
        }
        // skill na na na (prototype_archaic)
        let expect = 200.0 + 5.0 * 110.0 + 480.0;
        assert_eq!(total_dmg, expect);
    }

    #[test]
    fn prototype_archaic_cd() {
        let idx = FieldCharacterIndex(0);
        let mut enemy = TestEnvironment::enemy();
        let mut members: Vec<CharacterData> = Vec::new();
        let mut abilities: Vec<FieldAbility> = Vec::new();
        let mut atk_queue: Vec<*const Attack> = Vec::new();
        let mut field_energy: Vec<FieldEnergy> = Vec::new();

        let mut env1 = TestEnvironment::new();
        let mut wa = PrototypeArchaicR5::new(idx, &env1.timers);

        let (data, ability) = env1.no_skill_weapon(idx, State::new(), Pyro, &mut wa);
        members.push(data);
        abilities.push(ability);

        let mut total_dmg = 0.0;
        for _ in 0..5 {
            total_dmg += simulate(10.0, &mut members, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
        }

        // na na (prototype_archaic) na na (prototype_archaic)
        let expect = 100.0 + 100.0 + 480.0 + 100.0 + 100.0 + 480.0;
        assert_eq!(total_dmg, expect);
    }

    #[test]
    fn skywardblade() {
        let idx = FieldCharacterIndex(0);
        let mut enemy = TestEnvironment::enemy();
        let mut members: Vec<CharacterData> = Vec::new();
        let mut abilities: Vec<FieldAbility> = Vec::new();
        let mut atk_queue: Vec<*const Attack> = Vec::new();
        let mut field_energy: Vec<FieldEnergy> = Vec::new();

        let mut env1 = TestEnvironment::new();
        let mut wa = SkywardBlade::new(idx, &env1.timers);

        let (data, ability) = env1.no_skill_weapon(idx, State::new(), Pyro, &mut wa);
        members.push(data);
        abilities.push(ability);

        let mut total_dmg = 0.0;
        members[0].state.energy = members[0].character.energy_cost;
        for _ in 0..10 {
            total_dmg += simulate(0.2, &mut members, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
        }

        // TODO one extra AA?
        // burst na na na na
        let expect = 300.0 + 5.0 * 120.0 + 20.0;
        assert_eq!(total_dmg, expect);
    }

    #[test]
    fn songofbrokenpines() {
        let idx = FieldCharacterIndex(0);
        let mut enemy = TestEnvironment::enemy();
        let mut members: Vec<CharacterData> = Vec::new();
        let mut abilities: Vec<FieldAbility> = Vec::new();
        let mut atk_queue: Vec<*const Attack> = Vec::new();
        let mut field_energy: Vec<FieldEnergy> = Vec::new();

        let mut wa = SongOfBrokenPines::new(idx);

        let mut env1 = TestEnvironment::new();
        let (data, ability) = env1.no_skill_weapon(idx, State::new(), Pyro, &mut wa);
        members.push(data);
        abilities.push(ability);

        let mut total_dmg = 0.0;
        for _ in 0..20 {
            total_dmg += simulate(1.0, &mut members, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
        }

        let expect = 6.0 * 100.0 + 13.0 * 120.0;
        assert_eq!(total_dmg, expect);
    }
}
