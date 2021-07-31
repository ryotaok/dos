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

use crate::fc::{FieldCharacterIndex, WeaponAbility, WeaponRecord};
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

fn field<T: WeaponAbility>(wa: T) -> (WeaponRecord, T) {
    let a = wa.record();
    (a, wa)
}

pub struct AllWeapons {
    // sword_4star
    prototyperancourr5: (WeaponRecord, PrototypeRancourR5),
    theblackswordr5: (WeaponRecord, TheBlackSwordR5),
    blackclifflongswordr5: (WeaponRecord, BlackcliffLongswordR5),
    royallongswordr5: (WeaponRecord, RoyalLongswordR5),
    harbingerofdawnr5: (WeaponRecord, HarbingerOfDawnR5),
    thefluter5: (WeaponRecord, TheFluteR5),
    lionsroarr5: (WeaponRecord, LionsRoarR5),
    // claymore_4star
    prototypearchaicr5: (WeaponRecord, PrototypeArchaicR5),
    whiteblindr5: (WeaponRecord, WhiteblindR5),
    serpentspiner5: (WeaponRecord, SerpentSpineR5),
    blackcliffslasherr5: (WeaponRecord, BlackcliffSlasherR5),
    royalgreatswordr5: (WeaponRecord, RoyalGreatswordR5),
    rainslasherr5: (WeaponRecord, RainslasherR5),
    // polearm_4star
    prototypestarglitterr5: (WeaponRecord, PrototypeStarglitterR5),
    crescentpiker5: (WeaponRecord, CrescentPikeR5),
    deathmatchr5: (WeaponRecord, DeathmatchR5),
    blackcliffpoler5: (WeaponRecord, BlackcliffPoleR5),
    royalspearr5: (WeaponRecord, RoyalSpearR5),
    whitetasselr5: (WeaponRecord, WhiteTasselR5),
    dragonsbaner5: (WeaponRecord, DragonsBaneR5),
    // bow_4star
    prototypecrescentr5: (WeaponRecord, PrototypeCrescentR5),
    compoundbowr5: (WeaponRecord, CompoundBowR5),
    theviridescenthuntr5: (WeaponRecord, TheViridescentHuntR5),
    blackcliffwarbowr5: (WeaponRecord, BlackcliffWarbowR5),
    royalbowr5: (WeaponRecord, RoyalBowR5),
    slingshotr5: (WeaponRecord, SlingshotR5),
    rustr5: (WeaponRecord, RustR5),
    thestringlessr5: (WeaponRecord, TheStringlessR5),
    // catalyst_4star
    prototypeamberr5: (WeaponRecord, PrototypeAmberR5),
    mappamarer5: (WeaponRecord, MappaMareR5),
    solarpearlr5: (WeaponRecord, SolarPearlR5),
    blackcliffagater5: (WeaponRecord, BlackcliffAgateR5),
    royalgrimoirer5: (WeaponRecord, RoyalGrimoireR5),
    thrillingtalesofdragonslayersr5: (WeaponRecord, ThrillingTalesOfDragonSlayersR5),
    eyeofperceptionr5: (WeaponRecord, EyeOfPerceptionR5),
    thewidsithr5: (WeaponRecord, TheWidsithR5),
    // favonius_series
    favoniusgreatswordr5: (WeaponRecord, FavoniusGreatswordR5),
    favoniusswordr5: (WeaponRecord, FavoniusSwordR5),
    favoniuslancer5: (WeaponRecord, FavoniusLanceR5),
    favoniuswarbowr5: (WeaponRecord, FavoniusWarbowR5),
    favoniuscodexr5: (WeaponRecord, FavoniusCodexR5),
    // sacrificial_series
    sacrificialswordr5: (WeaponRecord, SacrificialSwordR5),
    sacrificialgreatswordr5: (WeaponRecord, SacrificialGreatswordR5),
    sacrificialbowr5: (WeaponRecord, SacrificialBowR5),
    sacrificialfragmentsr5: (WeaponRecord, SacrificialFragmentsR5),
    // version_1_5star
    skywardblade: (WeaponRecord, SkywardBlade),
    aquilafavonia: (WeaponRecord, AquilaFavonia),
    skywardpride: (WeaponRecord, SkywardPride),
    wolfsgravestone: (WeaponRecord, WolfsGravestone),
    skywardspine: (WeaponRecord, SkywardSpine),
    primordialjadewingedspear: (WeaponRecord, PrimordialJadeWingedSpear),
    skywardharp: (WeaponRecord, SkywardHarp),
    amosbow: (WeaponRecord, AmosBow),
    skywardatlas: (WeaponRecord, SkywardAtlas),
    lostprayertothesacredwinds: (WeaponRecord, LostPrayerToTheSacredWinds),
    // version_1_1
    theunforged: (WeaponRecord, TheUnforged),
    summitshaper: (WeaponRecord, SummitShaper),
    vortexvanquisher: (WeaponRecord, VortexVanquisher),
    memoryofdust: (WeaponRecord, MemoryOfDust),
    // version_1_2
    festeringdesire: (WeaponRecord, FesteringDesire),
    snowtombedstarsilver: (WeaponRecord, SnowTombedStarsilver),
    dragonspinespear: (WeaponRecord, DragonspineSpear),
    frostbearer: (WeaponRecord, Frostbearer),
    // version_1_3
    primordialjadecutter: (WeaponRecord, PrimordialJadeCutter),
    primordialjadegs: (WeaponRecord, PrimordialJadeGS),
    primordialjadevista: (WeaponRecord, PrimordialJadeVista),
    staffofhoma: (WeaponRecord, StaffOfHoma),
    lithicspear: (WeaponRecord, LithicSpear),
    lithicblade: (WeaponRecord, LithicBlade),
    // version_1_4
    elegyfortheend: (WeaponRecord, ElegyForTheEnd),
    thealleyflash: (WeaponRecord, TheAlleyFlash),
    alleyhunter: (WeaponRecord, AlleyHunter),
    wineandsong: (WeaponRecord, WineAndSong),
    windblumeode: (WeaponRecord, WindblumeOde),
    // version_1_5
    songofbrokenpines: (WeaponRecord, SongOfBrokenPines),
    // version_1_6
    freedomsworn: (WeaponRecord, FreedomSworn),
    mitternachtswaltz: (WeaponRecord, MitternachtsWaltz),
    dodocotales: (WeaponRecord, DodocoTales),
    // version_2_0
    mistsplitterreforged: (WeaponRecord, MistsplitterReforged),
    thunderingpulse: (WeaponRecord, ThunderingPulse),
    amenomakageuchi: (WeaponRecord, AmenomaKageuchi),
    katsuragikirinagamasa: (WeaponRecord, KatsuragikiriNagamasa),
    kitaincrossspear: (WeaponRecord, KitainCrossSpear),
    hamayumi: (WeaponRecord, Hamayumi),
    hakushinring: (WeaponRecord, HakushinRing),
    // version_2_1
    grasscutterslight: (WeaponRecord, GrasscuttersLight),
    fumetsugekka: (WeaponRecord, FumetsuGekka),
}

impl AllWeapons {
    pub fn new(idx: FieldCharacterIndex) -> Self {
        Self {
            // sword_4star
            prototyperancourr5: field(PrototypeRancourR5::new()),
            theblackswordr5: field(TheBlackSwordR5),
            blackclifflongswordr5: field(BlackcliffLongswordR5),
            royallongswordr5: field(RoyalLongswordR5),
            harbingerofdawnr5: field(HarbingerOfDawnR5),
            thefluter5: field(TheFluteR5::new(idx)),
            lionsroarr5: field(LionsRoarR5),
            // claymore_4star
            prototypearchaicr5: field(PrototypeArchaicR5::new(idx)),
            whiteblindr5: field(WhiteblindR5::new()),
            serpentspiner5: field(SerpentSpineR5::new()),
            blackcliffslasherr5: field(BlackcliffSlasherR5),
            royalgreatswordr5: field(RoyalGreatswordR5),
            rainslasherr5: field(RainslasherR5),
            // polearm_4star
            prototypestarglitterr5: field(PrototypeStarglitterR5::new()),
            crescentpiker5: field(CrescentPikeR5::new(idx)),
            deathmatchr5: field(DeathmatchR5),
            blackcliffpoler5: field(BlackcliffPoleR5),
            royalspearr5: field(RoyalSpearR5),
            whitetasselr5: field(WhiteTasselR5),
            dragonsbaner5: field(DragonsBaneR5),
            // bow_4star
            prototypecrescentr5: field(PrototypeCrescentR5),
            compoundbowr5: field(CompoundBowR5::new()),
            theviridescenthuntr5: field(TheViridescentHuntR5::new(idx)),
            blackcliffwarbowr5: field(BlackcliffWarbowR5),
            royalbowr5: field(RoyalBowR5),
            slingshotr5: field(SlingshotR5),
            rustr5: field(RustR5),
            thestringlessr5: field(TheStringlessR5),
            // catalyst_4star
            prototypeamberr5: field(PrototypeAmberR5),
            mappamarer5: field(MappaMareR5::new()),
            solarpearlr5: field(SolarPearlR5::new()),
            blackcliffagater5: field(BlackcliffAgateR5),
            royalgrimoirer5: field(RoyalGrimoireR5),
            thrillingtalesofdragonslayersr5: field(ThrillingTalesOfDragonSlayersR5::new()),
            eyeofperceptionr5: field(EyeOfPerceptionR5::new(idx)),
            thewidsithr5: field(TheWidsithR5::new()),
            // favonius_series
            favoniusgreatswordr5: field(FavoniusGreatswordR5::new()),
            favoniusswordr5: field(FavoniusSwordR5::new()),
            favoniuslancer5: field(FavoniusLanceR5::new()),
            favoniuswarbowr5: field(FavoniusWarbowR5::new()),
            favoniuscodexr5: field(FavoniusCodexR5::new()),
            // sacrificial_series
            sacrificialswordr5: field(SacrificialSwordR5::new()),
            sacrificialgreatswordr5: field(SacrificialGreatswordR5::new()),
            sacrificialbowr5: field(SacrificialBowR5::new()),
            sacrificialfragmentsr5: field(SacrificialFragmentsR5::new()),
            // version_1_5star
            skywardblade: field(SkywardBlade::new(idx)),
            aquilafavonia: field(AquilaFavonia::new(idx)),
            skywardpride: field(SkywardPride::new(idx)),
            wolfsgravestone: field(WolfsGravestone),
            skywardspine: field(SkywardSpine::new(idx)),
            primordialjadewingedspear: field(PrimordialJadeWingedSpear::new()),
            skywardharp: field(SkywardHarp::new(idx)),
            amosbow: field(AmosBow),
            skywardatlas: field(SkywardAtlas::new(idx)),
            lostprayertothesacredwinds: field(LostPrayerToTheSacredWinds::new()),
            // version_1_1
            theunforged: field(TheUnforged::new()),
            summitshaper: field(SummitShaper::new()),
            vortexvanquisher: field(VortexVanquisher::new()),
            memoryofdust: field(MemoryOfDust::new()),
            // version_1_2
            festeringdesire: field(FesteringDesire),
            snowtombedstarsilver: field(SnowTombedStarsilver::new(idx)),
            dragonspinespear: field(DragonspineSpear::new(idx)),
            frostbearer: field(Frostbearer::new(idx)),
            // version_1_3
            primordialjadecutter: field(PrimordialJadeCutter::new()),
            primordialjadegs: field(PrimordialJadeGS::new()),
            primordialjadevista: field(PrimordialJadeVista::new()),
            staffofhoma: field(StaffOfHoma::new()),
            lithicspear: field(LithicSpear),
            lithicblade: field(LithicBlade),
            // version_1_4
            elegyfortheend: field(ElegyForTheEnd::new()),
            thealleyflash: field(TheAlleyFlash),
            alleyhunter: field(AlleyHunter::new()),
            wineandsong: field(WineAndSong),
            windblumeode: field(WindblumeOde::new()),
            // version_1_5
            songofbrokenpines: field(SongOfBrokenPines::new()),
            // version_1_6
            freedomsworn: field(FreedomSworn::new()),
            mitternachtswaltz: field(MitternachtsWaltz::new()),
            dodocotales: field(DodocoTales::new()),
            // version_2_0
            mistsplitterreforged: field(MistsplitterReforged::new()),
            thunderingpulse: field(ThunderingPulse::new()),
            amenomakageuchi: field(AmenomaKageuchi::new()),
            katsuragikirinagamasa: field(KatsuragikiriNagamasa::new()),
            kitaincrossspear: field(KitainCrossSpear::new()),
            hamayumi: field(Hamayumi),
            hakushinring: field(HakushinRing::new()),
            // version_2_1
            grasscutterslight: field(GrasscuttersLight::new()),
            fumetsugekka: field(FumetsuGekka::new()),
        }
    }

    pub fn find<'a>(&'a mut self, name: &WeaponName) -> &'a mut (WeaponRecord, dyn WeaponAbility) {
        use WeaponName::*;
        match name {
            // sword_4star
            PrototypeRancourR5 => &mut self.prototyperancourr5,
            TheBlackSwordR5 => &mut self.theblackswordr5,
            BlackcliffLongswordR5 => &mut self.blackclifflongswordr5,
            RoyalLongswordR5 => &mut self.royallongswordr5,
            HarbingerOfDawnR5 => &mut self.harbingerofdawnr5,
            TheFluteR5 => &mut self.thefluter5,
            LionsRoarR5 => &mut self.lionsroarr5,
            // claymore_4star
            PrototypeArchaicR5 => &mut self.prototypearchaicr5,
            WhiteblindR5 => &mut self.whiteblindr5,
            SerpentSpineR5 => &mut self.serpentspiner5,
            BlackcliffSlasherR5 => &mut self.blackcliffslasherr5,
            RoyalGreatswordR5 => &mut self.royalgreatswordr5,
            RainslasherR5 => &mut self.rainslasherr5,
            // polearm_4star
            PrototypeStarglitterR5 => &mut self.prototypestarglitterr5,
            CrescentPikeR5 => &mut self.crescentpiker5,
            DeathmatchR5 => &mut self.deathmatchr5,
            BlackcliffPoleR5 => &mut self.blackcliffpoler5,
            RoyalSpearR5 => &mut self.royalspearr5,
            WhiteTasselR5 => &mut self.whitetasselr5,
            DragonsBaneR5 => &mut self.dragonsbaner5,
            // bow_4star
            PrototypeCrescentR5 => &mut self.prototypecrescentr5,
            CompoundBowR5 => &mut self.compoundbowr5,
            TheViridescentHuntR5 => &mut self.theviridescenthuntr5,
            BlackcliffWarbowR5 => &mut self.blackcliffwarbowr5,
            RoyalBowR5 => &mut self.royalbowr5,
            SlingshotR5 => &mut self.slingshotr5,
            RustR5 => &mut self.rustr5,
            TheStringlessR5 => &mut self.thestringlessr5,
            // catalyst_4star
            PrototypeAmberR5 => &mut self.prototypeamberr5,
            MappaMareR5 => &mut self.mappamarer5,
            SolarPearlR5 => &mut self.solarpearlr5,
            BlackcliffAgateR5 => &mut self.blackcliffagater5,
            RoyalGrimoireR5 => &mut self.royalgrimoirer5,
            ThrillingTalesOfDragonSlayersR5 => &mut self.thrillingtalesofdragonslayersr5,
            EyeOfPerceptionR5 => &mut self.eyeofperceptionr5,
            TheWidsithR5 => &mut self.thewidsithr5,
            // favonius_series
            FavoniusGreatswordR5 => &mut self.favoniusgreatswordr5,
            FavoniusSwordR5 => &mut self.favoniusswordr5,
            FavoniusLanceR5 => &mut self.favoniuslancer5,
            FavoniusWarbowR5 => &mut self.favoniuswarbowr5,
            FavoniusCodexR5 => &mut self.favoniuscodexr5,
            // sacrificial_series
            SacrificialSwordR5 => &mut self.sacrificialswordr5,
            SacrificialGreatswordR5 => &mut self.sacrificialgreatswordr5,
            SacrificialBowR5 => &mut self.sacrificialbowr5,
            SacrificialFragmentsR5 => &mut self.sacrificialfragmentsr5,
            // version_1_5star
            SkywardBlade => &mut self.skywardblade,
            AquilaFavonia => &mut self.aquilafavonia,
            SkywardPride => &mut self.skywardpride,
            WolfsGravestone => &mut self.wolfsgravestone,
            SkywardSpine => &mut self.skywardspine,
            PrimordialJadeWingedSpear => &mut self.primordialjadewingedspear,
            SkywardHarp => &mut self.skywardharp,
            AmosBow => &mut self.amosbow,
            SkywardAtlas => &mut self.skywardatlas,
            LostPrayerToTheSacredWinds => &mut self.lostprayertothesacredwinds,
            // version_1_1
            TheUnforged => &mut self.theunforged,
            SummitShaper => &mut self.summitshaper,
            VortexVanquisher => &mut self.vortexvanquisher,
            MemoryOfDust => &mut self.memoryofdust,
            // version_1_2
            FesteringDesire => &mut self.festeringdesire,
            SnowTombedStarsilver => &mut self.snowtombedstarsilver,
            DragonspineSpear => &mut self.dragonspinespear,
            Frostbearer => &mut self.frostbearer,
            // version_1_3
            PrimordialJadeCutter => &mut self.primordialjadecutter,
            PrimordialJadeGS => &mut self.primordialjadegs,
            PrimordialJadeVista => &mut self.primordialjadevista,
            StaffOfHoma => &mut self.staffofhoma,
            LithicSpear => &mut self.lithicspear,
            LithicBlade => &mut self.lithicblade,
            // version_1_4
            ElegyForTheEnd => &mut self.elegyfortheend,
            TheAlleyFlash => &mut self.thealleyflash,
            AlleyHunter => &mut self.alleyhunter,
            WineAndSong => &mut self.wineandsong,
            WindblumeOde => &mut self.windblumeode,
            // version_1_5
            SongOfBrokenPines => &mut self.songofbrokenpines,
            // version_1_6
            FreedomSworn => &mut self.freedomsworn,
            MitternachtsWaltz => &mut self.mitternachtswaltz,
            DodocoTales => &mut self.dodocotales,
            // version_2_0
            MistsplitterReforged => &mut self.mistsplitterreforged,
            ThunderingPulse => &mut self.thunderingpulse,
            AmenomaKageuchi => &mut self.amenomakageuchi,
            KatsuragikiriNagamasa => &mut self.katsuragikirinagamasa,
            KitainCrossSpear => &mut self.kitaincrossspear,
            Hamayumi => &mut self.hamayumi,
            HakushinRing => &mut self.hakushinring,
            // version_2_1
            GrasscuttersLight => &mut self.grasscutterslight,
            FumetsuGekka => &mut self.fumetsugekka,
        }
    }
}

#[derive(Debug)]
pub enum WeaponName {
    // sword_4star
    PrototypeRancourR5,
    TheBlackSwordR5,
    BlackcliffLongswordR5,
    RoyalLongswordR5,
    HarbingerOfDawnR5,
    TheFluteR5,
    LionsRoarR5,
    // claymore_4star
    PrototypeArchaicR5,
    WhiteblindR5,
    SerpentSpineR5,
    BlackcliffSlasherR5,
    RoyalGreatswordR5,
    RainslasherR5,
    // polearm_4star
    PrototypeStarglitterR5,
    CrescentPikeR5,
    DeathmatchR5,
    BlackcliffPoleR5,
    RoyalSpearR5,
    WhiteTasselR5,
    DragonsBaneR5,
    // bow_4star
    PrototypeCrescentR5,
    CompoundBowR5,
    TheViridescentHuntR5,
    BlackcliffWarbowR5,
    RoyalBowR5,
    SlingshotR5,
    RustR5,
    TheStringlessR5,
    // catalyst_4star
    PrototypeAmberR5,
    MappaMareR5,
    SolarPearlR5,
    BlackcliffAgateR5,
    RoyalGrimoireR5,
    ThrillingTalesOfDragonSlayersR5,
    EyeOfPerceptionR5,
    TheWidsithR5,
    // favonius_series
    FavoniusGreatswordR5,
    FavoniusSwordR5,
    FavoniusLanceR5,
    FavoniusWarbowR5,
    FavoniusCodexR5,
    // sacrificial_series
    SacrificialSwordR5,
    SacrificialGreatswordR5,
    SacrificialBowR5,
    SacrificialFragmentsR5,
    // version_1_5star
    SkywardBlade,
    AquilaFavonia,
    SkywardPride,
    WolfsGravestone,
    SkywardSpine,
    PrimordialJadeWingedSpear,
    SkywardHarp,
    AmosBow,
    SkywardAtlas,
    LostPrayerToTheSacredWinds,
    // version_1_1
    TheUnforged,
    SummitShaper,
    VortexVanquisher,
    MemoryOfDust,
    // version_1_2
    FesteringDesire,
    SnowTombedStarsilver,
    DragonspineSpear,
    Frostbearer,
    // version_1_3
    PrimordialJadeCutter,
    PrimordialJadeGS,
    PrimordialJadeVista,
    StaffOfHoma,
    LithicSpear,
    LithicBlade,
    // version_1_4
    ElegyForTheEnd,
    TheAlleyFlash,
    AlleyHunter,
    WineAndSong,
    WindblumeOde,
    // version_1_5
    SongOfBrokenPines,
    // version_1_6
    FreedomSworn,
    MitternachtsWaltz,
    DodocoTales,
    // version_2_0
    MistsplitterReforged,
    ThunderingPulse,
    AmenomaKageuchi,
    KatsuragikiriNagamasa,
    KitainCrossSpear,
    Hamayumi,
    HakushinRing,
    // version_2_1
    GrasscuttersLight,
    FumetsuGekka,
}

impl WeaponName {
    pub fn vec() -> Vec<WeaponName> {
        use WeaponName::*;
        vec![
    // sword_4star
    PrototypeRancourR5,
    TheBlackSwordR5,
    BlackcliffLongswordR5,
    RoyalLongswordR5,
    HarbingerOfDawnR5,
    TheFluteR5,
    LionsRoarR5,
    // claymore_4star
    PrototypeArchaicR5,
    WhiteblindR5,
    SerpentSpineR5,
    BlackcliffSlasherR5,
    RoyalGreatswordR5,
    RainslasherR5,
    // polearm_4star
    PrototypeStarglitterR5,
    CrescentPikeR5,
    DeathmatchR5,
    BlackcliffPoleR5,
    RoyalSpearR5,
    WhiteTasselR5,
    DragonsBaneR5,
    // bow_4star
    PrototypeCrescentR5,
    CompoundBowR5,
    TheViridescentHuntR5,
    BlackcliffWarbowR5,
    RoyalBowR5,
    SlingshotR5,
    RustR5,
    TheStringlessR5,
    // catalyst_4star
    PrototypeAmberR5,
    MappaMareR5,
    SolarPearlR5,
    BlackcliffAgateR5,
    RoyalGrimoireR5,
    ThrillingTalesOfDragonSlayersR5,
    EyeOfPerceptionR5,
    TheWidsithR5,
    // favonius_series
    FavoniusGreatswordR5,
    FavoniusSwordR5,
    FavoniusLanceR5,
    FavoniusWarbowR5,
    FavoniusCodexR5,
    // sacrificial_series
    SacrificialSwordR5,
    SacrificialGreatswordR5,
    SacrificialBowR5,
    SacrificialFragmentsR5,
    // version_1_5star
    SkywardBlade,
    AquilaFavonia,
    SkywardPride,
    WolfsGravestone,
    SkywardSpine,
    PrimordialJadeWingedSpear,
    SkywardHarp,
    AmosBow,
    SkywardAtlas,
    LostPrayerToTheSacredWinds,
    // version_1_1
    TheUnforged,
    SummitShaper,
    VortexVanquisher,
    MemoryOfDust,
    // version_1_2
    FesteringDesire,
    SnowTombedStarsilver,
    DragonspineSpear,
    Frostbearer,
    // version_1_3
    PrimordialJadeCutter,
    PrimordialJadeGS,
    PrimordialJadeVista,
    StaffOfHoma,
    LithicSpear,
    LithicBlade,
    // version_1_4
    ElegyForTheEnd,
    TheAlleyFlash,
    AlleyHunter,
    WineAndSong,
    WindblumeOde,
    // version_1_5
    SongOfBrokenPines,
    // version_1_6
    FreedomSworn,
    MitternachtsWaltz,
    DodocoTales,
    // version_2_0
    MistsplitterReforged,
    ThunderingPulse,
    AmenomaKageuchi,
    KatsuragikiriNagamasa,
    KitainCrossSpear,
    Hamayumi,
    HakushinRing,
    // version_2_1
    GrasscuttersLight,
    FumetsuGekka,
        ]
    }
}

impl<'a> From<&'a str> for WeaponName {
    fn from(name: &'a str) -> Self {
        use WeaponName::*;
        match name {
            // sword_4star
            "PrototypeRancourR5" => PrototypeRancourR5,
            "TheBlackSwordR5" => TheBlackSwordR5,
            "BlackcliffLongswordR5" => BlackcliffLongswordR5,
            "RoyalLongswordR5" => RoyalLongswordR5,
            "HarbingerOfDawnR5" => HarbingerOfDawnR5,
            "TheFluteR5" => TheFluteR5,
            "LionsRoarR5" => LionsRoarR5,
            // claymore_4star
            "PrototypeArchaicR5" => PrototypeArchaicR5,
            "WhiteblindR5" => WhiteblindR5,
            "SerpentSpineR5" => SerpentSpineR5,
            "BlackcliffSlasherR5" => BlackcliffSlasherR5,
            "RoyalGreatswordR5" => RoyalGreatswordR5,
            "RainslasherR5" => RainslasherR5,
            // polearm_4star
            "PrototypeStarglitterR5" => PrototypeStarglitterR5,
            "CrescentPikeR5" => CrescentPikeR5,
            "DeathmatchR5" => DeathmatchR5,
            "BlackcliffPoleR5" => BlackcliffPoleR5,
            "RoyalSpearR5" => RoyalSpearR5,
            "WhiteTasselR5" => WhiteTasselR5,
            "DragonsBaneR5" => DragonsBaneR5,
            // bow_4star
            "PrototypeCrescentR5" => PrototypeCrescentR5,
            "CompoundBowR5" => CompoundBowR5,
            "TheViridescentHuntR5" => TheViridescentHuntR5,
            "BlackcliffWarbowR5" => BlackcliffWarbowR5,
            "RoyalBowR5" => RoyalBowR5,
            "SlingshotR5" => SlingshotR5,
            "RustR5" => RustR5,
            "TheStringlessR5" => TheStringlessR5,
            // catalyst_4star
            "PrototypeAmberR5" => PrototypeAmberR5,
            "MappaMareR5" => MappaMareR5,
            "SolarPearlR5" => SolarPearlR5,
            "BlackcliffAgateR5" => BlackcliffAgateR5,
            "RoyalGrimoireR5" => RoyalGrimoireR5,
            "ThrillingTalesOfDragonSlayersR5" => ThrillingTalesOfDragonSlayersR5,
            "EyeOfPerceptionR5" => EyeOfPerceptionR5,
            "TheWidsithR5" => TheWidsithR5,
            // favonius_series
            "FavoniusGreatswordR5" => FavoniusGreatswordR5,
            "FavoniusSwordR5" => FavoniusSwordR5,
            "FavoniusLanceR5" => FavoniusLanceR5,
            "FavoniusWarbowR5" => FavoniusWarbowR5,
            "FavoniusCodexR5" => FavoniusCodexR5,
            // sacrificial_series
            "SacrificialSwordR5" => SacrificialSwordR5,
            "SacrificialGreatswordR5" => SacrificialGreatswordR5,
            "SacrificialBowR5" => SacrificialBowR5,
            "SacrificialFragmentsR5" => SacrificialFragmentsR5,
            // version_1_5star
            "SkywardBlade" => SkywardBlade,
            "AquilaFavonia" => AquilaFavonia,
            "SkywardPride" => SkywardPride,
            "WolfsGravestone" => WolfsGravestone,
            "SkywardSpine" => SkywardSpine,
            "PrimordialJadeWingedSpear" => PrimordialJadeWingedSpear,
            "SkywardHarp" => SkywardHarp,
            "AmosBow" => AmosBow,
            "SkywardAtlas" => SkywardAtlas,
            "LostPrayerToTheSacredWinds" => LostPrayerToTheSacredWinds,
            // version_1_1
            "TheUnforged" => TheUnforged,
            "SummitShaper" => SummitShaper,
            "VortexVanquisher" => VortexVanquisher,
            "MemoryOfDust" => MemoryOfDust,
            // version_1_2
            "FesteringDesire" => FesteringDesire,
            "SnowTombedStarsilver" => SnowTombedStarsilver,
            "DragonspineSpear" => DragonspineSpear,
            "Frostbearer" => Frostbearer,
            // version_1_3
            "PrimordialJadeCutter" => PrimordialJadeCutter,
            "PrimordialJadeGS" => PrimordialJadeGS,
            "PrimordialJadeVista" => PrimordialJadeVista,
            "StaffOfHoma" => StaffOfHoma,
            "LithicSpear" => LithicSpear,
            "LithicBlade" => LithicBlade,
            // version_1_4
            "ElegyForTheEnd" => ElegyForTheEnd,
            "TheAlleyFlash" => TheAlleyFlash,
            "AlleyHunter" => AlleyHunter,
            "WineAndSong" => WineAndSong,
            "WindblumeOde" => WindblumeOde,
            // version_1_5
            "SongOfBrokenPines" => SongOfBrokenPines,
            // version_1_6
            "FreedomSworn" => FreedomSworn,
            "MitternachtsWaltz" => MitternachtsWaltz,
            "DodocoTales" => DodocoTales,
            // version_2_0
            "MistsplitterReforged" => MistsplitterReforged,
            "ThunderingPulse" => ThunderingPulse,
            "AmenomaKageuchi" => AmenomaKageuchi,
            "KatsuragikiriNagamasa" => KatsuragikiriNagamasa,
            "KitainCrossSpear" => KitainCrossSpear,
            "Hamayumi" => Hamayumi,
            "HakushinRing" => HakushinRing,
            // version_2_1
            "GrasscuttersLight" => GrasscuttersLight,
            "FumetsuGekka" => FumetsuGekka,
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::state::State;
    use crate::types::Vision;
    use crate::simulate::simulate;
    use crate::fc::{FieldCharacterIndex};
    use crate::testutil::{TestEnvironment, TestWeaponAbility};

    use Vision::*;

    // Note that TestWeaponAbility disables each weapon's main stats

    #[test]
    fn prototype_rancour() {
        let idx = FieldCharacterIndex(0);
        let mut env = TestEnvironment::new();
        let mut wa = TestWeaponAbility(PrototypeRancourR5::new());
        let mut members = vec![
            env.weapon(idx, State::new(), Pyro, &mut wa)
        ];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // skill na na na
        let expect = 200.0 + 108.0 + 116.0 + 124.0 + 132.0;
        assert_eq!(total_dmg, 0.5 * expect);
    }

    #[test]
    fn the_flute() {
        let idx = FieldCharacterIndex(0);
        let mut env = TestEnvironment::new();
        let mut wa = TestWeaponAbility(TheFluteR5::new(idx));
        let mut members = vec![
            env.no_skill_weapon(idx, State::new(), Pyro, &mut wa)
        ];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..41 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // 20 na, 2 flute
        let expect = 20.0 * 100.0 + 2.0 * 200.0;
        assert_eq!(total_dmg, 0.5 * expect);
    }

    #[test]
    fn prototype_archaic() {
        let idx = FieldCharacterIndex(0);
        let mut env = TestEnvironment::new();
        let mut wa = TestWeaponAbility(PrototypeArchaicR5::new(idx));
        let mut members = vec![
            env.weapon(idx, State::new(), Pyro, &mut wa)
        ];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // skill na na na na (prototype_archaic)
        let expect = 200.0 + 4.0 * 100.0 + 480.0;
        assert_eq!(total_dmg, 0.5 * expect);
    }

    #[test]
    fn prototype_archaic_physical() {
        let idx = FieldCharacterIndex(0);
        let mut env = TestEnvironment::new();
        let mut wa = TestWeaponAbility(PrototypeArchaicR5::new(idx));
        let mut members = vec![
            env.weapon(idx, State::new().physical_dmg(10.0), Pyro, &mut wa)
        ];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // skill na na na na (prototype_archaic)
        let expect = 200.0 + 4.0 * 110.0 + 480.0 * 1.1;
        assert_eq!(total_dmg, 0.5 * expect);
    }

    #[test]
    fn prototype_archaic_na() {
        let idx = FieldCharacterIndex(0);
        let mut env = TestEnvironment::new();
        let mut wa = TestWeaponAbility(PrototypeArchaicR5::new(idx));
        let mut members = vec![
            env.weapon(idx, State::new().na_dmg(10.0), Pyro, &mut wa)
        ];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // skill na na na (prototype_archaic)
        let expect = 200.0 + 4.0 * 110.0 + 480.0;
        assert_eq!(total_dmg, 0.5 * expect);
    }

    #[test]
    fn prototype_archaic_cd() {
        let idx = FieldCharacterIndex(0);
        let mut env = TestEnvironment::new();
        let mut wa = TestWeaponAbility(PrototypeArchaicR5::new(idx));
        let mut members = vec![
            env.no_skill_weapon(idx, State::new(), Pyro, &mut wa)
        ];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..5 {
            total_dmg += simulate(&mut members, &mut enemy, 10.0);
        }
        // na na (prototype_archaic) na na (prototype_archaic)
        let expect = 100.0 + 100.0 + 480.0 + 100.0 + 100.0 + 480.0;
        assert_eq!(total_dmg, 0.5 * expect);
    }

    #[test]
    fn skywardblade() {
        let idx = FieldCharacterIndex(0);
        let mut env = TestEnvironment::new();
        let mut wa = TestWeaponAbility(SkywardBlade::new(idx));
        let mut members = vec![
            env.no_skill_weapon(idx, State::new(), Pyro, &mut wa)
        ];
        members[0].fc.data.state.energy = members[0].fc.data.cr.energy_cost;
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(&mut members, &mut enemy, 0.2);
        }
        // burst na na na na
        let expect = 300.0 + 4.0 * 120.0;
        assert_eq!(total_dmg, 0.5 * expect);
    }

    #[test]
    fn songofbrokenpines() {
        let idx = FieldCharacterIndex(0);
        let mut env = TestEnvironment::new();
        let mut wa = TestWeaponAbility(SongOfBrokenPines::new());
        let mut members = vec![
            env.no_skill_weapon(idx, State::new(), Pyro, &mut wa)
        ];
        let mut enemy = TestEnvironment::enemy();
        let mut total_dmg = 0.0;
        for _ in 0..21 {
            total_dmg += simulate(&mut members, &mut enemy, 1.0);
        }
        let expect = 9.0 * 100.0 + 11.0 * 120.0;
        assert_eq!(total_dmg, 0.5 * expect);
    }
}
