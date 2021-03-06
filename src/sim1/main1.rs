use std::error::Error;
use std::env;
use std::io;
use std::process;
use std::cmp::Ordering;
use std::mem::MaybeUninit;
use std::rc::Rc;
use std::cell::RefCell;

use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::time::{Duration};

use crate::sim1::state::State;
use crate::sim1::artifact::{Artifact};
use crate::sim1::action::{Attack, ICDTimers};
use crate::sim1::fc::{FieldCharacterIndex, FieldAbility, CharacterAbility, CharacterRecord, WeaponRecord, CharacterData, Enemy};
use crate::sim1::types::{Vision, Preference, FieldEnergy};
use crate::sim1::permutools::Permutation3;
use crate::sim1::cli::Args;

// #[derive(Debug, Eq)]
// struct Recorder {
//     head: Vec<&'static str>,
//     data: Vec<i32>,
// }

// impl Ord for Recorder {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.data.last().unwrap().cmp(&other.data.last().unwrap())
//     }
// }

// impl PartialOrd for Recorder {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl PartialEq for Recorder {
//     fn eq(&self, other: &Self) -> bool {
//         self.data.last().unwrap() == other.data.last().unwrap()
//     }
// }

// trait NewRecorder<T> {
//     fn new(item: T) -> Recorder;
// }

// impl NewRecorder<Vec<&'static str>> for Recorder {
//     fn new(item: Vec<&'static str>) -> Self {
//         Self {
//             head: item,
//             data: Vec::new()
//         }
//     }
// }

// impl NewRecorder<&CharacterData<'_>> for Recorder {
//     fn new(item: &CharacterData) -> Self {
//         Self {
//             head: vec![item.character.name, item.weapon.name, item.artifact.name],
//             data: Vec::new()
//         }
//     }
// }

// impl NewRecorder<&(&CharacterRecord, &WeaponRecord, &Artifact)> for Recorder {
//     fn new(item: &(&CharacterRecord, &WeaponRecord, &Artifact)) -> Self {
//         let (cr, wr, ar) = item;
//         Self {
//             head: vec![cr.name, wr.name, ar.name],
//             data: Vec::new()
//         }
//     }
// }

// impl Recorder {
//     fn record(&mut self, time: f32, value: f32) -> () {
//         if self.data.len() == time.floor() as usize {
//             self.data.push(value.floor() as i32);
//         }
//     }

//     fn make_row(&mut self) -> Vec<String> {
//         let mut result: Vec<String> = Vec::with_capacity(self.head.len() + self.data.len());
//         // result.append(&mut self.head);
//         for x in &self.head {
//             result.push(x.to_string());
//         }
//         for x in &self.data {
//             result.push(x.to_string());
//         }
//         result
//     }
// }

// fn near_eq(a: f32, b: f32) -> bool {
//     let diff = (a - b).abs();
//     diff <= 0.001
// }

// fn main_loop(members: &mut [CharacterData], abilities: &mut [FieldAbility], args: &Args) -> Recorder {
//     let mut enemy = Enemy::hilichurl();
//     let mut atk_queue: Vec<*const Attack> = Vec::new();
//     let mut field_energy: Vec<FieldEnergy> = Vec::new();
//     // setup for Simulator
//     let mut head: Vec<&str> = Vec::new();
//     for m in members.iter() {
//         head.push(m.character.name);
//         head.push(m.weapon.name);
//         head.push(m.artifact.name);
//     }
//     let mut rc = Recorder::new(head);
//     let mut current_time = 0.0;
//     let mut total_dmg = 0.0;

//     while current_time < args.simulation_time {
//         // setup for artifacts and supporters
//         for data in members.iter_mut() {
//             data.state.clear();
//             data.init();
//             data.state.flat_atk += 311.0;
//             if data.idx.0 > 0 {
//                 // data.state.na_dmg -= 100.0;
//                 // data.state.ca_dmg -= 100.0;
//                 data.state.atk_spd -= 90.0;
//             }
//         }
//         if current_time == 0.0 {
//             if args.start_energy < 0 {
//                 for m in members.iter_mut() {
//                     m.state.energy = m.character.energy_cost;
//                 }
//             } else {
//                 let e = args.start_energy as f32;
//                 for m in members.iter_mut() {
//                     m.state.energy = e;
//                 }
//             }
//         }
//         if near_eq(current_time, 5.0) || near_eq(current_time, 15.0) {
//         // if near_eq(current_time, 5.0) {
//             for m in members.iter_mut() {
//                 m.state.energy += 10.0 * m.state.ER();
//             }
//         }
//         total_dmg += sim1::simulate::simulate(args.unit_time, members, abilities, &mut atk_queue, &mut field_energy, &mut enemy);
//         current_time += args.unit_time;
//         rc.record(current_time, total_dmg);
//     }
//     rc
// }

// fn combination_filter_attacker(cr: &CharacterRecord, wr: &WeaponRecord, ar: &Artifact, args: &Args) -> bool {
//     // TODO
//     let physical_infusion: Vec<&'static str> = vec!["Razor", "Eula", ];

//     if cr.version > args.character_version ||
//        wr.version > args.weapon_version ||
//        ar.version > args.artifact_version {
//         return false;
//     }

//     // check weapon
//     if cr.weapon != wr.type_ {
//         return false;
//     }

//     // check artifact
//     let physical_attack = physical_infusion.contains(&cr.name);
//     let mut result = if ar.preference.len() == 0 {
//         true
//     } else {
//         false
//     };
//     for p in ar.preference.iter() {
//         if p == &cr.vision
//         || (physical_attack && p == &Preference::Physical)
//         || p == &cr.weapon
//         || p == &Preference::Attacker {
//             result = true;
//             break;
//         }
//     }

//     result
// }

// fn combination_filter_supporter(cr: &CharacterRecord, wr: &WeaponRecord, ar: &Artifact, args: &Args) -> bool {
//     if cr.version > args.character_version ||
//        wr.version > args.weapon_version ||
//        ar.version > args.artifact_version {
//         return false;
//     }

//     // check weapon
//     if cr.weapon != wr.type_ {
//         return false;
//     }

//     // check artifact
//     let mut result = if ar.preference.len() == 0 {
//         true
//     } else {
//         false
//     };
//     for p in ar.preference {
//         if *p == cr.vision
//         || *p == cr.weapon
//         || *p == Preference::Supporter {
//             result = true;
//             break;
//         }
//     }

//     result
// }

// #[cfg(test)]
// mod tests {
//     use crate::sim1::fc::{SpecialAbility};
//     use super::*;

//     #[test]
//     fn filter_1() {
//         let c = characters::pyro::Diluc::record();
//         let w = weapons::claymore_4star::RainslasherR5::record();
//         let a = artifact::ViridescentVenerer::record();
//         let args = Args::default();
//         assert!(!combination_filter_attacker(&c, &w, &a, &args));
//         assert!(!combination_filter_supporter(&c, &w, &a, &args));
//     }

//     #[test]
//     fn filter_2() {
//         let c = characters::pyro::Diluc::record();
//         let w = weapons::claymore_4star::RainslasherR5::record();
//         let a = artifact::GladiatorsFinale::record();
//         let args = Args::default();
//         assert!(combination_filter_attacker(&c, &w, &a, &args));
//         assert!(combination_filter_supporter(&c, &w, &a, &args));
//     }

//     #[test]
//     fn filter_3() {
//         let c = characters::electro::Razor::record();
//         let w = weapons::claymore_4star::RainslasherR5::record();
//         let a = artifact::PaleFlame::record();
//         let args = Args::default();
//         assert!(combination_filter_attacker(&c, &w, &a, &args));
//         // assert!(combination_filter_supporter(&c, &w, &a, &args));
//     }

//     #[test]
//     fn filter_4() {
//         let c = characters::electro::Razor::record();
//         let w = weapons::claymore_4star::RainslasherR5::record();
//         let a = artifact::ThunderingFury::record();
//         let args = Args::default();
//         assert!(combination_filter_attacker(&c, &w, &a, &args));
//         assert!(combination_filter_supporter(&c, &w, &a, &args));
//     }

//     #[test]
//     fn filter_5() {
//         let c = characters::hydro::Xingqiu::record();
//         let w = weapons::sword_4star::PrototypeRancourR5::record();
//         let a = artifact::BlizzardStrayer::record();
//         let args = Args::default();
//         assert!(combination_filter_attacker(&c, &w, &a, &args));
//         assert!(combination_filter_supporter(&c, &w, &a, &args));
//     }

//     #[test]
//     fn filter_6() {
//         let c = characters::cryo::Kaeya::record();
//         let w = weapons::sword_4star::PrototypeRancourR5::record();
//         let a = artifact::BlizzardStrayer::record();
//         let args = Args::default();
//         assert!(combination_filter_attacker(&c, &w, &a, &args));
//         assert!(combination_filter_supporter(&c, &w, &a, &args));
//     }
// }

// fn permu6(tx: Sender<Vec<Recorder>>, start: usize, end: usize, args: &Args) -> () {
//     let idx0 = FieldCharacterIndex(0);
//     let idx1 = FieldCharacterIndex(1);
//     let mut timers0 = ICDTimers::new();
//     let mut timers1 = ICDTimers::new();
//     let input_characters: Vec<(CharacterRecord, Box<dyn CharacterAbility>)> = characters::all(idx0, &timers0).drain(start..end).collect();
//     // TODO
//     let physical_infusion: Vec<&'static str> = vec!["Razor", "Eula", ];

//     let mut member1 = Permutation3::new(
//         input_characters,
//         weapons::all(idx0, &timers0),
//         artifact::all(idx0),
//     );
//     let mut member2 = Permutation3::new(
//         characters::all(idx1, &timers1),
//         weapons::all(idx1, &timers1),
//         artifact::all(idx1),
//     );
//     for ((cr1, mut ca1), (wr1, mut wa1), (mut ar1, mut aa1)) in member1.iter() {
//         if !combination_filter_attacker(&cr1, &wr1, &ar1, args) {
//             member1.back(((cr1, ca1), (wr1, wa1), (ar1, aa1)));
//             continue;
//         }
//         if physical_infusion.contains(&cr1.name) {
//             ar1.infuse_goblet(&Vision::Physical);
//         } else {
//             ar1.infuse_goblet(&cr1.vision);
//         }

//         let mut items: Vec<Recorder> = Vec::new();
//         for ((cr2, mut ca2), (wr2, mut wa2), (mut ar2, mut aa2)) in member2.iter() {
//             let mut members: Vec<CharacterData> = Vec::with_capacity(2);
//             let mut abilities: Vec<FieldAbility> = Vec::with_capacity(2);
//             // identical characters cannot be on the same field.
//             if cr1.name == cr2.name ||
//                !combination_filter_supporter(&cr2, &wr2, &ar2, args) {
//                 member2.back(((cr2, ca2), (wr2, wa2), (ar2, aa2)));
//                 continue;
//             }
//             if physical_infusion.contains(&cr2.name) {
//                 ar2.infuse_goblet(&Vision::Physical);
//             } else {
//                 ar2.infuse_goblet(&cr2.vision);
//             }
//             members.push(CharacterData::new(idx0, &cr1, &wr1, &ar1));
//             members.push(CharacterData::new(idx1, &cr2, &wr2, &ar2));
//             abilities.push(FieldAbility {
//                 timers: &mut timers0,
//                 character: &mut *ca1,
//                 weapon: &mut *wa1,
//                 artifact: &mut *aa1,
//             });
//             abilities.push(FieldAbility {
//                 timers: &mut timers1,
//                 character: &mut *ca2,
//                 weapon: &mut *wa2,
//                 artifact: &mut *aa2,
//             });
//             let rc = main_loop(&mut members, &mut abilities, args);
//             items.push(rc);

//             // destruct objects
//             for a in abilities.iter_mut() {
//                 a.reset();
//             }
//             timers0.reset();
//             timers1.reset();
//             ar1.dry_goblet();
//             ar2.dry_goblet();
//             member2.back(((cr2, ca2), (wr2, wa2), (ar2, aa2)));
//         }
//         member2.reset();
//         member1.back(((cr1, ca1), (wr1, wa1), (ar1, aa1)));
//         tx.send(items).unwrap();
//     }
// }

// fn permu3(tx: Sender<Vec<Recorder>>, start: usize, end: usize, args: &Args) -> () {
//     let idx = FieldCharacterIndex(0);
//     let mut timers0 = ICDTimers::new();
//     let input_characters: Vec<(CharacterRecord, Box<dyn CharacterAbility>)> = characters::all(idx, &timers0).drain(start..end).collect();
//     // TODO
//     let physical_infusion: Vec<&'static str> = vec!["Razor", "Eula", ];
//     let mut member1 = Permutation3::new(
//         input_characters,
//         weapons::all(idx, &timers0),
//         artifact::all(idx),
//     );
//     let mut items: Vec<Recorder> = Vec::new();
//     for ((cr1, mut ca1), (wr1, mut wa1), (mut ar1, mut aa1)) in member1.iter() {
//         let mut members: Vec<CharacterData> = Vec::with_capacity(1);
//         let mut abilities: Vec<FieldAbility> = Vec::with_capacity(1);
//         if !combination_filter_attacker(&cr1, &wr1, &ar1, args) {
//             member1.back(((cr1, ca1), (wr1, wa1), (ar1, aa1)));
//             continue;
//         }
//         if physical_infusion.contains(&cr1.name) {
//             ar1.infuse_goblet(&Vision::Physical);
//         } else {
//             ar1.infuse_goblet(&cr1.vision);
//         }
//         members.push(CharacterData::new(idx, &cr1, &wr1, &ar1));
//         abilities.push(FieldAbility {
//             timers: &mut timers0,
//             character: &mut *ca1,
//             weapon: &mut *wa1,
//             artifact: &mut *aa1,
//         });
//         let rc = main_loop(&mut members, &mut abilities, args);
//         items.push(rc);

//         // destruct objects
//         for a in abilities.iter_mut() {
//             a.reset();
//         }
//         timers0.reset();
//         ar1.dry_goblet();
//         member1.back(((cr1, ca1), (wr1, wa1), (ar1, aa1)));
//     }
//     tx.send(items).unwrap();
// }

// // fn debugging(args: &Args, debug_args: Vec<String>) -> () {
// //     let idx = FieldCharacterIndex(0);
// //     let mut timers = ICDTimers::new();
// //     let mut builder = FieldAbilityBuilder::new();
// //     let mut characters = AllCharacters::new(idx, &timers);
// //     let mut weapons = AllWeapons::new(idx, &timers);
// //     let mut artifacts = AllArtifacts::new(idx);
// //     let mut character_name = MaybeUninit::<CharacterName>::uninit();
// //     let mut weapon_name = MaybeUninit::<WeaponName>::uninit();
// //     let mut artifact_name = MaybeUninit::<ArtifactName>::uninit();
// //     let mut members: Vec<CharacterData> = Vec::new();
// //     let mut abilities: Vec<FieldAbility> = Vec::new();
// //     for i in 0..3 {
// //         match i {
// //             0 => unsafe { character_name.as_mut_ptr().write(CharacterName::from(debug_args[0].as_str())) },
// //             1 => unsafe { weapon_name.as_mut_ptr().write(WeaponName::from(debug_args[1].as_str())) },
// //             2 => unsafe { artifact_name.as_mut_ptr().write(ArtifactName::from(debug_args[2].as_str())) },
// //             _ => (),
// //         }
// //     };
// //     let (cr1, ca1) = characters.find(unsafe { &character_name.assume_init() }, &mut builder);
// //     let (wr1, wa1) = weapons.find(unsafe { &weapon_name.assume_init() }, &mut builder);
// //     let (ar1, aa1) = artifacts.find(unsafe { &artifact_name.assume_init() }, &mut builder);
// //     let physical_infusion: Vec<&'static str> = vec!["Razor", "Eula", ];
// //     if physical_infusion.contains(&cr1.name) {
// //         ar1.infuse_goblet(&Vision::Physical);
// //     } else {
// //         ar1.infuse_goblet(&cr1.vision);
// //     }
// //     members.push(CharacterData::new(idx, &cr1, &wr1, &ar1));
// //     abilities.push(builder.build(&mut timers));
// //     println!("debugging: {:?} {:?} {:?}", cr1.name, wr1.name, ar1.name);
// //     main_loop(&mut members, &mut abilities, args);
// // }

// fn start_and_wait() -> Result<(), Box<dyn Error + 'static>> {
//     let mut debug_args: Vec<String> = Vec::new();
//     let args = Args::parse(&mut env::args(), &mut debug_args)?;
//     if debug_args.len() > 0 {
//         return Ok(());
//         // return Ok(debugging(&args, debug_args));
//     }
//     let num_cpu = 4;
//     let character_size = characters::N_CHARACTERS;
//     let chunk_size = character_size / num_cpu + 1;
//     let (tx, rx) = mpsc::channel();
//     if num_cpu == 1 {
//         permu3(tx, 0, character_size, &args);
//     } else {
//         for i in 0..num_cpu {
//             let start = i * chunk_size;
//             let mut end = (i + 1) * chunk_size;
//             if start >= character_size {
//                 break;
//             } else if end > character_size {
//                 end = character_size;
//             }
//             let txn = tx.clone();
//             match args.n_members {
//                 1 => thread::spawn(move || permu3(txn, start, end, &args) ),
//                 2 => thread::spawn(move || permu6(txn, start, end, &args) ),
//                 _ => unimplemented!(),
//             };
//         }
//         drop(tx);
//     }
//     let mut wtr = csv::Writer::from_writer(io::stdout());
//     for mut received in rx {
//         if args.truncate && args.n_members > 1 {
//             received.sort();
//             received.drain(..received.len() / 2);
//         }
//         for rc in &mut received {
//             wtr.write_record(&rc.make_row())?;
//         }
//         wtr.flush()?;
//     }
//     Ok(())
// }

fn main1() {
    // thread::sleep(Duration::from_secs(10));
    // if let Err(err) = start_and_wait() {
    //     println!("{:?}", err);
    //     process::exit(1);
    // };
}
