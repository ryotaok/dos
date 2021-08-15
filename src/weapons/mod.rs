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

use crate::fc::{FieldCharacterIndex, WeaponRecord, SpecialAbility, FieldAbilityBuilder};
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
    pub fn new(idx: FieldCharacterIndex, icd_timer: &Rc<RefCell<ICDTimer>>) -> Self {
        Self {
            // sword_4star
            prototyperancourr5: (PrototypeRancourR5::record(), PrototypeRancourR5::new()),
            theblackswordr5: (TheBlackSwordR5::record(), TheBlackSwordR5),
            blackclifflongswordr5: (BlackcliffLongswordR5::record(), BlackcliffLongswordR5),
            royallongswordr5: (RoyalLongswordR5::record(), RoyalLongswordR5),
            harbingerofdawnr5: (HarbingerOfDawnR5::record(), HarbingerOfDawnR5),
            thefluter5: (TheFluteR5::record(), TheFluteR5::new(idx, icd_timer)),
            lionsroarr5: (LionsRoarR5::record(), LionsRoarR5),
            // claymore_4star
            prototypearchaicr5: (PrototypeArchaicR5::record(), PrototypeArchaicR5::new(idx, icd_timer)),
            whiteblindr5: (WhiteblindR5::record(), WhiteblindR5::new()),
            serpentspiner5: (SerpentSpineR5::record(), SerpentSpineR5::new()),
            blackcliffslasherr5: (BlackcliffSlasherR5::record(), BlackcliffSlasherR5),
            royalgreatswordr5: (RoyalGreatswordR5::record(), RoyalGreatswordR5),
            rainslasherr5: (RainslasherR5::record(), RainslasherR5),
            // polearm_4star
            prototypestarglitterr5: (PrototypeStarglitterR5::record(), PrototypeStarglitterR5::new()),
            crescentpiker5: (CrescentPikeR5::record(), CrescentPikeR5::new(idx, icd_timer)),
            deathmatchr5: (DeathmatchR5::record(), DeathmatchR5),
            blackcliffpoler5: (BlackcliffPoleR5::record(), BlackcliffPoleR5),
            royalspearr5: (RoyalSpearR5::record(), RoyalSpearR5),
            whitetasselr5: (WhiteTasselR5::record(), WhiteTasselR5),
            dragonsbaner5: (DragonsBaneR5::record(), DragonsBaneR5),
            // bow_4star
            prototypecrescentr5: (PrototypeCrescentR5::record(), PrototypeCrescentR5),
            compoundbowr5: (CompoundBowR5::record(), CompoundBowR5::new()),
            theviridescenthuntr5: (TheViridescentHuntR5::record(), TheViridescentHuntR5::new(idx, icd_timer)),
            blackcliffwarbowr5: (BlackcliffWarbowR5::record(), BlackcliffWarbowR5),
            royalbowr5: (RoyalBowR5::record(), RoyalBowR5),
            slingshotr5: (SlingshotR5::record(), SlingshotR5),
            rustr5: (RustR5::record(), RustR5),
            thestringlessr5: (TheStringlessR5::record(), TheStringlessR5),
            // catalyst_4star
            prototypeamberr5: (PrototypeAmberR5::record(), PrototypeAmberR5),
            mappamarer5: (MappaMareR5::record(), MappaMareR5::new()),
            solarpearlr5: (SolarPearlR5::record(), SolarPearlR5::new()),
            blackcliffagater5: (BlackcliffAgateR5::record(), BlackcliffAgateR5),
            royalgrimoirer5: (RoyalGrimoireR5::record(), RoyalGrimoireR5),
            thrillingtalesofdragonslayersr5: (ThrillingTalesOfDragonSlayersR5::record(), ThrillingTalesOfDragonSlayersR5::new()),
            eyeofperceptionr5: (EyeOfPerceptionR5::record(), EyeOfPerceptionR5::new(idx, icd_timer)),
            thewidsithr5: (TheWidsithR5::record(), TheWidsithR5::new()),
            // favonius_series
            favoniusgreatswordr5: (FavoniusGreatswordR5::record(), FavoniusGreatswordR5::new()),
            favoniusswordr5: (FavoniusSwordR5::record(), FavoniusSwordR5::new()),
            favoniuslancer5: (FavoniusLanceR5::record(), FavoniusLanceR5::new()),
            favoniuswarbowr5: (FavoniusWarbowR5::record(), FavoniusWarbowR5::new()),
            favoniuscodexr5: (FavoniusCodexR5::record(), FavoniusCodexR5::new()),
            // sacrificial_series
            sacrificialswordr5: (SacrificialSwordR5::record(), SacrificialSwordR5::new()),
            sacrificialgreatswordr5: (SacrificialGreatswordR5::record(), SacrificialGreatswordR5::new()),
            sacrificialbowr5: (SacrificialBowR5::record(), SacrificialBowR5::new()),
            sacrificialfragmentsr5: (SacrificialFragmentsR5::record(), SacrificialFragmentsR5::new()),
            // version_1_5star
            skywardblade: (SkywardBlade::record(), SkywardBlade::new(idx, icd_timer)),
            aquilafavonia: (AquilaFavonia::record(), AquilaFavonia::new(idx, icd_timer)),
            skywardpride: (SkywardPride::record(), SkywardPride::new(idx, icd_timer)),
            wolfsgravestone: (WolfsGravestone::record(), WolfsGravestone),
            skywardspine: (SkywardSpine::record(), SkywardSpine::new(idx, icd_timer)),
            primordialjadewingedspear: (PrimordialJadeWingedSpear::record(), PrimordialJadeWingedSpear::new()),
            skywardharp: (SkywardHarp::record(), SkywardHarp::new(idx, icd_timer)),
            amosbow: (AmosBow::record(), AmosBow),
            skywardatlas: (SkywardAtlas::record(), SkywardAtlas::new(idx, icd_timer)),
            lostprayertothesacredwinds: (LostPrayerToTheSacredWinds::record(), LostPrayerToTheSacredWinds::new()),
            // version_1_1
            theunforged: (TheUnforged::record(), TheUnforged::new(idx, icd_timer)),
            summitshaper: (SummitShaper::record(), SummitShaper::new(idx, icd_timer)),
            vortexvanquisher: (VortexVanquisher::record(), VortexVanquisher::new(idx, icd_timer)),
            memoryofdust: (MemoryOfDust::record(), MemoryOfDust::new(idx, icd_timer)),
            // version_1_2
            festeringdesire: (FesteringDesire::record(), FesteringDesire::new(idx, icd_timer)),
            snowtombedstarsilver: (SnowTombedStarsilver::record(), SnowTombedStarsilver::new(idx, icd_timer)),
            dragonspinespear: (DragonspineSpear::record(), DragonspineSpear::new(idx, icd_timer)),
            frostbearer: (Frostbearer::record(), Frostbearer::new(idx, icd_timer)),
            // version_1_3
            primordialjadecutter: (PrimordialJadeCutter::record(), PrimordialJadeCutter::new()),
            primordialjadegs: (PrimordialJadeGS::record(), PrimordialJadeGS::new()),
            primordialjadevista: (PrimordialJadeVista::record(), PrimordialJadeVista::new()),
            staffofhoma: (StaffOfHoma::record(), StaffOfHoma::new()),
            lithicspear: (LithicSpear::record(), LithicSpear),
            lithicblade: (LithicBlade::record(), LithicBlade),
            // version_1_4
            elegyfortheend: (ElegyForTheEnd::record(), ElegyForTheEnd::new()),
            thealleyflash: (TheAlleyFlash::record(), TheAlleyFlash),
            alleyhunter: (AlleyHunter::record(), AlleyHunter::new()),
            wineandsong: (WineAndSong::record(), WineAndSong),
            windblumeode: (WindblumeOde::record(), WindblumeOde::new()),
            // version_1_5
            songofbrokenpines: (SongOfBrokenPines::record(), SongOfBrokenPines::new()),
            // version_1_6
            freedomsworn: (FreedomSworn::record(), FreedomSworn::new()),
            mitternachtswaltz: (MitternachtsWaltz::record(), MitternachtsWaltz::new()),
            dodocotales: (DodocoTales::record(), DodocoTales::new()),
            // version_2_0
            mistsplitterreforged: (MistsplitterReforged::record(), MistsplitterReforged::new()),
            thunderingpulse: (ThunderingPulse::record(), ThunderingPulse::new()),
            amenomakageuchi: (AmenomaKageuchi::record(), AmenomaKageuchi::new()),
            katsuragikirinagamasa: (KatsuragikiriNagamasa::record(), KatsuragikiriNagamasa::new()),
            kitaincrossspear: (KitainCrossSpear::record(), KitainCrossSpear::new()),
            hamayumi: (Hamayumi::record(), Hamayumi::new()),
            hakushinring: (HakushinRing::record(), HakushinRing::new()),
            // version_2_1
            grasscutterslight: (GrasscuttersLight::record(), GrasscuttersLight::new()),
            fumetsugekka: (FumetsuGekka::record(), FumetsuGekka::new()),
        }
    }

    pub fn find<'a>(&'a mut self, name: &WeaponName, builder: &mut FieldAbilityBuilder) -> &'a mut (WeaponRecord, dyn SpecialAbility + 'a) {
        use WeaponName::*;
        match name {
            // sword_4star
            PrototypeRancourR5 => { builder.weapon(&mut self.prototyperancourr5.1); &mut self.prototyperancourr5 },
            TheBlackSwordR5 => { builder.weapon(&mut self.theblackswordr5.1); &mut self.theblackswordr5 },
            BlackcliffLongswordR5 => { builder.weapon(&mut self.blackclifflongswordr5.1); &mut self.blackclifflongswordr5 },
            RoyalLongswordR5 => { builder.weapon(&mut self.royallongswordr5.1); &mut self.royallongswordr5 },
            HarbingerOfDawnR5 => { builder.weapon(&mut self.harbingerofdawnr5.1); &mut self.harbingerofdawnr5 },
            TheFluteR5 => { builder.weapon(&mut self.thefluter5.1); &mut self.thefluter5 },
            LionsRoarR5 => { builder.weapon(&mut self.lionsroarr5.1); &mut self.lionsroarr5 },
            // claymore_4star
            PrototypeArchaicR5 => { builder.weapon(&mut self.prototypearchaicr5.1); &mut self.prototypearchaicr5 },
            WhiteblindR5 => { builder.weapon(&mut self.whiteblindr5.1); &mut self.whiteblindr5 },
            SerpentSpineR5 => { builder.weapon(&mut self.serpentspiner5.1); &mut self.serpentspiner5 },
            BlackcliffSlasherR5 => { builder.weapon(&mut self.blackcliffslasherr5.1); &mut self.blackcliffslasherr5 },
            RoyalGreatswordR5 => { builder.weapon(&mut self.royalgreatswordr5.1); &mut self.royalgreatswordr5 },
            RainslasherR5 => { builder.weapon(&mut self.rainslasherr5.1); &mut self.rainslasherr5 },
            // polearm_4star
            PrototypeStarglitterR5 => { builder.weapon(&mut self.prototypestarglitterr5.1); &mut self.prototypestarglitterr5 },
            CrescentPikeR5 => { builder.weapon(&mut self.crescentpiker5.1); &mut self.crescentpiker5 },
            DeathmatchR5 => { builder.weapon(&mut self.deathmatchr5.1); &mut self.deathmatchr5 },
            BlackcliffPoleR5 => { builder.weapon(&mut self.blackcliffpoler5.1); &mut self.blackcliffpoler5 },
            RoyalSpearR5 => { builder.weapon(&mut self.royalspearr5.1); &mut self.royalspearr5 },
            WhiteTasselR5 => { builder.weapon(&mut self.whitetasselr5.1); &mut self.whitetasselr5 },
            DragonsBaneR5 => { builder.weapon(&mut self.dragonsbaner5.1); &mut self.dragonsbaner5 },
            // bow_4star
            PrototypeCrescentR5 => { builder.weapon(&mut self.prototypecrescentr5.1); &mut self.prototypecrescentr5 },
            CompoundBowR5 => { builder.weapon(&mut self.compoundbowr5.1); &mut self.compoundbowr5 },
            TheViridescentHuntR5 => { builder.weapon(&mut self.theviridescenthuntr5.1); &mut self.theviridescenthuntr5 },
            BlackcliffWarbowR5 => { builder.weapon(&mut self.blackcliffwarbowr5.1); &mut self.blackcliffwarbowr5 },
            RoyalBowR5 => { builder.weapon(&mut self.royalbowr5.1); &mut self.royalbowr5 },
            SlingshotR5 => { builder.weapon(&mut self.slingshotr5.1); &mut self.slingshotr5 },
            RustR5 => { builder.weapon(&mut self.rustr5.1); &mut self.rustr5 },
            TheStringlessR5 => { builder.weapon(&mut self.thestringlessr5.1); &mut self.thestringlessr5 },
            // catalyst_4star
            PrototypeAmberR5 => { builder.weapon(&mut self.prototypeamberr5.1); &mut self.prototypeamberr5 },
            MappaMareR5 => { builder.weapon(&mut self.mappamarer5.1); &mut self.mappamarer5 },
            SolarPearlR5 => { builder.weapon(&mut self.solarpearlr5.1); &mut self.solarpearlr5 },
            BlackcliffAgateR5 => { builder.weapon(&mut self.blackcliffagater5.1); &mut self.blackcliffagater5 },
            RoyalGrimoireR5 => { builder.weapon(&mut self.royalgrimoirer5.1); &mut self.royalgrimoirer5 },
            ThrillingTalesOfDragonSlayersR5 => { builder.weapon(&mut self.thrillingtalesofdragonslayersr5.1); &mut self.thrillingtalesofdragonslayersr5 },
            EyeOfPerceptionR5 => { builder.weapon(&mut self.eyeofperceptionr5.1); &mut self.eyeofperceptionr5 },
            TheWidsithR5 => { builder.weapon(&mut self.thewidsithr5.1); &mut self.thewidsithr5 },
            // favonius_series
            FavoniusGreatswordR5 => { builder.weapon(&mut self.favoniusgreatswordr5.1); &mut self.favoniusgreatswordr5 },
            FavoniusSwordR5 => { builder.weapon(&mut self.favoniusswordr5.1); &mut self.favoniusswordr5 },
            FavoniusLanceR5 => { builder.weapon(&mut self.favoniuslancer5.1); &mut self.favoniuslancer5 },
            FavoniusWarbowR5 => { builder.weapon(&mut self.favoniuswarbowr5.1); &mut self.favoniuswarbowr5 },
            FavoniusCodexR5 => { builder.weapon(&mut self.favoniuscodexr5.1); &mut self.favoniuscodexr5 },
            // sacrificial_series
            SacrificialSwordR5 => { builder.weapon(&mut self.sacrificialswordr5.1); &mut self.sacrificialswordr5 },
            SacrificialGreatswordR5 => { builder.weapon(&mut self.sacrificialgreatswordr5.1); &mut self.sacrificialgreatswordr5 },
            SacrificialBowR5 => { builder.weapon(&mut self.sacrificialbowr5.1); &mut self.sacrificialbowr5 },
            SacrificialFragmentsR5 => { builder.weapon(&mut self.sacrificialfragmentsr5.1); &mut self.sacrificialfragmentsr5 },
            // version_1_5star
            SkywardBlade => { builder.weapon(&mut self.skywardblade.1); &mut self.skywardblade },
            AquilaFavonia => { builder.weapon(&mut self.aquilafavonia.1); &mut self.aquilafavonia },
            SkywardPride => { builder.weapon(&mut self.skywardpride.1); &mut self.skywardpride },
            WolfsGravestone => { builder.weapon(&mut self.wolfsgravestone.1); &mut self.wolfsgravestone },
            SkywardSpine => { builder.weapon(&mut self.skywardspine.1); &mut self.skywardspine },
            PrimordialJadeWingedSpear => { builder.weapon(&mut self.primordialjadewingedspear.1); &mut self.primordialjadewingedspear },
            SkywardHarp => { builder.weapon(&mut self.skywardharp.1); &mut self.skywardharp },
            AmosBow => { builder.weapon(&mut self.amosbow.1); &mut self.amosbow },
            SkywardAtlas => { builder.weapon(&mut self.skywardatlas.1); &mut self.skywardatlas },
            LostPrayerToTheSacredWinds => { builder.weapon(&mut self.lostprayertothesacredwinds.1); &mut self.lostprayertothesacredwinds },
            // version_1_1
            TheUnforged => { builder.weapon(&mut self.theunforged.1); &mut self.theunforged },
            SummitShaper => { builder.weapon(&mut self.summitshaper.1); &mut self.summitshaper },
            VortexVanquisher => { builder.weapon(&mut self.vortexvanquisher.1); &mut self.vortexvanquisher },
            MemoryOfDust => { builder.weapon(&mut self.memoryofdust.1); &mut self.memoryofdust },
            // version_1_2
            FesteringDesire => { builder.weapon(&mut self.festeringdesire.1); &mut self.festeringdesire },
            SnowTombedStarsilver => { builder.weapon(&mut self.snowtombedstarsilver.1); &mut self.snowtombedstarsilver },
            DragonspineSpear => { builder.weapon(&mut self.dragonspinespear.1); &mut self.dragonspinespear },
            Frostbearer => { builder.weapon(&mut self.frostbearer.1); &mut self.frostbearer },
            // version_1_3
            PrimordialJadeCutter => { builder.weapon(&mut self.primordialjadecutter.1); &mut self.primordialjadecutter },
            PrimordialJadeGS => { builder.weapon(&mut self.primordialjadegs.1); &mut self.primordialjadegs },
            PrimordialJadeVista => { builder.weapon(&mut self.primordialjadevista.1); &mut self.primordialjadevista },
            StaffOfHoma => { builder.weapon(&mut self.staffofhoma.1); &mut self.staffofhoma },
            LithicSpear => { builder.weapon(&mut self.lithicspear.1); &mut self.lithicspear },
            LithicBlade => { builder.weapon(&mut self.lithicblade.1); &mut self.lithicblade },
            // version_1_4
            ElegyForTheEnd => { builder.weapon(&mut self.elegyfortheend.1); &mut self.elegyfortheend },
            TheAlleyFlash => { builder.weapon(&mut self.thealleyflash.1); &mut self.thealleyflash },
            AlleyHunter => { builder.weapon(&mut self.alleyhunter.1); &mut self.alleyhunter },
            WineAndSong => { builder.weapon(&mut self.wineandsong.1); &mut self.wineandsong },
            WindblumeOde => { builder.weapon(&mut self.windblumeode.1); &mut self.windblumeode },
            // version_1_5
            SongOfBrokenPines => { builder.weapon(&mut self.songofbrokenpines.1); &mut self.songofbrokenpines },
            // version_1_6
            FreedomSworn => { builder.weapon(&mut self.freedomsworn.1); &mut self.freedomsworn },
            MitternachtsWaltz => { builder.weapon(&mut self.mitternachtswaltz.1); &mut self.mitternachtswaltz },
            DodocoTales => { builder.weapon(&mut self.dodocotales.1); &mut self.dodocotales },
            // version_2_0
            MistsplitterReforged => { builder.weapon(&mut self.mistsplitterreforged.1); &mut self.mistsplitterreforged },
            ThunderingPulse => { builder.weapon(&mut self.thunderingpulse.1); &mut self.thunderingpulse },
            AmenomaKageuchi => { builder.weapon(&mut self.amenomakageuchi.1); &mut self.amenomakageuchi },
            KatsuragikiriNagamasa => { builder.weapon(&mut self.katsuragikirinagamasa.1); &mut self.katsuragikirinagamasa },
            KitainCrossSpear => { builder.weapon(&mut self.kitaincrossspear.1); &mut self.kitaincrossspear },
            Hamayumi => { builder.weapon(&mut self.hamayumi.1); &mut self.hamayumi },
            HakushinRing => { builder.weapon(&mut self.hakushinring.1); &mut self.hakushinring },
            // version_2_1
            GrasscuttersLight => { builder.weapon(&mut self.grasscutterslight.1); &mut self.grasscutterslight },
            FumetsuGekka => { builder.weapon(&mut self.fumetsugekka.1); &mut self.fumetsugekka },
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
    use crate::types::{Vision, FieldEnergy};
    use crate::simulate::simulate;
    use crate::fc::{CharacterData, FieldAbility};
    use crate::action::Attack;
    use crate::testutil::{TestEnvironment};

    use Vision::*;

    // Note that Test disables each weapon's main stats

    #[test]
    fn prototype_rancour() {
        let mut enemy = TestEnvironment::enemy();
        let mut members: Vec<CharacterData> = Vec::new();
        let mut state: Vec<State> = Vec::new();
        let mut abilities: Vec<FieldAbility> = Vec::new();
        let mut atk_queue: Vec<*const Attack> = Vec::new();
        let mut field_energy: Vec<FieldEnergy> = Vec::new();

        let mut wa = PrototypeRancourR5::new();

        let mut env1 = TestEnvironment::new();
        let (data, ability) = env1.weapon(&mut state, State::new(), Pyro, &mut wa);
        members.push(data);
        abilities.push(ability);

        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(0.2, &members, &mut state, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
        }
        // skill na na na
        // TODO why twice 116.0
        let expect = 200.0 + 108.0 + 116.0 * 2.0 + 124.0 + 132.0;
        assert_eq!(total_dmg, 0.5 * expect);
    }

    #[test]
    fn the_flute() {
        let mut enemy = TestEnvironment::enemy();
        let mut members: Vec<CharacterData> = Vec::new();
        let mut state: Vec<State> = Vec::new();
        let mut abilities: Vec<FieldAbility> = Vec::new();
        let mut atk_queue: Vec<*const Attack> = Vec::new();
        let mut field_energy: Vec<FieldEnergy> = Vec::new();

        let mut wa = TheFluteR5::new(FieldCharacterIndex(0));

        let mut env1 = TestEnvironment::new();
        let (data, ability) = env1.no_skill_weapon(&mut state, State::new(), Pyro, &mut wa);
        members.push(data);
        abilities.push(ability);

        let mut total_dmg = 0.0;
        for _ in 0..40 {
            total_dmg += simulate(0.2, &members, &mut state, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
        }
        // 20 na, 2 flute
        let expect = 20.0 * 100.0 + 2.0 * 200.0;
        assert_eq!(total_dmg, 0.5 * expect);
    }

    #[test]
    fn prototype_archaic() {
        let mut enemy = TestEnvironment::enemy();
        let mut members: Vec<CharacterData> = Vec::new();
        let mut state: Vec<State> = Vec::new();
        let mut abilities: Vec<FieldAbility> = Vec::new();
        let mut atk_queue: Vec<*const Attack> = Vec::new();
        let mut field_energy: Vec<FieldEnergy> = Vec::new();

        let mut wa = PrototypeArchaicR5::new(FieldCharacterIndex(0));

        let mut env1 = TestEnvironment::new();
        let (data, ability) = env1.weapon(&mut state, State::new(), Pyro, &mut wa);
        members.push(data);
        abilities.push(ability);

        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(0.2, &members, &mut state, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
        }
        // skill na na na na (prototype_archaic)
        let expect = 200.0 + 5.0 * 100.0 + 480.0;
        assert_eq!(total_dmg, 0.5 * expect);
    }

    #[test]
    fn prototype_archaic_physical() {
        let mut enemy = TestEnvironment::enemy();
        let mut members: Vec<CharacterData> = Vec::new();
        let mut state: Vec<State> = Vec::new();
        let mut abilities: Vec<FieldAbility> = Vec::new();
        let mut atk_queue: Vec<*const Attack> = Vec::new();
        let mut field_energy: Vec<FieldEnergy> = Vec::new();

        let mut wa = PrototypeArchaicR5::new(FieldCharacterIndex(0));

        let mut env1 = TestEnvironment::new();
        let (data, ability) = env1.weapon(&mut state, State::new().physical_dmg(10.0), Pyro, &mut wa);
        members.push(data);
        abilities.push(ability);

        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(0.2, &members, &mut state, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
        }
        // skill na na na na (prototype_archaic)
        let expect = 200.0 + 5.0 * 110.0 + 480.0 * 1.1;
        assert_eq!(total_dmg, 0.5 * expect);
    }

    #[test]
    fn prototype_archaic_na() {
        let mut enemy = TestEnvironment::enemy();
        let mut members: Vec<CharacterData> = Vec::new();
        let mut state: Vec<State> = Vec::new();
        let mut abilities: Vec<FieldAbility> = Vec::new();
        let mut atk_queue: Vec<*const Attack> = Vec::new();
        let mut field_energy: Vec<FieldEnergy> = Vec::new();

        let mut wa = PrototypeArchaicR5::new(FieldCharacterIndex(0));

        let mut env1 = TestEnvironment::new();
        let (data, ability) = env1.weapon(&mut state, State::new().na_dmg(10.0), Pyro, &mut wa);
        members.push(data);
        abilities.push(ability);

        let mut total_dmg = 0.0;
        for _ in 0..10 {
            total_dmg += simulate(0.2, &members, &mut state, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
        }
        // skill na na na (prototype_archaic)
        let expect = 200.0 + 5.0 * 110.0 + 480.0;
        assert_eq!(total_dmg, 0.5 * expect);
    }

    #[test]
    fn prototype_archaic_cd() {
        let mut enemy = TestEnvironment::enemy();
        let mut members: Vec<CharacterData> = Vec::new();
        let mut state: Vec<State> = Vec::new();
        let mut abilities: Vec<FieldAbility> = Vec::new();
        let mut atk_queue: Vec<*const Attack> = Vec::new();
        let mut field_energy: Vec<FieldEnergy> = Vec::new();

        let mut wa = PrototypeArchaicR5::new(FieldCharacterIndex(0));

        let mut env1 = TestEnvironment::new();
        let (data, ability) = env1.no_skill_weapon(&mut state, State::new(), Pyro, &mut wa);
        members.push(data);
        abilities.push(ability);

        let mut total_dmg = 0.0;
        for _ in 0..5 {
            total_dmg += simulate(10.0, &members, &mut state, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
        }

        // na na (prototype_archaic) na na (prototype_archaic)
        let expect = 100.0 + 100.0 + 480.0 + 100.0 + 100.0 + 480.0;
        assert_eq!(total_dmg, 0.5 * expect);
    }

    #[test]
    fn skywardblade() {
        let mut enemy = TestEnvironment::enemy();
        let mut members: Vec<CharacterData> = Vec::new();
        let mut state: Vec<State> = Vec::new();
        let mut abilities: Vec<FieldAbility> = Vec::new();
        let mut atk_queue: Vec<*const Attack> = Vec::new();
        let mut field_energy: Vec<FieldEnergy> = Vec::new();

        let mut wa = SkywardBlade::new(FieldCharacterIndex(0));

        let mut env1 = TestEnvironment::new();
        let (data, ability) = env1.no_skill_weapon(&mut state, State::new(), Pyro, &mut wa);
        members.push(data);
        abilities.push(ability);

        let mut total_dmg = 0.0;
        state[0].energy = members[0].character.energy_cost;
        for _ in 0..10 {
            total_dmg += simulate(0.2, &members, &mut state, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
        }

        // TODO one extra AA?
        // burst na na na na
        let expect = 300.0 + 5.0 * 120.0 + 20.0;
        assert_eq!(total_dmg, 0.5 * expect);
    }

    #[test]
    fn songofbrokenpines() {
        let mut enemy = TestEnvironment::enemy();
        let mut members: Vec<CharacterData> = Vec::new();
        let mut state: Vec<State> = Vec::new();
        let mut abilities: Vec<FieldAbility> = Vec::new();
        let mut atk_queue: Vec<*const Attack> = Vec::new();
        let mut field_energy: Vec<FieldEnergy> = Vec::new();

        let mut wa = SongOfBrokenPines::new();

        let mut env1 = TestEnvironment::new();
        let (data, ability) = env1.no_skill_weapon(&mut state, State::new(), Pyro, &mut wa);
        members.push(data);
        abilities.push(ability);

        let mut total_dmg = 0.0;
        for _ in 0..20 {
            total_dmg += simulate(1.0, &members, &mut state, &mut abilities, &mut atk_queue, &mut field_energy, &mut enemy);
        }

        let expect = 6.0 * 100.0 + 13.0 * 120.0;
        assert_eq!(total_dmg, 0.5 * expect);
    }
}
