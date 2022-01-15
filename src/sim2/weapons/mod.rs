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
pub mod version_2_2;
pub mod version_2_3;
pub mod version_2_4;
pub mod version_2_5;

use crate::sim2::timeline::Timeline;
use crate::sim2::attack::WeaponAttack;
use crate::sim2::record::{WeaponRecord};

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
use version_2_2::*;
use version_2_3::*;
use version_2_4::*;
use version_2_5::*;

pub enum WeaponUnion {
    // sword_4star
    PrototypeRancourR5(PrototypeRancourR5),
    TheBlackSwordR5(TheBlackSwordR5),
    BlackcliffLongswordR5(BlackcliffLongswordR5),
    RoyalLongswordR5(RoyalLongswordR5),
    HarbingerOfDawnR5(HarbingerOfDawnR5),
    TheFluteR5(TheFluteR5),
    LionsRoarR5(LionsRoarR5),
    // claymore_4star
    PrototypeArchaicR5(PrototypeArchaicR5),
    WhiteblindR5(WhiteblindR5),
    SerpentSpineR5(SerpentSpineR5),
    BlackcliffSlasherR5(BlackcliffSlasherR5),
    RoyalGreatswordR5(RoyalGreatswordR5),
    RainslasherR5(RainslasherR5),
    // polearm_4star
    PrototypeStarglitterR5(PrototypeStarglitterR5),
    CrescentPikeR5(CrescentPikeR5),
    DeathmatchR5(DeathmatchR5),
    BlackcliffPoleR5(BlackcliffPoleR5),
    RoyalSpearR5(RoyalSpearR5),
    WhiteTasselR5(WhiteTasselR5),
    DragonsBaneR5(DragonsBaneR5),
    // bow_4star
    PrototypeCrescentR5(PrototypeCrescentR5),
    CompoundBowR5(CompoundBowR5),
    TheViridescentHuntR5(TheViridescentHuntR5),
    BlackcliffWarbowR5(BlackcliffWarbowR5),
    RoyalBowR5(RoyalBowR5),
    SlingshotR5(SlingshotR5),
    RustR5(RustR5),
    TheStringlessR5(TheStringlessR5),
    // catalyst_4star
    PrototypeAmberR5(PrototypeAmberR5),
    MappaMareR5(MappaMareR5),
    SolarPearlR5(SolarPearlR5),
    BlackcliffAgateR5(BlackcliffAgateR5),
    RoyalGrimoireR5(RoyalGrimoireR5),
    ThrillingTalesOfDragonSlayersR5(ThrillingTalesOfDragonSlayersR5),
    EyeOfPerceptionR5(EyeOfPerceptionR5),
    TheWidsithR5(TheWidsithR5),
    // favonius_series
    FavoniusGreatswordR5(FavoniusGreatswordR5),
    FavoniusSwordR5(FavoniusSwordR5),
    FavoniusLanceR5(FavoniusLanceR5),
    FavoniusWarbowR5(FavoniusWarbowR5),
    FavoniusCodexR5(FavoniusCodexR5),
    // sacrificial_series
    SacrificialSwordR5(SacrificialSwordR5),
    SacrificialGreatswordR5(SacrificialGreatswordR5),
    SacrificialBowR5(SacrificialBowR5),
    SacrificialFragmentsR5(SacrificialFragmentsR5),
    // version_1_5star
    SkywardBlade(SkywardBlade),
    AquilaFavonia(AquilaFavonia),
    SkywardPride(SkywardPride),
    WolfsGravestone(WolfsGravestone),
    SkywardSpine(SkywardSpine),
    PrimordialJadeWingedSpear(PrimordialJadeWingedSpear),
    SkywardHarp(SkywardHarp),
    AmosBow(AmosBow),
    SkywardAtlas(SkywardAtlas),
    LostPrayerToTheSacredWinds(LostPrayerToTheSacredWinds),
    // version_1_1
    TheUnforged(TheUnforged),
    SummitShaper(SummitShaper),
    VortexVanquisher(VortexVanquisher),
    MemoryOfDust(MemoryOfDust),
    // version_1_2
    FesteringDesire(FesteringDesire),
    SnowTombedStarsilver(SnowTombedStarsilver),
    DragonspineSpear(DragonspineSpear),
    Frostbearer(Frostbearer),
    // version_1_3
    PrimordialJadeCutter(PrimordialJadeCutter),
    PrimordialJadeGS(PrimordialJadeGS),
    PrimordialJadeVista(PrimordialJadeVista),
    StaffOfHoma(StaffOfHoma),
    LithicSpear(LithicSpear),
    LithicBlade(LithicBlade),
    // version_1_4
    ElegyForTheEnd(ElegyForTheEnd),
    TheAlleyFlash(TheAlleyFlash),
    AlleyHunter(AlleyHunter),
    WineAndSong(WineAndSong),
    WindblumeOde(WindblumeOde),
    // version_1_5
    SongOfBrokenPines(SongOfBrokenPines),
    // version_1_6
    FreedomSworn(FreedomSworn),
    MitternachtsWaltz(MitternachtsWaltz),
    DodocoTales(DodocoTales),
    // version_2_0
    MistsplitterReforged(MistsplitterReforged),
    MistsplitterReforgedClaymore(MistsplitterReforgedClaymore),
    MistsplitterReforgedPolearm(MistsplitterReforgedPolearm),
    MistsplitterReforgedBow(MistsplitterReforgedBow),
    MistsplitterReforgedCatalyst(MistsplitterReforgedCatalyst),
    ThunderingPulse(ThunderingPulse),
    AmenomaKageuchi(AmenomaKageuchi),
    KatsuragikiriNagamasa(KatsuragikiriNagamasa),
    KitainCrossSpear(KitainCrossSpear),
    Hamayumi(Hamayumi),
    HakushinRing(HakushinRing),
    // version_2_1
    EngulfingLightning(EngulfingLightning),
    EverlastingMoonglow(EverlastingMoonglow),
    LuxuriousSeaLord(LuxuriousSeaLord),
    TheCatch(TheCatch),
    // version_2_2
    PolarStar(PolarStar),
    PolarStarSword(PolarStarSword),
    PolarStarClaymore(PolarStarClaymore),
    PolarStarPolearm(PolarStarPolearm),
    PolarStarCatalyst(PolarStarCatalyst),
    Akuoumaru(Akuoumaru),
    MouunsMoon(MouunsMoon),
    WavebreakersFin(WavebreakersFin),
    // version_2_3
    RedhornStonethresher(RedhornStonethresher),
    CinnabarSpindle(CinnabarSpindle),
    // version_2_4
    CalamityQueller(CalamityQueller),
    // version_2_5
    KagurasVerity(KagurasVerity),
}

impl WeaponUnion {
    pub fn timeline(&mut self) -> &mut dyn Timeline {
        use WeaponUnion::*;
        match self {
            // sword_4star
            PrototypeRancourR5(x) => x,
            TheBlackSwordR5(x) => x,
            BlackcliffLongswordR5(x) => x,
            RoyalLongswordR5(x) => x,
            HarbingerOfDawnR5(x) => x,
            TheFluteR5(x) => x,
            LionsRoarR5(x) => x,
            // claymore_4star
            PrototypeArchaicR5(x) => x,
            WhiteblindR5(x) => x,
            SerpentSpineR5(x) => x,
            BlackcliffSlasherR5(x) => x,
            RoyalGreatswordR5(x) => x,
            RainslasherR5(x) => x,
            // polearm_4star
            PrototypeStarglitterR5(x) => x,
            CrescentPikeR5(x) => x,
            DeathmatchR5(x) => x,
            BlackcliffPoleR5(x) => x,
            RoyalSpearR5(x) => x,
            WhiteTasselR5(x) => x,
            DragonsBaneR5(x) => x,
            // bow_4star
            PrototypeCrescentR5(x) => x,
            CompoundBowR5(x) => x,
            TheViridescentHuntR5(x) => x,
            BlackcliffWarbowR5(x) => x,
            RoyalBowR5(x) => x,
            SlingshotR5(x) => x,
            RustR5(x) => x,
            TheStringlessR5(x) => x,
            // catalyst_4star
            PrototypeAmberR5(x) => x,
            MappaMareR5(x) => x,
            SolarPearlR5(x) => x,
            BlackcliffAgateR5(x) => x,
            RoyalGrimoireR5(x) => x,
            ThrillingTalesOfDragonSlayersR5(x) => x,
            EyeOfPerceptionR5(x) => x,
            TheWidsithR5(x) => x,
            // favonius_series
            FavoniusGreatswordR5(x) => x,
            FavoniusSwordR5(x) => x,
            FavoniusLanceR5(x) => x,
            FavoniusWarbowR5(x) => x,
            FavoniusCodexR5(x) => x,
            // sacrificial_series
            SacrificialSwordR5(x) => x,
            SacrificialGreatswordR5(x) => x,
            SacrificialBowR5(x) => x,
            SacrificialFragmentsR5(x) => x,
            // version_1_5star
            SkywardBlade(x) => x,
            AquilaFavonia(x) => x,
            SkywardPride(x) => x,
            WolfsGravestone(x) => x,
            SkywardSpine(x) => x,
            PrimordialJadeWingedSpear(x) => x,
            SkywardHarp(x) => x,
            AmosBow(x) => x,
            SkywardAtlas(x) => x,
            LostPrayerToTheSacredWinds(x) => x,
            // version_1_1
            TheUnforged(x) => x,
            SummitShaper(x) => x,
            VortexVanquisher(x) => x,
            MemoryOfDust(x) => x,
            // version_1_2
            FesteringDesire(x) => x,
            SnowTombedStarsilver(x) => x,
            DragonspineSpear(x) => x,
            Frostbearer(x) => x,
            // version_1_3
            PrimordialJadeCutter(x) => x,
            PrimordialJadeGS(x) => x,
            PrimordialJadeVista(x) => x,
            StaffOfHoma(x) => x,
            LithicSpear(x) => x,
            LithicBlade(x) => x,
            // version_1_4
            ElegyForTheEnd(x) => x,
            TheAlleyFlash(x) => x,
            AlleyHunter(x) => x,
            WineAndSong(x) => x,
            WindblumeOde(x) => x,
            // version_1_5
            SongOfBrokenPines(x) => x,
            // version_1_6
            FreedomSworn(x) => x,
            MitternachtsWaltz(x) => x,
            DodocoTales(x) => x,
            // version_2_0
            MistsplitterReforged(x) => x,
            MistsplitterReforgedClaymore(x) => x,
            MistsplitterReforgedPolearm(x) => x,
            MistsplitterReforgedBow(x) => x,
            MistsplitterReforgedCatalyst(x) => x,
            ThunderingPulse(x) => x,
            AmenomaKageuchi(x) => x,
            KatsuragikiriNagamasa(x) => x,
            KitainCrossSpear(x) => x,
            Hamayumi(x) => x,
            HakushinRing(x) => x,
            // version_2_1
            EngulfingLightning(x) => x,
            EverlastingMoonglow(x) => x,
            LuxuriousSeaLord(x) => x,
            TheCatch(x) => x,
            // version_2_2
            PolarStar(x) => x,
            PolarStarSword(x) => x,
            PolarStarClaymore(x) => x,
            PolarStarPolearm(x) => x,
            PolarStarCatalyst(x) => x,
            Akuoumaru(x) => x,
            MouunsMoon(x) => x,
            WavebreakersFin(x) => x,
            // version_2_3
            RedhornStonethresher(x) => x,
            CinnabarSpindle(x) => x,
            // version_2_4
            CalamityQueller(x) => x,
            // version_2_5
            KagurasVerity(x) => x,
        }
    }

    pub fn field(&mut self) -> &mut dyn WeaponAttack {
        use WeaponUnion::*;
        match self {
            // sword_4star
            PrototypeRancourR5(x) => x,
            TheBlackSwordR5(x) => x,
            BlackcliffLongswordR5(x) => x,
            RoyalLongswordR5(x) => x,
            HarbingerOfDawnR5(x) => x,
            TheFluteR5(x) => x,
            LionsRoarR5(x) => x,
            // claymore_4star
            PrototypeArchaicR5(x) => x,
            WhiteblindR5(x) => x,
            SerpentSpineR5(x) => x,
            BlackcliffSlasherR5(x) => x,
            RoyalGreatswordR5(x) => x,
            RainslasherR5(x) => x,
            // polearm_4star
            PrototypeStarglitterR5(x) => x,
            CrescentPikeR5(x) => x,
            DeathmatchR5(x) => x,
            BlackcliffPoleR5(x) => x,
            RoyalSpearR5(x) => x,
            WhiteTasselR5(x) => x,
            DragonsBaneR5(x) => x,
            // bow_4star
            PrototypeCrescentR5(x) => x,
            CompoundBowR5(x) => x,
            TheViridescentHuntR5(x) => x,
            BlackcliffWarbowR5(x) => x,
            RoyalBowR5(x) => x,
            SlingshotR5(x) => x,
            RustR5(x) => x,
            TheStringlessR5(x) => x,
            // catalyst_4star
            PrototypeAmberR5(x) => x,
            MappaMareR5(x) => x,
            SolarPearlR5(x) => x,
            BlackcliffAgateR5(x) => x,
            RoyalGrimoireR5(x) => x,
            ThrillingTalesOfDragonSlayersR5(x) => x,
            EyeOfPerceptionR5(x) => x,
            TheWidsithR5(x) => x,
            // favonius_series
            FavoniusGreatswordR5(x) => x,
            FavoniusSwordR5(x) => x,
            FavoniusLanceR5(x) => x,
            FavoniusWarbowR5(x) => x,
            FavoniusCodexR5(x) => x,
            // sacrificial_series
            SacrificialSwordR5(x) => x,
            SacrificialGreatswordR5(x) => x,
            SacrificialBowR5(x) => x,
            SacrificialFragmentsR5(x) => x,
            // version_1_5star
            SkywardBlade(x) => x,
            AquilaFavonia(x) => x,
            SkywardPride(x) => x,
            WolfsGravestone(x) => x,
            SkywardSpine(x) => x,
            PrimordialJadeWingedSpear(x) => x,
            SkywardHarp(x) => x,
            AmosBow(x) => x,
            SkywardAtlas(x) => x,
            LostPrayerToTheSacredWinds(x) => x,
            // version_1_1
            TheUnforged(x) => x,
            SummitShaper(x) => x,
            VortexVanquisher(x) => x,
            MemoryOfDust(x) => x,
            // version_1_2
            FesteringDesire(x) => x,
            SnowTombedStarsilver(x) => x,
            DragonspineSpear(x) => x,
            Frostbearer(x) => x,
            // version_1_3
            PrimordialJadeCutter(x) => x,
            PrimordialJadeGS(x) => x,
            PrimordialJadeVista(x) => x,
            StaffOfHoma(x) => x,
            LithicSpear(x) => x,
            LithicBlade(x) => x,
            // version_1_4
            ElegyForTheEnd(x) => x,
            TheAlleyFlash(x) => x,
            AlleyHunter(x) => x,
            WineAndSong(x) => x,
            WindblumeOde(x) => x,
            // version_1_5
            SongOfBrokenPines(x) => x,
            // version_1_6
            FreedomSworn(x) => x,
            MitternachtsWaltz(x) => x,
            DodocoTales(x) => x,
            // version_2_0
            MistsplitterReforged(x) => x,
            MistsplitterReforgedClaymore(x) => x,
            MistsplitterReforgedPolearm(x) => x,
            MistsplitterReforgedBow(x) => x,
            MistsplitterReforgedCatalyst(x) => x,
            ThunderingPulse(x) => x,
            AmenomaKageuchi(x) => x,
            KatsuragikiriNagamasa(x) => x,
            KitainCrossSpear(x) => x,
            Hamayumi(x) => x,
            HakushinRing(x) => x,
            // version_2_1
            EngulfingLightning(x) => x,
            EverlastingMoonglow(x) => x,
            LuxuriousSeaLord(x) => x,
            TheCatch(x) => x,
            // version_2_2
            PolarStar(x) => x,
            PolarStarSword(x) => x,
            PolarStarClaymore(x) => x,
            PolarStarPolearm(x) => x,
            PolarStarCatalyst(x) => x,
            Akuoumaru(x) => x,
            MouunsMoon(x) => x,
            WavebreakersFin(x) => x,
            // version_2_3
            RedhornStonethresher(x) => x,
            CinnabarSpindle(x) => x,
            // version_2_4
            CalamityQueller(x) => x,
            // version_2_5
            KagurasVerity(x) => x,
        }
    }
}

pub fn all() -> Vec<(WeaponRecord, WeaponUnion)> {
    vec![
    // sword_4star
    (PrototypeRancourR5::record(), WeaponUnion::PrototypeRancourR5(PrototypeRancourR5::new())),
    (TheBlackSwordR5::record(), WeaponUnion::TheBlackSwordR5(TheBlackSwordR5)),
    (BlackcliffLongswordR5::record(), WeaponUnion::BlackcliffLongswordR5(BlackcliffLongswordR5)),
    (RoyalLongswordR5::record(), WeaponUnion::RoyalLongswordR5(RoyalLongswordR5)),
    (HarbingerOfDawnR5::record(), WeaponUnion::HarbingerOfDawnR5(HarbingerOfDawnR5)),
    (TheFluteR5::record(), WeaponUnion::TheFluteR5(TheFluteR5::new())),
    (LionsRoarR5::record(), WeaponUnion::LionsRoarR5(LionsRoarR5::new())),
    // claymore_4star
    (PrototypeArchaicR5::record(), WeaponUnion::PrototypeArchaicR5(PrototypeArchaicR5::new())),
    (WhiteblindR5::record(), WeaponUnion::WhiteblindR5(WhiteblindR5::new())),
    (SerpentSpineR5::record(), WeaponUnion::SerpentSpineR5(SerpentSpineR5::new())),
    (BlackcliffSlasherR5::record(), WeaponUnion::BlackcliffSlasherR5(BlackcliffSlasherR5)),
    (RoyalGreatswordR5::record(), WeaponUnion::RoyalGreatswordR5(RoyalGreatswordR5)),
    (RainslasherR5::record(), WeaponUnion::RainslasherR5(RainslasherR5::new())),
    // polearm_4star
    (PrototypeStarglitterR5::record(), WeaponUnion::PrototypeStarglitterR5(PrototypeStarglitterR5::new())),
    (CrescentPikeR5::record(), WeaponUnion::CrescentPikeR5(CrescentPikeR5::new())),
    (DeathmatchR5::record(), WeaponUnion::DeathmatchR5(DeathmatchR5)),
    (BlackcliffPoleR5::record(), WeaponUnion::BlackcliffPoleR5(BlackcliffPoleR5)),
    (RoyalSpearR5::record(), WeaponUnion::RoyalSpearR5(RoyalSpearR5)),
    (WhiteTasselR5::record(), WeaponUnion::WhiteTasselR5(WhiteTasselR5)),
    (DragonsBaneR5::record(), WeaponUnion::DragonsBaneR5(DragonsBaneR5::new())),
    // bow_4star
    (PrototypeCrescentR5::record(), WeaponUnion::PrototypeCrescentR5(PrototypeCrescentR5)),
    (CompoundBowR5::record(), WeaponUnion::CompoundBowR5(CompoundBowR5::new())),
    (TheViridescentHuntR5::record(), WeaponUnion::TheViridescentHuntR5(TheViridescentHuntR5::new())),
    (BlackcliffWarbowR5::record(), WeaponUnion::BlackcliffWarbowR5(BlackcliffWarbowR5)),
    (RoyalBowR5::record(), WeaponUnion::RoyalBowR5(RoyalBowR5)),
    (SlingshotR5::record(), WeaponUnion::SlingshotR5(SlingshotR5)),
    (RustR5::record(), WeaponUnion::RustR5(RustR5)),
    (TheStringlessR5::record(), WeaponUnion::TheStringlessR5(TheStringlessR5)),
    // catalyst_4star
    (PrototypeAmberR5::record(), WeaponUnion::PrototypeAmberR5(PrototypeAmberR5)),
    (MappaMareR5::record(), WeaponUnion::MappaMareR5(MappaMareR5::new())),
    (SolarPearlR5::record(), WeaponUnion::SolarPearlR5(SolarPearlR5::new())),
    (BlackcliffAgateR5::record(), WeaponUnion::BlackcliffAgateR5(BlackcliffAgateR5)),
    (RoyalGrimoireR5::record(), WeaponUnion::RoyalGrimoireR5(RoyalGrimoireR5)),
    (ThrillingTalesOfDragonSlayersR5::record(), WeaponUnion::ThrillingTalesOfDragonSlayersR5(ThrillingTalesOfDragonSlayersR5::new())),
    (EyeOfPerceptionR5::record(), WeaponUnion::EyeOfPerceptionR5(EyeOfPerceptionR5::new())),
    (TheWidsithR5::record(), WeaponUnion::TheWidsithR5(TheWidsithR5::new())),
    // favonius_series
    (FavoniusGreatswordR5::record(), WeaponUnion::FavoniusGreatswordR5(FavoniusGreatswordR5::new())),
    (FavoniusSwordR5::record(), WeaponUnion::FavoniusSwordR5(FavoniusSwordR5::new())),
    (FavoniusLanceR5::record(), WeaponUnion::FavoniusLanceR5(FavoniusLanceR5::new())),
    (FavoniusWarbowR5::record(), WeaponUnion::FavoniusWarbowR5(FavoniusWarbowR5::new())),
    (FavoniusCodexR5::record(), WeaponUnion::FavoniusCodexR5(FavoniusCodexR5::new())),
    // sacrificial_series
    (SacrificialSwordR5::record(), WeaponUnion::SacrificialSwordR5(SacrificialSwordR5::new())),
    (SacrificialGreatswordR5::record(), WeaponUnion::SacrificialGreatswordR5(SacrificialGreatswordR5::new())),
    (SacrificialBowR5::record(), WeaponUnion::SacrificialBowR5(SacrificialBowR5::new())),
    (SacrificialFragmentsR5::record(), WeaponUnion::SacrificialFragmentsR5(SacrificialFragmentsR5::new())),
    // version_1_5star
    (SkywardBlade::record(1), WeaponUnion::SkywardBlade(SkywardBlade::new(1))),
    (AquilaFavonia::record(1), WeaponUnion::AquilaFavonia(AquilaFavonia::new(1))),
    (SkywardPride::record(1), WeaponUnion::SkywardPride(SkywardPride::new(1))),
    (WolfsGravestone::record(1), WeaponUnion::WolfsGravestone(WolfsGravestone)),
    (SkywardSpine::record(1), WeaponUnion::SkywardSpine(SkywardSpine::new(1))),
    (PrimordialJadeWingedSpear::record(1), WeaponUnion::PrimordialJadeWingedSpear(PrimordialJadeWingedSpear::new(1))),
    (SkywardHarp::record(1), WeaponUnion::SkywardHarp(SkywardHarp::new(1))),
    (AmosBow::record(1), WeaponUnion::AmosBow(AmosBow)),
    (SkywardAtlas::record(1), WeaponUnion::SkywardAtlas(SkywardAtlas::new(1))),
    (LostPrayerToTheSacredWinds::record(1), WeaponUnion::LostPrayerToTheSacredWinds(LostPrayerToTheSacredWinds::new(1))),
    // version_1_1
    (TheUnforged::record(1), WeaponUnion::TheUnforged(TheUnforged::new(1))),
    (SummitShaper::record(1), WeaponUnion::SummitShaper(SummitShaper::new(1))),
    (VortexVanquisher::record(1), WeaponUnion::VortexVanquisher(VortexVanquisher::new(1))),
    (MemoryOfDust::record(1), WeaponUnion::MemoryOfDust(MemoryOfDust::new(1))),
    (MemoryOfDust::record(4), WeaponUnion::MemoryOfDust(MemoryOfDust::new(4))),
    // version_1_2
    (FesteringDesire::record(), WeaponUnion::FesteringDesire(FesteringDesire::new())),
    (SnowTombedStarsilver::record(), WeaponUnion::SnowTombedStarsilver(SnowTombedStarsilver::new())),
    (DragonspineSpear::record(), WeaponUnion::DragonspineSpear(DragonspineSpear::new())),
    (Frostbearer::record(), WeaponUnion::Frostbearer(Frostbearer::new())),
    // version_1_3
    (PrimordialJadeCutter::record(), WeaponUnion::PrimordialJadeCutter(PrimordialJadeCutter::new())),
    (PrimordialJadeGS::record(), WeaponUnion::PrimordialJadeGS(PrimordialJadeGS::new())),
    (PrimordialJadeVista::record(), WeaponUnion::PrimordialJadeVista(PrimordialJadeVista::new())),
    (StaffOfHoma::record(), WeaponUnion::StaffOfHoma(StaffOfHoma::new())),
    (LithicSpear::record(), WeaponUnion::LithicSpear(LithicSpear)),
    (LithicBlade::record(), WeaponUnion::LithicBlade(LithicBlade)),
    // version_1_4
    (ElegyForTheEnd::record(), WeaponUnion::ElegyForTheEnd(ElegyForTheEnd::new())),
    (TheAlleyFlash::record(), WeaponUnion::TheAlleyFlash(TheAlleyFlash)),
    (AlleyHunter::record(), WeaponUnion::AlleyHunter(AlleyHunter::new())),
    (WineAndSong::record(), WeaponUnion::WineAndSong(WineAndSong)),
    (WindblumeOde::record(), WeaponUnion::WindblumeOde(WindblumeOde::new())),
    // version_1_5
    (SongOfBrokenPines::record(), WeaponUnion::SongOfBrokenPines(SongOfBrokenPines::new())),
    // version_1_6
    (FreedomSworn::record(), WeaponUnion::FreedomSworn(FreedomSworn::new())),
    (MitternachtsWaltz::record(), WeaponUnion::MitternachtsWaltz(MitternachtsWaltz::new())),
    (DodocoTales::record(), WeaponUnion::DodocoTales(DodocoTales::new())),
    // version_2_0
    (MistsplitterReforged::record(), WeaponUnion::MistsplitterReforged(MistsplitterReforged::new())),
    (MistsplitterReforgedClaymore::record(), WeaponUnion::MistsplitterReforgedClaymore(MistsplitterReforgedClaymore::new())),
    (MistsplitterReforgedPolearm::record(), WeaponUnion::MistsplitterReforgedPolearm(MistsplitterReforgedPolearm::new())),
    (MistsplitterReforgedBow::record(), WeaponUnion::MistsplitterReforgedBow(MistsplitterReforgedBow::new())),
    (MistsplitterReforgedCatalyst::record(), WeaponUnion::MistsplitterReforgedCatalyst(MistsplitterReforgedCatalyst::new())),
    (ThunderingPulse::record(), WeaponUnion::ThunderingPulse(ThunderingPulse::new())),
    (AmenomaKageuchi::record(), WeaponUnion::AmenomaKageuchi(AmenomaKageuchi::new())),
    (KatsuragikiriNagamasa::record(), WeaponUnion::KatsuragikiriNagamasa(KatsuragikiriNagamasa::new())),
    (KitainCrossSpear::record(), WeaponUnion::KitainCrossSpear(KitainCrossSpear::new())),
    (Hamayumi::record(), WeaponUnion::Hamayumi(Hamayumi::new())),
    (HakushinRing::record(), WeaponUnion::HakushinRing(HakushinRing::new())),
    // version_2_1
    (EngulfingLightning::record(), WeaponUnion::EngulfingLightning(EngulfingLightning::new())),
    (EverlastingMoonglow::record(), WeaponUnion::EverlastingMoonglow(EverlastingMoonglow::new())),
    (LuxuriousSeaLord::record(), WeaponUnion::LuxuriousSeaLord(LuxuriousSeaLord::new())),
    (TheCatch::record(), WeaponUnion::TheCatch(TheCatch::new())),
    // version_2_2
    (PolarStar::record(), WeaponUnion::PolarStar(PolarStar::new())),
    (PolarStarSword::record(), WeaponUnion::PolarStarSword(PolarStarSword::new())),
    (PolarStarClaymore::record(), WeaponUnion::PolarStarClaymore(PolarStarClaymore::new())),
    (PolarStarPolearm::record(), WeaponUnion::PolarStarPolearm(PolarStarPolearm::new())),
    (PolarStarCatalyst::record(), WeaponUnion::PolarStarCatalyst(PolarStarCatalyst::new())),
    (Akuoumaru::record(), WeaponUnion::Akuoumaru(Akuoumaru::new())),
    (MouunsMoon::record(), WeaponUnion::MouunsMoon(MouunsMoon::new())),
    (WavebreakersFin::record(), WeaponUnion::WavebreakersFin(WavebreakersFin::new())),
    // version_2_3
    (RedhornStonethresher::record(), WeaponUnion::RedhornStonethresher(RedhornStonethresher::new())),
    (CinnabarSpindle::record(), WeaponUnion::CinnabarSpindle(CinnabarSpindle::new())),
    // version_2_4
    (CalamityQueller::record(), WeaponUnion::CalamityQueller(CalamityQueller::new())),
    // version_2_5
    (KagurasVerity::record(), WeaponUnion::KagurasVerity(KagurasVerity::new())),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::sim2::testutil;
    use crate::sim2::testutil::{Sim2TestCharacter, NoopTimeline};
    use crate::sim2::simulate;
    use crate::sim2::simulate::History;
    use crate::sim2::element::{ElementalGauge, ElementalGaugeDecay};
    use crate::sim2::types::{CharacterAction, Vision};
    use crate::sim2::attack::{DamageResultUtil};
    use crate::sim2::timeline::{ActionState, Timeline};
    use crate::sim2::record::{WeaponRecord, Artifact, CharacterData, FieldMember, TimelineMember, Enemy};

    use Vision::*;

    // Note that Test disables each weapon's main stats

    #[test]
    fn prototype_rancour() {
        let mut history = testutil::history_7at02();
        let mut enemy = Enemy::simple();
        let mut character = Sim2TestCharacter::new();
        let mut weapon = PrototypeRancourR5::new();
        let mut artifact = Artifact::default();
        let mut members = [FieldMember {
            character: &mut character,
            weapon: &mut weapon,
            artifact: &mut artifact,
        }; 1];
        let cr = Sim2TestCharacter::record(Pyro);
        let wr = WeaponRecord::default();
        let ar = Artifact::default();
        let mut data = [CharacterData::new(0, &cr, &wr, &ar); 1];
        let dmg = simulate::calculate_damage(&mut history, &mut members, &mut data, &mut enemy).total_damage();
        let expect = (
            // na
            14.*100.*1.32 + 100.*1.24 + 100.*1.16 + 100.*1.08 +
            // skill
            200. + 200.*1.32 +
            // burst
            1.*300.
        );
        assert_eq!(dmg, expect);
    }

    #[test]
    fn the_flute() {
        let mut history = testutil::history_7at02();
        let mut enemy = Enemy::simple();
        let mut character = Sim2TestCharacter::new();
        let mut weapon = TheFluteR5::new();
        let mut artifact = Artifact::default();
        let mut members = [FieldMember {
            character: &mut character,
            weapon: &mut weapon,
            artifact: &mut artifact,
        }; 1];
        let cr = Sim2TestCharacter::record(Pyro);
        let wr = WeaponRecord::default();
        let ar = Artifact::default();
        let mut data = [CharacterData::new(0, &cr, &wr, &ar); 1];
        let dmg = simulate::calculate_damage(&mut history, &mut members, &mut data, &mut enemy).total_damage();
        let expect = (
            // flute
            200. +
            // na
            17.*100. +
            // skill
            2.*200. +
            // burst
            1.*300.
        );
        assert_eq!(dmg, expect);
    }

    #[test]
    fn prototype_archaic() {
        let mut history = testutil::history_7at02();
        let mut enemy = Enemy::simple();
        let mut character = Sim2TestCharacter::new();
        let mut weapon = PrototypeArchaicR5::new();
        let mut artifact = Artifact::default();
        let mut members = [FieldMember {
            character: &mut character,
            weapon: &mut weapon,
            artifact: &mut artifact,
        }; 1];
        let cr = Sim2TestCharacter::record(Pyro);
        let wr = WeaponRecord::default();
        let ar = Artifact::default();
        let mut data = [CharacterData::new(0, &cr, &wr, &ar); 1];
        let dmg = simulate::calculate_damage(&mut history, &mut members, &mut data, &mut enemy).total_damage();
        let expect = (
            // prototype_archaic
            480. +
            // na
            17.*100. +
            // skill
            2.*200. +
            // burst
            1.*300.
        );
        assert_eq!(dmg, expect);
    }

    #[test]
    fn prototype_archaic_and_buff() {
        let mut history = testutil::history_7at02();
        let mut enemy = Enemy::simple();
        let mut character = Sim2TestCharacter::new();
        let mut weapon = PrototypeArchaicR5::new();
        let mut artifact = Artifact::default();
        let mut members = [FieldMember {
            character: &mut character,
            weapon: &mut weapon,
            artifact: &mut artifact,
        }; 1];
        let cr = Sim2TestCharacter::record(Pyro);
        let wr = WeaponRecord::default();
        let ar = Artifact::default().physical_dmg(10.).na_dmg(10.);
        let mut data = [CharacterData::new(0, &cr, &wr, &ar); 1];
        let dmg = simulate::calculate_damage(&mut history, &mut members, &mut data, &mut enemy).total_damage();
        let expect = (
            // prototype_archaic
            480.*1.1 +
            // na
            17.*100.*1.2 +
            // skill
            2.*200. +
            // burst
            1.*300.
        );
        assert_eq!(dmg, expect);
    }

    #[test]
    fn skywardblade() {
        let mut history = History::<1>::new(4.0, 0.2);
        let cr = Sim2TestCharacter::record(Pyro);
        let wr = WeaponRecord::default();
        let ar = Artifact::default();
        let mut data      = [CharacterData::new(0, &cr, &wr, &ar); 1];
        let mut enemy     = Enemy::simple();
        let mut character = Sim2TestCharacter::new();
        let mut weapon    = SkywardBlade::new(1);
        let mut artifact  = Artifact::default();
        {
            let mut states = [ActionState::new(); 1];
            let mut members = [TimelineMember {
                character: &mut character,
                weapon: &mut weapon,
                artifact: &mut artifact,
            }; 1];
            states[0].energy += 40.0;
            simulate::decide_action(&mut history, &mut members, &mut states, &mut data);
            use CharacterAction::*;
            assert_eq!(history.action, vec![
                [Burst],
                [PressSkill],
                [Na1(0.0)], [StandStill],
                [Na2(0.04000002)], [StandStill],
                [Na3(0.08000004)], [StandStill],
                [Na4(0.120000035)], [StandStill],
                [Na1(0.16000006)], [StandStill],
                [Na2(0.20000008)],
                [Na3(0.0200001)], [StandStill],
                [Na4(0.06000012)], [StandStill],
                [Na1(0.10000011)], [StandStill],
                [Na2(0.14000013)]
            ]);
            // println!("{:?}", history.action);
        }
        let mut members = [FieldMember {
            character: &mut character,
            weapon: &mut weapon,
            artifact: &mut artifact,
        }; 1];
        let dmg = simulate::calculate_damage(&mut history, &mut members, &mut data, &mut enemy).total_damage();
        let expect = (
            // skywardblade
            10.*20. +
            // na
            10.*100. +
            // skill and burst
            500.
        );
        assert_eq!(dmg, expect);
    }

    #[test]
    fn songofbrokenpines() {
        let mut history = History::<1>::new(7., 0.2);
        let cr = Sim2TestCharacter::record(Pyro);
        let wr = WeaponRecord::default();
        let ar = Artifact::default();
        let mut data      = [CharacterData::new(0, &cr, &wr, &ar); 1];
        let mut enemy     = Enemy::simple();
        let mut character = Sim2TestCharacter::new();
        let mut weapon    = SongOfBrokenPines::new();
        let mut artifact  = Artifact::default();
        {
            let mut states = [ActionState::new(); 1];
            let mut members = [TimelineMember {
                character: &mut character,
                weapon: &mut weapon,
                artifact: &mut artifact,
            }; 1];
            simulate::decide_action(&mut history, &mut members, &mut states, &mut data);
        }
        weapon.reset_timeline();
        let mut members = [FieldMember {
            character: &mut character,
            weapon: &mut weapon,
            artifact: &mut artifact,
        }; 1];
        let dmg = simulate::calculate_damage(&mut history, &mut members, &mut data, &mut enemy).total_damage();
        let expect = (
            // na
            3.*100. + 15.*120. +
            // skill
            200. + 240.
        );
        assert_eq!(dmg, expect);
    }
}
